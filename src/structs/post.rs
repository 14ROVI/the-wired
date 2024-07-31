use chrono::{DateTime, Utc};
use maud::html;
use maud::Markup;
use maud::Render;

pub struct PostAuthor {
    pub display_name: String,
    pub username: String,
    pub avatar: String,
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
                div.flex.flex-row {
                    img src=(self.author.avatar);
                    div.flex.flex-col {
                        b { (self.author.display_name) }
                        span { (self.author.username) }
                    }
                }
                div {
                    span { (self.content) }
                }
                div {
                    span.text-xs { (self.created_at.format("%d/%m/%Y %H:%M")) }
                }
            }
        }
    }
}
