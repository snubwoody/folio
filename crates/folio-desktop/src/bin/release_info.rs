
#[tokio::main]
async fn main() -> folio_lib::Result<()>{
    tracing_subscriber::fmt().init();
    folio_lib::update::release_info().await?;
    Ok(())
}