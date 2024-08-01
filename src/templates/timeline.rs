use crate::{
    structs::{Post, PostAuthor},
    utils::render_iterable,
};
use maud::Markup;
use sqlx::{Pool, Postgres};

pub async fn timeline(pool: &Pool<Postgres>) -> Markup {
    let posts: Vec<Post> = sqlx::query!("SELECT * FROM posts")
        .fetch_all(pool)
        .await
        .map(|rows| {
            rows.into_iter().map(|row| Post {
                author: PostAuthor {
                    avatar: "avatar".to_owned(),
                    display_name: row.author.to_string(),
                    username: row.author.to_string(),
                },
                content: row.content.to_owned(),
                created_at: row.created_at.to_owned(),
            })
            // .collect()
        })
        .map(|posts| posts.collect())
        .expect("couldnt fetch posts");

    render_iterable(&posts)
}
