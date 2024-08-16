use maud::{html, Markup};

use crate::structs::Post;

use super::{base, timeline};

pub fn index(posts: Vec<Post>) -> Markup {
    base(html! {
    h1 { "The wired" }
        div.mx-auto.max-w-xl.px-4 {
            h2 { "Posts" }
            div.flex.flex-col.gap-8 {
                form hx-post="/post" hx-swap="afterend" {
                    textarea.w-full id="content" name="content" {}
                    button type="submit" {"Post"}
                }
                (timeline(posts))
            }
        }
    })
}
