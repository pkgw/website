// Copyright Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! State of the backing version control repository.

use anyhow::anyhow;
use log::info;
use std::path::{Path, PathBuf};
use thiserror::Error as ThisError;

use crate::{atry, errors::Result};

/// Opaque type representing a commit in the repository.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommitId(git2::Oid);

impl std::fmt::Display for CommitId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// An empty error returned when the backing repository is "bare", without a
/// working directory. We cannot operate on such repositories.
#[derive(Debug, ThisError)]
#[error("cannot operate on a bare repository")]
pub struct BareRepositoryError;

/// An error returned when the backing repository is "dirty", i.e. there are
/// modified files, and this has situation has been deemed unacceptable. The
/// inner value is one of the culprit paths.
#[derive(Debug, ThisError)]
pub struct DirtyRepositoryError(pub RepoPathBuf);

/// An error returned when some metadata references a commit in the repository,
/// and that reference is bogus. The inner value is the text of the reference.
#[derive(Debug, ThisError)]
#[error("commit reference `{0}` is invalid or refers to a nonexistent commit")]
pub struct InvalidHistoryReferenceError(pub String);

impl std::fmt::Display for DirtyRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "the file backing repository is dirty: file {} has been modified",
            self.0.escaped()
        )
    }
}

/// Information about the backing version control repository.
pub struct Repository {
    /// The underlying `git2` repository object.
    repo: git2::Repository,
}

impl Repository {
    /// Open the repository using standard environmental cues.
    pub fn open_from_env() -> Result<Repository> {
        let repo = git2::Repository::open_from_env()?;

        if repo.is_bare() {
            return Err(BareRepositoryError.into());
        }

        Ok(Repository { repo })
    }

    /// Get the name of the currently active branch, if there is one.
    pub fn current_branch_name(&self) -> Result<Option<String>> {
        let head_ref = self.repo.head()?;

        Ok(if !head_ref.is_branch() {
            None
        } else {
            Some(
                head_ref
                    .shorthand()
                    .ok_or_else(|| anyhow!("current branch name not Unicode"))?
                    .to_owned(),
            )
        })
    }

    /// Resolve a `RepoPath` repository path to a filesystem path in the working
    /// directory.
    pub fn resolve_workdir(&self, p: &RepoPath) -> PathBuf {
        let mut fullpath = self.repo.workdir().unwrap().to_owned();
        fullpath.push(p.as_path());
        fullpath
    }

    /// Scan the paths in the repository index.
    pub fn scan_paths<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(&RepoPath) -> Result<()>,
    {
        // We have to use a callback here since the IndexEntries iter holds a
        // ref to the index, which therefore has to be immovable (pinned) during
        // the iteration process.
        let index = self.repo.index()?;

        for entry in index.iter() {
            let p = RepoPath::new(&entry.path);
            atry!(
                f(p);
                ["encountered a problem while scanning repository entry `{}`", p.escaped()]
            );
        }

        Ok(())
    }

    /// Check if the working tree is clean. Returns None if there are no
    /// modifications and Some(escaped_path) if there are any. (The escaped_path
    /// will be the first one encountered in the check, an essentially arbitrary
    /// selection.) Modifications to any of the paths matched by `ok_matchers`
    /// are allowed.
    pub fn check_if_dirty(&self) -> Result<Option<RepoPathBuf>> {
        // Default options are what we want.
        let mut opts = git2::StatusOptions::new();

        for entry in self.repo.statuses(Some(&mut opts))?.iter() {
            // Is this correct / sufficient?
            if entry.status() != git2::Status::CURRENT {
                let repo_path = RepoPath::new(entry.path_bytes());
                // skipping https://github.com/pkgw/cranko/issues/41
                return Ok(Some(repo_path.to_owned()));
            }
        }

        Ok(None)
    }

    /// Get the binary content of the file at the specified path, at the time of
    /// the specified commit. If the path did not exist, `Ok(None)` is returned.
    pub fn get_file_at_commit(&self, cid: &CommitId, path: &RepoPath) -> Result<Option<Vec<u8>>> {
        let commit = self.repo.find_commit(cid.0)?;
        let tree = commit.tree()?;
        let entry = match tree.get_path(path.as_path()) {
            Ok(e) => e,
            Err(e) => {
                return if e.code() == git2::ErrorCode::NotFound {
                    Ok(None)
                } else {
                    Err(e.into())
                };
            }
        };
        let object = entry.to_object(&self.repo)?;
        let blob = object.as_blob().ok_or_else(|| {
            anyhow!(
                "path `{}` should correspond to a Git blob but does not",
                path.escaped(),
            )
        })?;

        Ok(Some(blob.content().to_owned()))
    }

    fn get_signature(&self) -> Result<git2::Signature> {
        Ok(git2::Signature::now("deploytool", "deploytool@devnull")?)
    }

    pub fn get_deploy_tree(&self) -> Result<git2::Tree<'_>> {
        // XXXX origin/deploy
        let short_name = "deploy";

        Ok(self
            .repo
            .resolve_reference_from_short_name(short_name)?
            .peel_to_commit()?
            .tree()?)
    }

    pub fn make_deploy_commit(&mut self) -> Result<()> {
        // Gather useful info.

        let head_ref = self.repo.head()?;
        let head_commit = head_ref.peel_to_commit()?;
        let sig = self.get_signature()?;
        let local_ref_name = "refs/heads/deploy";
        let message = "Deployment metadata commit";

        // Turn the current index into a Tree.

        let tree_oid = {
            let mut index = self.repo.index()?;
            index.write_tree()?
        };
        let tree = self.repo.find_tree(tree_oid)?;

        // Create the merged deploy commit and save it under the
        // local_ref_name.

        let prev_deploy_commit = self
            .repo
            .resolve_reference_from_short_name(local_ref_name)?
            .peel_to_commit()?;

        self.repo.reference(
            &local_ref_name,
            prev_deploy_commit.id(),
            true,
            "update deploy",
        )?;

        let commit_id = self.repo.commit(
            Some(&local_ref_name), // update_ref
            &sig,                  // author
            &sig,                  // committer
            &message,
            &tree,
            &[&prev_deploy_commit, &head_commit], // parents
        )?;

        // Switch the working directory to be the checkout of our new merge
        // commit. By construction, nothing on the filesystem should actually
        // change.

        info!("switching HEAD to `{}`", local_ref_name);
        self.repo.set_head(&local_ref_name)?;
        self.repo.reset(
            self.repo.find_commit(commit_id)?.as_object(),
            git2::ResetType::Mixed,
            None,
        )?;

        // Phew, all done!

        Ok(())
    }
}

// Below we have helpers for trying to deal with git's paths properly, on the
// off-chance that they contain invalid UTF-8 and the like.

/// A borrowed reference to a pathname as understood by the backing repository.
///
/// In git, such a path is a byte array. The directory separator is always "/".
/// The bytes are often convertible to UTF-8, but not always. (These are the
/// same semantics as Unix paths.)
#[derive(Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct RepoPath([u8]);

impl std::convert::AsRef<RepoPath> for [u8] {
    fn as_ref(&self) -> &RepoPath {
        unsafe { &*(self as *const [_] as *const RepoPath) }
    }
}

impl std::convert::AsRef<[u8]> for RepoPath {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self.0.as_ref() as *const [u8]) }
    }
}

impl RepoPath {
    fn new(p: &[u8]) -> &Self {
        p.as_ref()
    }

    /// Split a path into a directory name and a file basename.
    ///
    /// Returns `(dirname, basename)`. The dirname will be empty if the path
    /// contains no separator. Otherwise, it will end with the path separator.
    /// It is always true that `self = concat(dirname, basename)`.
    pub fn split_basename(&self) -> (&RepoPath, &RepoPath) {
        // Have to index the dirname manually since split() and friends don't
        // include the separating items, which we want.
        let basename = self.0.rsplit(|c| *c == b'/').next().unwrap();
        let ndir = self.0.len() - basename.len();
        return (self.0[..ndir].as_ref(), basename.as_ref());
    }

    /// Return this path with a trailing directory separator removed, if one is
    /// present.
    pub fn pop_sep(&self) -> &RepoPath {
        let n = self.0.len();

        if n == 0 || self.0[n - 1] != b'/' {
            self
        } else {
            self.0[..n - 1].as_ref()
        }
    }

    /// Get the length of the path, in bytes
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Convert the repository path into an OS path.
    pub fn as_path(&self) -> &Path {
        bytes2path(&self.0)
    }

    /// Convert this borrowed reference into an owned copy.
    pub fn to_owned(&self) -> RepoPathBuf {
        RepoPathBuf::new(&self.0[..])
    }

    /// Compute a user-displayable escaped version of this path.
    pub fn escaped(&self) -> String {
        escape_pathlike(&self.0)
    }

    /// Return true if this path starts with the argument.
    pub fn starts_with<P: AsRef<[u8]>>(&self, other: P) -> bool {
        let other = other.as_ref();
        let sn = self.len();
        let on = other.len();

        if sn < on {
            false
        } else {
            &self.0[..on] == other
        }
    }

    /// Return true if this path ends with the argument.
    pub fn ends_with<P: AsRef<[u8]>>(&self, other: P) -> bool {
        let other = other.as_ref();
        let sn = self.len();
        let on = other.len();

        if sn < on {
            false
        } else {
            &self.0[(sn - on)..] == other
        }
    }
}

impl git2::IntoCString for &RepoPath {
    fn into_c_string(self) -> std::result::Result<std::ffi::CString, git2::Error> {
        self.0.into_c_string()
    }
}

// Copied from git2-rs src/util.rs
#[cfg(unix)]
fn bytes2path(b: &[u8]) -> &Path {
    use std::{ffi::OsStr, os::unix::prelude::*};
    Path::new(OsStr::from_bytes(b))
}
#[cfg(windows)]
fn bytes2path(b: &[u8]) -> &Path {
    use std::str;
    Path::new(str::from_utf8(b).unwrap())
}

/// An owned reference to a pathname as understood by the backing repository.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct RepoPathBuf(Vec<u8>);

impl std::convert::AsRef<RepoPath> for RepoPathBuf {
    fn as_ref(&self) -> &RepoPath {
        RepoPath::new(&self.0[..])
    }
}

impl std::convert::AsRef<[u8]> for RepoPathBuf {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl RepoPathBuf {
    pub fn new(b: &[u8]) -> Self {
        RepoPathBuf(b.to_vec())
    }

    /// Create a RepoPathBuf from a Path-like. It is assumed that the path is
    /// relative to the repository working directory root and doesn't have any
    /// funny business like ".." in it.
    #[cfg(unix)]
    #[allow(clippy::unnecessary_wraps)]
    fn from_path<P: AsRef<Path>>(p: P) -> Result<Self> {
        use std::os::unix::ffi::OsStrExt;
        Ok(Self::new(p.as_ref().as_os_str().as_bytes()))
    }

    /// Create a RepoPathBuf from a Path-like. It is assumed that the path is
    /// relative to the repository working directory root and doesn't have any
    /// funny business like ".." in it.
    #[cfg(windows)]
    fn from_path<P: AsRef<Path>>(p: P) -> Result<Self> {
        let mut first = true;
        let mut b = Vec::new();

        for cmpt in p.as_ref().components() {
            if first {
                first = false;
            } else {
                b.push(b'/');
            }

            if let std::path::Component::Normal(c) = cmpt {
                b.extend(c.to_str().unwrap().as_bytes());
            } else {
                bail!(
                    "path with unexpected components: `{}`",
                    p.as_ref().display()
                );
            }
        }

        Ok(RepoPathBuf(b))
    }

    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
    }

    pub fn push<C: AsRef<[u8]>>(&mut self, component: C) {
        let n = self.0.len();

        if n > 0 && self.0[n - 1] != b'/' {
            self.0.push(b'/');
        }

        self.0.extend(component.as_ref());
    }
}

impl std::ops::Deref for RepoPathBuf {
    type Target = RepoPath;

    fn deref(&self) -> &RepoPath {
        RepoPath::new(&self.0[..])
    }
}

/// Convert an arbitrary byte slice to something printable.
///
/// If the bytes can be interpreted as UTF-8, their Unicode stringification will
/// be returned. Otherwise, bytes that aren't printable ASCII will be
/// backslash-escaped, and the whole string will be wrapped in double quotes.
///
/// **Note**: we should probably only do a direct conversion if it's printable
/// ASCII without whitespaces, etc. To be refined.
pub fn escape_pathlike(b: &[u8]) -> String {
    if let Ok(s) = std::str::from_utf8(b) {
        s.to_owned()
    } else {
        let mut buf = vec![b'\"'];
        buf.extend(b.iter().flat_map(|c| std::ascii::escape_default(*c)));
        buf.push(b'\"');
        String::from_utf8(buf).unwrap()
    }
}
