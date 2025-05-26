use hypertext::prelude::*;

use crate::{renderables::{css::Css, js::Js}, views::nav::Nav};

#[component]
pub fn document<'a, R: Renderable>(children: &R, current: &'a str) -> impl Renderable {
    maud! {
        !DOCTYPE
        html {
            head {
                title { "Thumbnailer" }
                (Css("/assets/pico.purple.min.css"))
                (Js("/assets/htmx.min.js"))
            }
            body {
                header {
                    div .container {
                        Nav current=(current) oob=(true);
                    }
                }
                main .container {
                    (children)
                }
            }
        }
    }
}