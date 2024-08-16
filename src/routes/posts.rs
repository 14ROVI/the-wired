use axum::{extract::State, Form};
use chrono::Utc;
use maud::{Markup, Render};
use serde::Deserialize;
use sqlx::PgPool;

use crate::structs::{Post, PostAuthor};

#[derive(Deserialize)]
pub struct CreatePost {
    pub content: String,
}

pub async fn post_post(State(pool): State<PgPool>, Form(post): Form<CreatePost>) -> Markup {
    Post {
        author: PostAuthor {
            avatar: None,
            display_name: "random user".to_string(),
            username: "random_username".to_string(),
        },
        content: post.content,
        created_at: Utc::now(),
    }
    .render()
}
