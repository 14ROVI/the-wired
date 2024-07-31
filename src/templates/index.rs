use maud::html;
use maud::Markup;
use maud::PreEscaped;
use maud::DOCTYPE;

pub async fn index() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                title { "title of the page" }
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta charset="UTF-8";
                script {
                    (PreEscaped(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "./assets/htmx.min.js"))))
                }
                style {
                    (PreEscaped(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "./assets/main.css"))))
                }
            }
            body {
                h1."text-3xl"."font-bold" { "The wired" }
                div.prose {
                    h2 { "Posts" }
                    div.flex.flex-col.gap-8.m-4 hx-get="/timeline" hx-trigger="load" {}
                }
            }
        }

    }
}
