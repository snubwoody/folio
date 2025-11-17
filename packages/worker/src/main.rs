mod client;

use std::time::SystemTime;
use poem::{get, handler, Route, Server};
use poem::listener::TcpListener;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let app = Route::new()
        .at("/health",get(health));

    let listener = TcpListener::bind("0.0.0.0:8080");
    Server::new(listener)
        .run(app)
        .await?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeatureRequest{
    title: String,
    body: String,
    os: String,
    version: String,
}

fn feature_request(){

}

#[derive(Debug,Serialize,Deserialize)]
struct CreateIssueBody{
    title: String,
    body: String,
}

impl CreateIssueBody{
    pub fn new(title:&str,body:&str)->Self{
        Self{
            title: String::from(title),
            body: String::from(body),
        }
    }
}


#[handler]
fn health(){

}

pub struct Claims{
    pub iss: String,
    pub iat: u64,
    pub exp: u64,
}

impl Claims{
    pub fn new() -> Self{
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self{
            iss: String::from("Iv23libGSoaUaiaWkC3D"),
            iat: now,
            exp: now + 600,
        }
    }
}

fn create_jwt(){
    
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn claims_iss(){
        let claims = Claims::new();
        assert_eq!(claims.iss, "Iv23libGSoaUaiaWkC3D");
    }

    #[test]
    fn claims_expiry(){
        let claims = Claims::new();
        assert_eq!(claims.exp,claims.iat+600);
    }
}