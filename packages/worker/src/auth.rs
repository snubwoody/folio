use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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
pub fn create_jwt() -> anyhow::Result<String> {
    let key = std::env::var("WORKER_PRIVATE_KEY")?;
    let claims = Claims::new();
    let key = EncodingKey::from_rsa_pem(key.as_bytes())?;
    let token = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, &key)?;
    Ok(token)
}

#[cfg(test)]
pub fn create_private_key() -> anyhow::Result<String> {
    use rsa::pkcs1::LineEnding;
    use rsa::pkcs8::EncodePrivateKey;

    let mut rng = rsa::rand_core::OsRng;
    let key = rsa::RsaPrivateKey::new(&mut rng, 2048)?;
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
