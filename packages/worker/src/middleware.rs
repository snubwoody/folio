use poem::{Endpoint, IntoResponse, Middleware, Request, Response};
use tracing::{info, warn};

pub async fn logging_middleware<E: Endpoint>(next: E, req: Request) -> poem::Result<Response> {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    let res = next.call(req).await;
    match res {
        Ok(resp) => {
            let resp = resp.into_response();
            let status = resp.status().as_u16();
            info!("[{status}] {method} {path}");
            Ok(resp)
        }
        Err(err) => {
            let status = err.status().as_u16();
            warn!("[{status}] {method} {path}");
            Err(err)
        }
    }
}
