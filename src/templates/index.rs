use maud::{html, Markup};

use crate::structs::Post;

use super::{base, timeline};

pub fn index(posts: Vec<Post>) -> Markup {
    base(html! {
    h1."text-3xl"."font-bold" { "The wired" }
        div.prose {
            h2 { "Posts" }
            div.flex.flex-col.gap-8.m-4 {
                (timeline(posts))
            }
        }
    })
}
