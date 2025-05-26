use crate::extensions::AriaAttributes;
use hypertext::prelude::*;

#[component]
pub fn nav<'a>(current: &'a str, oob: bool) -> impl Renderable {
    let pages = [("Home", "/"), ("Uploads", "/uploads")];
    maud! {
        nav #nav hx-swap-oob=(oob) {
            ul { li { strong { "Thumbnailer" } } }
            ul {
                @for (name, path) in pages {
                    li {
                        a href=(path) aria-current={
                            @if path == current { "page" } @else { (false) }
                        } hx-get=(path) hx-target="main" hx-swap="innerHTML" hx-push-url=(true) { (name) }
                    }
                }
            }
        }
    }
}
