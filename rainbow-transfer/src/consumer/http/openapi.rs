use utoipa::openapi::OpenApi as OpenApiModel;
use utoipa::{OpenApi, PartialSchema};
use utoipa_redoc::{Redoc, Servable};

// https://github.com/juhaku/utoipa/blob/master/examples/axum-utoipa-bindings/src/main.rs
#[derive(OpenApi)]
struct HighLevelConsumerApiDoc;

pub fn open_api_setup() -> anyhow::Result<Redoc<OpenApiModel>> {
    let api_docs = HighLevelConsumerApiDoc::openapi();
    let router = Redoc::with_url("/api/v1", api_docs);
    Ok(router)
}
