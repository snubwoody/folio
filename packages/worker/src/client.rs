use crate::CreateIssueBody;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize,Serialize,Default)]
pub struct CreateIssueResponse{
    pub id: u32,
    pub url: String,
    pub repository_url: String,
    /// A number uniquely identifying the issue within its repository.
    pub number: u32,
}

#[derive(Clone)]
pub struct GithubClient {
    base_url: String,
    version: String,
    client: Client,
}

impl GithubClient {
    pub fn new(base_url: &str) -> GithubClient {
        GithubClient {
            base_url: base_url.to_string(),
            version: String::from("2022-11-28"),
            client: Client::new(),
        }
    }

    fn issue_url(&self) -> String {
        format!("{}/repos/snubwoody/folio/issues", self.base_url)
    }

    /// Creates a github issue with the specified title and body.
    pub async fn create_issue(&self, title: &str, body: &str) -> anyhow::Result<CreateIssueBody> {
        let body = CreateIssueBody::new(title, body);
        let response = self.client
            .post(self.issue_url())
            .header("Accept", "application/vnd.github.raw+json")
            .header("Authorization", "Token ")
            .header("X-Github-Api-Version", &self.version)
            .json(&body)
            .send()
            .await?;

        if response.status().is_client_error() || response.status().is_server_error(){
            // TODO: return error
            return Err(anyhow::anyhow!("Failed to create issue"));
        }
        let response_body: CreateIssueBody = response
            .json()
            .await?;
        Ok(response_body)
    }

    // /// Gets a new access token from github
    // async fn access_token(&self) -> anyhow::Result<()> {
    //     let jwt = create_jwt()?;
    //     let installation_id = 94393199;
    //     let url =
    //         format!("https://api.github.com/app/installations/{installation_id}/access_tokens");
    //
    //     Ok(())
    // }
}

#[cfg(test)]
mod test {
    use crate::CreateIssueBody;
    use crate::client::GithubClient;
    use httpmock::Method::POST;
    use httpmock::MockServer;

    #[tokio::test]
    async fn request_headers() -> anyhow::Result<()> {
        let server = MockServer::start();
        let client = GithubClient::new(server.base_url().as_str());
        let mock = server.mock(|when, then| {
            when.method(POST)
                .header("Accept", "application/vnd.github.raw+json")
                .header_includes("Authorization", "Token")
                .header("X-Github-Api-Version", &client.version)
                .path("/repos/snubwoody/folio/issues");
            then.status(200);
        });
        client.create_issue("", "").await?;
        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn request_body() -> anyhow::Result<()> {
        let server = MockServer::start();
        let client = GithubClient::new(server.base_url().as_str());
        let mock = server.mock(|when, then| {
            when.method(POST)
                .json_body_obj(&CreateIssueBody::new("[Feature request]", "Body"))
                .path("/repos/snubwoody/folio/issues");
            then.status(200);
        });
        client.create_issue("[Feature request]", "Body").await?;
        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn correct_issue_endpoint() -> anyhow::Result<()> {
        let server = MockServer::start();
        let client = GithubClient::new(server.base_url().as_str());
        let mock = server.mock(|when, then| {
            when.method(POST).path("/repos/snubwoody/folio/issues");
            then.status(200);
        });
        client.create_issue("", "").await?;
        mock.assert();
        Ok(())
    }
}
