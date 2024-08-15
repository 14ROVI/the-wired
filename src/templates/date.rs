use chrono::{DateTime, Utc};
use maud::html;
use maud::Markup;

pub fn date(date: DateTime<Utc>) -> Markup {
    html! {
        span.text-xs x-text=(format!("new Date('{}').toLocaleString()", date)) {  }
    }
}
