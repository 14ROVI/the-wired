use maud::{html, Markup, Render};

pub fn render_iterable<'a, T, I>(iterable: &'a I) -> Markup
where
    T: Render,
    &'a I: IntoIterator<Item = T> + 'a,
{
    html! {
        @for item in iterable {
            (item)
        }
    }
}
