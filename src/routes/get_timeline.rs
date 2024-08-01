use axum::extract::State;
use maud::Markup;
use sqlx::PgPool;

use crate::templates::timeline;

pub async fn get_timeline(State(pool): State<PgPool>) -> Markup {
    timeline(&pool).await
}
