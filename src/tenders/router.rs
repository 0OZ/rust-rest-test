use axum::{routing::get, Router};

use crate::tenders::handler::list_tenders;

pub fn create_tender_router() -> Router {
    Router::new().route("/", get(list_tenders))
}
