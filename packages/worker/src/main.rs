mod auth;
mod client;
mod middleware;

use crate::client::GithubClient;
use crate::middleware::logging_middleware;
use dotenvy::dotenv;
use folio_core::{BugReport, FeatureRequest, SupportResponse};
use poem::http::{Method, StatusCode};
use poem::listener::TcpListener;
use poem::middleware::Cors;
use poem::web::{Data, Json};
use poem::{Body, EndpointExt, IntoResponse, Response, Route, Server, get, handler, post};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::warn;

const GITHUB_API_URL: &str = "https://api.github.com";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv();
    tracing_subscriber::fmt::init();
    let cors = Cors::new()
        .allow_method(Method::GET)
        .allow_method(Method::POST);

    let client = GithubClient::new(GITHUB_API_URL).await?;
    let data = Arc::new(Mutex::new(client));
    let app = Route::new()
        .at("/health", get(health))
        .at("/api/v1/features", post(feature_request))
        .at("/api/v1/bugs", post(bug_report))
        .with(cors)
        .data(data)
        .around(logging_middleware);

    let listener = TcpListener::bind("0.0.0.0:8080");
    Server::new(listener).run(app).await?;

    Ok(())
}

#[handler]
async fn feature_request(
    Json(request): Json<FeatureRequest>,
    Data(client): Data<&Arc<Mutex<GithubClient>>>,
) -> impl IntoResponse {
    let mut client = client.lock().await;
    let response = client
        .create_issue(&request.issue_title(), &request.body())
        .await;

    match response {
        Ok(issue) => {
            let response = SupportResponse {
                issue_url: issue.url,
                issue_id: issue.number,
            };

            ApiResponse::created(response)
        }
        Err(error) => {
            let message = error.to_string();
            warn!(error = message, "Failed to create feature request");
            ApiResponse::error(&message, StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[handler]
async fn bug_report(
    Json(report): Json<BugReport>,
    Data(client): Data<&Arc<Mutex<GithubClient>>>,
) -> impl IntoResponse {
    let mut client = client.lock().await;
    let response = client
        .create_issue(&report.issue_title(), &report.body())
        .await;

    match response {
        Ok(issue) => {
            let response = SupportResponse {
                issue_url: issue.url,
                issue_id: issue.number,
            };

            ApiResponse::created(response)
        }
        Err(error) => {
            let message = error.to_string();
            warn!(error = message, "Failed to create bug report");
            ApiResponse::error(&message, StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug)]
pub struct ApiResponse<T> {
    status: StatusCode,
    body: Option<T>,
    error: Option<ApiError>,
}

impl<T: Serialize + Send + Sync> ApiResponse<T> {
    pub fn new(status: StatusCode, body: Option<T>, error: Option<ApiError>) -> Self {
        Self {
            status,
            body,
            error,
        }
    }

    /// Created an api response with a 201 status code
    pub fn created(body: T) -> Self {
        Self::new(StatusCode::CREATED, Some(body), None)
    }

    pub fn error(message: &str, code: StatusCode) -> Self {
        Self::new(code, None, Some(ApiError::new(message)))
    }
}

impl<T: Serialize + Send + Sync> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let body = match self.body {
            Some(body) => Body::from_json(&body).unwrap(),
            None => Body::from_json(self.error.unwrap()).unwrap(),
        };

        (self.status, body).into_response()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiError {
    message: String,
}

impl ApiError {
    pub fn new(message: &str) -> ApiError {
        Self {
            message: String::from(message),
        }
    }
}

#[handler]
async fn health() -> &'static str {
    "The server is up and running"
}
