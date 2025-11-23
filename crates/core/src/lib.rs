use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct FeatureRequest {
    pub title: String,
    pub description: String,
    pub os: String,
    pub version: String,
}

impl FeatureRequest {
    pub fn issue_title(&self) -> String {
        format!("[Feature request] {}", self.title)
    }
    pub fn body(&self) -> String {
        let mut body = String::new();
        body.push_str(&format!("OS: {}\n", self.os));
        body.push_str(&format!("Version: {}\n", self.version));
        body.push_str("## Description\n\n");
        body.push_str(&self.description.to_string());
        body
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BugReport {
    pub title: String,
    pub version: String,
    pub os: String,
    pub description: String,
}

impl BugReport {
    pub fn issue_title(&self) -> String {
        format!("[Bug report] {}", self.title)
    }
    pub fn body(&self) -> String {
        let mut body = String::new();
        body.push_str(&format!("OS: {}\n", self.os));
        body.push_str(&format!("Version: {}\n", self.version));
        body.push_str("## Description\n\n");
        body.push_str(&self.description.to_string());
        body
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SupportResponse {
    pub issue_url: String,
    pub issue_id: u32,
}
