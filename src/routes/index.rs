use axum::extract::State;
use maud::Markup;
use sqlx::PgPool;

use crate::{db, templates};

pub async fn index(State(pool): State<PgPool>) -> Markup {
    let posts = db::timeline(&pool).await.expect("Couldn't fetch posts");
    templates::index(posts)
}
