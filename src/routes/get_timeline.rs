use axum::extract::State;
use maud::Markup;
use sqlx::PgPool;

use crate::{db, templates};

pub async fn get_timeline(State(pool): State<PgPool>) -> Markup {
    let posts = db::timeline(&pool).await;
    templates::timeline(posts)
}
