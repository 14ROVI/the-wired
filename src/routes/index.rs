use axum::extract::State;
use maud::html;
use maud::Markup;
use sqlx::PgPool;

use crate::templates::base;
use crate::templates::timeline;

pub async fn index(State(pool): State<PgPool>) -> Markup {
    base(html! {
    h1."text-3xl"."font-bold" { "The wired" }
        div.prose {
            h2 { "Posts" }
            div.flex.flex-col.gap-8.m-4 {
                (timeline(&pool).await)
            }
        }
    })
}
