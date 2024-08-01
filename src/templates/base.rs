use maud::html;
use maud::Markup;
use maud::DOCTYPE;

pub fn base(page: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                title { "title of the page" }
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta charset="UTF-8";
                script src="/assets/htmx.min.js" {}
                link href="/assets/main.css" rel="stylesheet" type="text/css";
            }
            body {
                (page)
            }
        }
    }
}
