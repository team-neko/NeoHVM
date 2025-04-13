pub mod tool;
pub mod virt;
pub mod host;
pub mod auth;

use axum::Router;
pub fn routes() -> Router {
    Router::new()
        .merge(tool::routes())
        .nest("/virt", virt::routes())
        .nest("/host", host::routes())
        .nest("/auth", auth::routes())
}
