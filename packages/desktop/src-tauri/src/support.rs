// Copyright (C) 2025 Wakunguma Kalimukwa
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use folio_core::{BugReport, FeatureRequest, SupportResponse};
use reqwest::Client;

const API_URL: &str = "https://folio-worker.fly.dev";

/// Sends a feature request to the Github bot.
pub async fn feature_request(request: FeatureRequest) -> crate::Result<SupportResponse> {
    let url = format!("{}/api/v1/features", API_URL);
    let client = Client::new();

    let response = client.post(&url).json(&request).send().await?;

    let body: SupportResponse = response.json().await?;
    Ok(body)
}

/// Sends a bug report to the Github bot.
pub async fn bug_report(report: BugReport) -> crate::Result<SupportResponse> {
    let url = format!("{}/api/v1/bugs", API_URL);
    let client = Client::new();

    let response = client.post(&url).json(&report).send().await?;

    let body: SupportResponse = response.json().await?;
    Ok(body)
}
