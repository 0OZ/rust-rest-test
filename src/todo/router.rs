use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    todo::handler::{
        create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler,
        health_checker_handler, todos_list_handler,
    },
    todo::model,
};

pub fn create_todo_router() -> Router {
    let db = model::todo_db();

    Router::new()
        .route("/healthchecker", get(health_checker_handler))
        .route("/", post(create_todo_handler).get(todos_list_handler))
        .route(
            "/:id",
            get(get_todo_handler)
                .patch(edit_todo_handler)
                .delete(delete_todo_handler),
        )
        .with_state(db)
}
