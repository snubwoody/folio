use crate::auth::create_jwt;
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::warn;

const INSTALLATION_ID: u32 = 94393199;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIssueBody {
    title: String,
    body: String,
}

impl CreateIssueBody {
    pub fn new(title: &str, body: &str) -> Self {
        Self {
            title: String::from(title),
            body: String::from(body),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CreateIssueResponse {
    pub id: u32,
    pub url: String,
    pub repository_url: String,
    /// A number uniquely identifying the issue within its repository.
    pub number: u32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AccessTokenResponse {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone)]
pub struct AccessToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl From<AccessTokenResponse> for AccessToken {
    fn from(value: AccessTokenResponse) -> Self {
        Self {
            token: value.token,
            expires_at: value.expires_at,
        }
    }
}

#[derive(Clone)]
pub struct GithubClient {
    base_url: String,
    /// A date string representing the API version.
    version: String,
    client: Client,
    /// The unique identifier of the installation.
    installation_id: u32,
    token: AccessToken,
}

impl GithubClient {
    pub async fn new(base_url: &str) -> anyhow::Result<Self> {
        let mut client = GithubClient {
            base_url: base_url.to_string(),
            version: String::from("2022-11-28"),
            client: Client::new(),
            installation_id: INSTALLATION_ID,
            // Set to a dummy default token
            token: AccessToken::default(),
        };

        let token = client.access_token().await?;
        client.token = token.into();
        Ok(client)
    }

    /// Returns the name of the Github app.
    pub fn name(&self) -> &str {
        "Folio Automated Issues"
    }

    /// Returns true if the access token has expired or has an expiry within
    /// 10 minutes from now.
    pub fn token_expired(&self) -> bool {
        Utc::now() >= (self.token.expires_at - Duration::minutes(10))
    }

    /// Checks if the token is expired and refreshes it.
    pub async fn refresh_token(&mut self) -> anyhow::Result<()> {
        if !self.token_expired() {
            return Ok(());
        }

        self.token = self.access_token().await?.into();
        Ok(())
    }

    fn issue_url(&self) -> String {
        format!("{}/repos/snubwoody/folio/issues", self.base_url)
    }

    fn access_token_url(&self) -> String {
        format!(
            "{}/app/installations/{}/access_tokens",
            self.base_url, self.installation_id
        )
    }

    /// Creates a Github issue with the specified title and body.
    ///
    /// [API Endpoint](https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#create-an-issue)
    pub async fn create_issue(
        &mut self,
        title: &str,
        body: &str,
    ) -> anyhow::Result<CreateIssueResponse> {
        self.refresh_token().await?;
        let body = CreateIssueBody::new(title, body);
        let response = self
            .client
            .post(self.issue_url())
            .header("Accept", "application/vnd.github.raw+json")
            .header("Authorization", format!("Token {}", self.token.token))
            .header("User-Agent", self.name())
            .header("X-Github-Api-Version", &self.version)
            .json(&body)
            .send()
            .await?;

        if response.status().is_client_error() || response.status().is_server_error() {
            let response_body = response.text().await?;
            warn!(response = response_body, "Failed to create issue");
            return Err(anyhow::anyhow!("Failed to create issue"));
        }
        let response_body: CreateIssueResponse = response.json().await?;
        Ok(response_body)
    }

    /// Retrieves a new access token from Github.
    ///
    /// Installation tokens expire one hour from the time you create them.
    ///
    /// [API Endpoint](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#create-an-installation-access-token-for-an-app)
    async fn access_token(&self) -> anyhow::Result<AccessTokenResponse> {
        let jwt = create_jwt()?;
        let response = self
            .client
            .post(self.access_token_url())
            .header("Accept", "application/vnd.github.raw+json")
            .header("Authorization", format!("Bearer {}", jwt))
            .header("User-Agent", self.name())
            .header("X-Github-Api-Version", &self.version)
            .send()
            .await?;

        if response.status().is_client_error() || response.status().is_server_error() {
            let response_body = response.text().await?;
            dbg!(&response_body);
            warn!(response = response_body, "Failed to create access token");
            return Err(anyhow::anyhow!("Failed to create access token"));
        }
        let response_body: AccessTokenResponse = response.json().await?;
        Ok(response_body)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::auth::create_private_key;
    use chrono::Utc;
    use httpmock::Method::POST;
    use httpmock::{Mock, MockServer};

    async fn access_token_mock(server: &'_ MockServer) -> anyhow::Result<Mock<'_>> {
        let key = create_private_key()?;
        unsafe { std::env::set_var("WORKER_PRIVATE_KEY", key) }
        let mock = server.mock(|when, then| {
            when.method(POST)
                .header("Accept", "application/vnd.github.raw+json")
                .header_includes("Authorization", "Bearer ")
                .path(format!(
                    "/app/installations/{INSTALLATION_ID}/access_tokens"
                ));
            let expires_at = Utc::now() + Duration::minutes(10);
            let body = AccessTokenResponse {
                expires_at,
                ..Default::default()
            };
            then.status(201).json_body_obj(&body);
        });
        Ok(mock)
    }

    #[tokio::test]
    async fn access_token_url() -> anyhow::Result<()> {
        let server = MockServer::start();
        let mock = access_token_mock(&server).await?;
        let client = GithubClient::new(server.base_url().as_str()).await?;
        client.access_token().await?;
        mock.assert_calls(2);
        Ok(())
    }

    #[tokio::test]
    async fn fetch_access_token_on_init() -> anyhow::Result<()> {
        let server = MockServer::start();
        let mock = access_token_mock(&server).await?;
        GithubClient::new(server.base_url().as_str()).await?;
        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn create_issue_request_headers() -> anyhow::Result<()> {
        let server = MockServer::start();
        access_token_mock(&server).await?;
        let mut client = GithubClient::new(server.base_url().as_str()).await?;
        let mock = server.mock(|when, then| {
            when.method(POST)
                .header("Accept", "application/vnd.github.raw+json")
                .header_includes("Authorization", "Token")
                .header("User-Agent", client.name())
                .header("X-Github-Api-Version", &client.version)
                .path("/repos/snubwoody/folio/issues");
            let body = CreateIssueResponse::default();
            then.status(201).json_body_obj(&body);
        });
        client.create_issue("", "").await?;
        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn request_body() -> anyhow::Result<()> {
        let server = MockServer::start();
        access_token_mock(&server).await?;
        let mut client = GithubClient::new(server.base_url().as_str()).await?;
        let mock = server.mock(|when, then| {
            when.method(POST)
                .json_body_obj(&CreateIssueBody::new("[Feature request]", "Body"))
                .path("/repos/snubwoody/folio/issues");
            let body = CreateIssueResponse::default();
            then.status(201).json_body_obj(&body);
        });
        client.create_issue("[Feature request]", "Body").await?;
        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn correct_issue_endpoint() -> anyhow::Result<()> {
        let server = MockServer::start();
        access_token_mock(&server).await?;
        let mut client = GithubClient::new(server.base_url().as_str()).await?;
        let mock = server.mock(|when, then| {
            when.method(POST).path("/repos/snubwoody/folio/issues");
            let body = CreateIssueResponse::default();
            then.status(201).json_body_obj(&body);
        });
        client.create_issue("", "").await?;
        mock.assert();
        Ok(())
    }
}
