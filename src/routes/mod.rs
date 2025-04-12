pub mod tool;
pub mod virt;
use axum::Router;
pub fn routes() -> Router {
    Router::new()
        .merge(tool::routes())
        .nest("/virt", virt::routes())
}
