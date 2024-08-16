use sqlx::{Pool, Postgres};

use crate::structs::{Post, PostAuthor};

pub async fn timeline(pool: &Pool<Postgres>) -> Result<Vec<Post>, sqlx::Error> {
    let records = sqlx::query!("SELECT posts.*, users.username, users.display_name, users.avatar_ref FROM posts JOIN users ON author=users.id ORDER BY posts.created_at DESC")
        .fetch_all(pool)
        .await?;

    let posts = records
        .into_iter()
        .map(|row| Post {
            author: PostAuthor {
                avatar: row.avatar_ref,
                display_name: row.display_name,
                username: row.username,
            },
            content: row.content,
            created_at: row.created_at,
        })
        .collect();

    Ok(posts)
}
