use anyhow::Result;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine as _;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GHError {
    #[error("API rate limit exceeded")]
    RateLimitExceeded,
    #[error("Repository not found: {0}")]
    RepoNotFound(String),
    #[error("Authentication required")]
    AuthRequired,
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Parser)]
struct Cli {
    /// Repository (format: owner/name)
    repo: String,
    /// GitHub token
    #[arg(long)]
    token: Option<String>,
    /// Output format (json or yaml)
    #[arg(long, default_value = "json")]
    format: String,
    /// Additional files to fetch
    #[arg(long)]
    extra_files: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RepoSummary {
    repo: RepoInfo,
    stats: RepoStats,
    languages: HashMap<String, u32>,
    content: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RepoInfo {
    name: String,
    description: Option<String>,
    html_url: String,
    stargazers_count: u32,
    forks_count: u32,
    created_at: String,
    updated_at: String,
    default_branch: String,
    #[serde(default)]
    license: Option<License>,
    #[serde(default)]
    topics: Vec<String>,
    #[serde(default)]
    visibility: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RepoStats {
    open_issues_count: u32,
    watchers_count: u32,
    network_count: u32,
    size: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct License {
    key: String,
    name: String,
    spdx_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RateLimit {
    remaining: u32,
    reset: u64,
}

#[derive(Debug, Deserialize)]
struct ContentResponse {
    #[serde(rename = "type")]
    content_type: String,
    content: Option<String>,
}

struct GitHub {
    client: Client,
    progress: ProgressBar,
}

impl GitHub {
    fn new(token: Option<String>) -> Result<Self> {
        let client = build_client(&token)?;
        let progress = ProgressBar::new(0);
        progress.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );

        Ok(Self { client, progress })
    }

    async fn check_rate_limit(&self) -> Result<RateLimit> {
        let resp = self
            .client
            .get("https://api.github.com/rate_limit")
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => {
                let data: serde_json::Value = resp.json().await?;
                Ok(RateLimit {
                    remaining: data["rate"]["remaining"].as_u64().unwrap_or(0) as u32,
                    reset: data["rate"]["reset"].as_u64().unwrap_or(0),
                })
            }
            StatusCode::UNAUTHORIZED => Err(GHError::AuthRequired.into()),
            _ => Err(GHError::NetworkError(resp.error_for_status().unwrap_err()).into()),
        }
    }

    async fn get_repo_data(&self, repo_path: &str) -> Result<RepoSummary> {
        self.progress.set_message("Fetching repository info...");

        // Check rate limit first
        let rate_limit = self.check_rate_limit().await?;
        if rate_limit.remaining == 0 {
            let reset_time = chrono::DateTime::from_timestamp(rate_limit.reset as i64, 0)
                .unwrap()
                .format("%H:%M:%S")
                .to_string();
            return Err(anyhow::anyhow!(
                "API rate limit exceeded. Resets at {}",
                reset_time
            ));
        }

        let base_url = format!("https://api.github.com/repos/{}", repo_path);

        // Fetch basic repo info
        let resp = self.client.get(&base_url).send().await?;
        match resp.status() {
            StatusCode::NOT_FOUND => {
                return Err(GHError::RepoNotFound(repo_path.to_string()).into())
            }
            StatusCode::OK => (),
            _ => return Err(GHError::NetworkError(resp.error_for_status().unwrap_err()).into()),
        }

        let repo_data: serde_json::Value = resp.json().await?;
        let repo: RepoInfo = serde_json::from_value(repo_data.clone())?;

        // Parse repo stats
        let stats = RepoStats {
            open_issues_count: repo_data["open_issues_count"].as_u64().unwrap_or(0) as u32,
            watchers_count: repo_data["watchers_count"].as_u64().unwrap_or(0) as u32,
            network_count: repo_data["network_count"].as_u64().unwrap_or(0) as u32,
            size: repo_data["size"].as_u64().unwrap_or(0) as u32,
        };

        // Get languages
        self.progress.set_message("Fetching language statistics...");
        let languages: HashMap<String, u32> = self
            .client
            .get(format!("{}/languages", base_url))
            .send()
            .await?
            .json()
            .await?;

        // Get key files
        self.progress.set_message("Fetching repository contents...");
        let mut content = HashMap::new();
        let default_files = vec![
            "README.md",
            "CONTRIBUTING.md",
            "LICENSE",
            "setup.py",
            "requirements.txt",
            "Cargo.toml",
            "package.json",
            "go.mod",
            "composer.json",
            "Gemfile",
        ];

        for path in default_files {
            if let Ok(Some(file_content)) = self.get_file_content(&base_url, path).await {
                content.insert(path.to_string(), file_content);
            }
        }

        self.progress.finish_with_message("Analysis complete!");

        Ok(RepoSummary {
            repo,
            stats,
            languages,
            content,
        })
    }

    async fn get_file_content(&self, base_url: &str, path: &str) -> Result<Option<String>> {
        let url = format!("{}/contents/{}", base_url, path);
        let response = self.client.get(&url).send().await?;

        match response.status() {
            StatusCode::OK => {
                let content: ContentResponse = response.json().await?;
                if content.content_type != "file" {
                    return Ok(None);
                }

                if let Some(encoded) = content.content {
                    let decoded = BASE64.decode(encoded.replace('\n', ""))?;
                    Ok(Some(String::from_utf8(decoded)?))
                } else {
                    Ok(None)
                }
            }
            StatusCode::NOT_FOUND => Ok(None),
            _ => Err(GHError::NetworkError(response.error_for_status().unwrap_err()).into()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let github = GitHub::new(args.token.clone())?;

    match github.get_repo_data(&args.repo).await {
        Ok(summary) => {
            let output = format!("{}_summary.{}", summary.repo.name, args.format);

            let content = match args.format.as_str() {
                "json" => serde_json::to_string_pretty(&summary)?,
                "yaml" => serde_yaml::to_string(&summary)?,
                _ => return Err(anyhow::anyhow!("Unsupported output format")),
            };

            fs::write(&output, content)?;
            println!("✓ Summary saved to {}", output);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn build_client(token: &Option<String>) -> Result<Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("gh-analyzer-rust"),
    );

    let builder = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .default_headers(headers.clone());

    let builder = if let Some(token) = token {
        let mut auth_value = header::HeaderValue::from_str(&format!("Bearer {}", token))?;
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        builder.default_headers(headers)
    } else {
        builder
    };

    Ok(builder.build()?)
}
