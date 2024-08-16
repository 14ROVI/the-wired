use maud::html;
use maud::Markup;
use maud::DOCTYPE;

pub fn base(page: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                title { "The Wired" }
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta charset="UTF-8";
                script src="/assets/htmx.min.js" {}
                script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js" {}
                link href="/assets/main.css" rel="stylesheet" type="text/css";
            }
            body.prose.max-w-full x-init hx-headers=r#"js:{"x-timezone": Intl.DateTimeFormat().resolvedOptions().timeZone}"# {
                (page)
            }
        }
    }
}
