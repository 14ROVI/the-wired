use sqlx::{Pool, Postgres};

use crate::structs::{Post, PostAuthor};

pub async fn timeline(pool: &Pool<Postgres>) -> Vec<Post> {
    sqlx::query!("SELECT posts.*, users.username, users.display_name FROM posts JOIN users ON author=users.id ORDER BY posts.created_at DESC")
        .fetch_all(pool)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|row| Post {
                author: PostAuthor {
                    avatar: "avatar".to_owned(),
                    display_name: row.display_name.to_string(),
                    username: row.username.to_string(),
                },
                content: row.content.to_owned(),
                created_at: row.created_at.to_owned(),
            })
            // .collect()
        })
        .map(|posts| posts.collect())
        .expect("couldnt fetch posts")
}
