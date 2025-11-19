mod client;
mod middleware;

use crate::client::GithubClient;
use crate::middleware::logging_middleware;
use dotenvy::dotenv;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use poem::http::Method;
use poem::listener::TcpListener;
use poem::middleware::Cors;
use poem::web::{Data, Json};
use poem::{Body, EndpointExt, Route, Server, get, handler, post};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;


const GITHUB_API_URL: &str = "https://api.github.com";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv();
    tracing_subscriber::fmt::init();
    let cors = Cors::new()
        .allow_method(Method::GET)
        .allow_method(Method::POST);

    // TODO: wrap client in arc
    let client = GithubClient::new(GITHUB_API_URL).await?;
    let app = Route::new()
        .at("/health", get(health))
        .at("/api/v1/features", post(feature_request))
        .with(cors)
        .data(client)
        .around(logging_middleware);

    let listener = TcpListener::bind("0.0.0.0:8080");
    Server::new(listener).run(app).await?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeatureRequest {
    title: String,
    body: String,
    os: String,
    version: String,
}

#[handler]
async fn feature_request(
    Json(FeatureRequest { title, body, .. }): Json<FeatureRequest>,
    Data(client): Data<&GithubClient>,
) {
    // FIXME
    let _ = client.create_issue(&title, &body).await;
}

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

#[handler]
async fn health() -> &'static str {
    "The server is up and running"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub iat: u64,
    pub exp: u64,
}

impl Default for Claims {
    fn default() -> Self {
        Self::new()
    }
}

impl Claims {
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            iss: String::from("Iv23libGSoaUaiaWkC3D"),
            iat: now,
            exp: now + 600,
        }
    }
}

/// Creates a json web token using the `WORKER_PRIVATE_KEY`
/// environment variable as the private key.
fn create_jwt() -> anyhow::Result<String> {
    let key = std::env::var("WORKER_PRIVATE_KEY")?;
    let claims = Claims::new();
    let key = EncodingKey::from_rsa_pem(key.as_bytes())?;
    let token = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, &key)?;
    Ok(token)
}

#[cfg(test)]
fn create_private_key() -> anyhow::Result<String> {
    use rsa::pkcs1::LineEnding;
    use rsa::pkcs8::EncodePrivateKey;

    let mut rng = rsa::rand_core::OsRng;
    let key = rsa::RsaPrivateKey::new(&mut rng,2048)?;
    let pem = key.to_pkcs8_pem(LineEnding::LF)?;
    // This is just for testing so we can leave the
    // security features
    Ok(pem.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn claims_iss() {
        let claims = Claims::new();
        assert_eq!(claims.iss, "Iv23libGSoaUaiaWkC3D");
    }

    #[test]
    fn claims_expiry() {
        let claims = Claims::new();
        assert_eq!(claims.exp, claims.iat + 600);
    }
}
