use crate::{structs::Post, utils::render_iterable};
use maud::Markup;

pub fn timeline(posts: Vec<Post>) -> Markup {
    render_iterable(&posts)
}
