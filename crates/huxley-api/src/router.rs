use axum::Router;
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    limit::RequestBodyLimitLayer,
};
use std::time::Duration;
use huxley_state::HuxleyState;

use crate::{HuxleyApiResult, routes};

pub fn build_router(state: HuxleyState) -> HuxleyApiResult<Router> {
    let api = Router::new()
        .nest("/me", routes::me::router());

    let router = Router::new()
        .nest("/api/v1", api)
        .merge(routes::health::router())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(CompressionLayer::new())
        .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .with_state(state);

    Ok(router)
}
