use mason_registry_api::setup_tracing;
use vercel_runtime::{run, bundled_api, Error, Request, Response, Body};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();
    run(handler).await
}

#[bundled_api]
async fn handler(req: Request) -> Result<Response<Body>, Error> {
    tracing::info!("Handling request: {:?req}");
}
