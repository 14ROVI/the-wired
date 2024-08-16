use chrono::{DateTime, Utc};
use maud::html;
use maud::Markup;
use maud::Render;

use crate::templates::date;

pub struct PostAuthor {
    pub display_name: String,
    pub username: String,
    pub avatar: Option<String>,
}

pub struct Post {
    pub author: PostAuthor,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
impl Render for Post {
    fn render(&self) -> Markup {
        html! {
            div.flex.flex-col.gap-2 {
                div.flex.flex-row.gap-4 {
                    img.h-14.my-auto src=(self.author.avatar.clone().unwrap_or("/assets/avatars/default.png".to_string()));
                    div.flex.flex-col {
                        b { (self.author.display_name) }
                        span { (self.author.username) }
                    }
                }
                div {
                    span { (self.content) }
                }
                div {
                    (date(self.created_at))
                }
            }
        }
    }
}
