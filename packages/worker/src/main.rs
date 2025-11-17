mod client;
mod middleware;

use std::sync::Arc;
use dotenvy::dotenv;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use poem::listener::TcpListener;
use poem::{Route, Server, get, handler, EndpointExt, Endpoint, Request};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use poem::http::Method;
use poem::middleware::{Cors, FnMiddleware, ForceHttps, RequestId};
use poem::web::Data;
use crate::client::GithubClient;
use crate::middleware::{logging_middleware};

const GITHUB_API_URL: &str = "https://api.github.com";

#[derive(Clone)]
struct State{
    client: Arc<GithubClient>
}

impl State {
    pub fn new() -> State{
        Self{
           client: Arc::new(GithubClient::new(GITHUB_API_URL))
        }
    }

    pub fn client(&self) -> Arc<GithubClient> {
        self.client.clone()
    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv();
    tracing_subscriber::fmt::init();
    let cors = Cors::new()
        .allow_method(Method::GET)
        .allow_method(Method::POST);

    let mut app = Route::new()
        .at("/health", get(health))
        .with(cors)
        .data(State::new())
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


async fn feature_request( Data(state):Data<State> ) {
    // state.client().create_issue("","");
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateIssueBody {
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
async fn health() {}

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

fn create_jwt() -> anyhow::Result<String> {
    let key = std::env::var("WORKER_PRIVATE_KEY")?;
    let claims = Claims::new();
    let key = EncodingKey::from_rsa_pem(key.as_bytes())?;
    let token = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, &key)?;
    Ok(token)
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn jwt() -> anyhow::Result<()> {
    //     let rng = rand::rng();
    //     unsafe {
    //         std::env::set_var("WORKER_PRIVATE_KEY", "");
    //     }
    //     create_jwt()?;
    //     Ok(())
    // }

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
