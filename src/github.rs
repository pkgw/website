// Copyright Peter Williams <peter@newton.cx>
// Licensed under the MIT License.

//! GitHub API invocations.

use anyhow::anyhow;
use json::{object, JsonValue};

use crate::{env::require_var, errors::Result};

pub struct GitHubClient {
    token: String,
}

const API_BASE: &str = "https://api.github.com/repos/";

impl GitHubClient {
    pub fn new() -> Result<Self> {
        let token = require_var("GITHUB_TOKEN")?;

        Ok(GitHubClient { token })
    }

    pub fn make_blocking_client(&self) -> Result<reqwest::blocking::Client> {
        use reqwest::header;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("token {}", self.token))?,
        );
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str("deploytool")?,
        );

        Ok(reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?)
    }

    /// Create a new issue comment
    pub fn create_comment(
        &self,
        slug: &str,
        issue_num: usize,
        body: String,
        client: &mut reqwest::blocking::Client,
    ) -> Result<JsonValue> {
        let url = format!("{}{}/issues/{}/comments", API_BASE, slug, issue_num);

        let payload = object! {
            "body" => body,
        };

        let resp = client
            .post(url)
            .header(reqwest::header::ACCEPT, "application/vnd.github+json")
            .header(
                reqwest::header::HeaderName::from_static("x-github-api-version"),
                "2022-11-28",
            )
            .body(json::stringify(payload))
            .send()?;
        let status = resp.status();
        let parsed = json::parse(&resp.text()?)?;

        if status.is_success() {
            Ok(parsed)
        } else {
            Err(anyhow!("failed to create GitHub comment: {}", parsed))
        }
    }
}
