use chrono::{DateTime, TimeZone, Utc};
use maud::Markup;

use crate::{
    structs::{Post, PostAuthor},
    utils::render_iterable,
};

pub async fn get_timeline() -> Markup {
    let posts = vec![
        Post {
            author: PostAuthor {
                avatar: "avatar".to_owned(),
                display_name: "rovi".to_owned(),
                username: "14rovi".to_owned(),
            },
            content: "this is the content of my post wow this is so cool!".to_owned(),
            created_at: DateTime::<Utc>::MAX_UTC,
        },
        Post {
            author: PostAuthor {
                avatar: "avatar".to_owned(),
                display_name: "Abi".to_owned(),
                username: "iameveryoneelse".to_owned(),
            },
            content: ":DDDDDD".to_owned(),
            created_at: Utc.with_ymd_and_hms(2024, 7, 6, 16, 0, 0).unwrap(),
        },
    ];

    render_iterable(&posts)
}
