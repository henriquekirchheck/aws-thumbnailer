use hypertext::prelude::*;

#[component]
pub fn uploads() -> impl Renderable {
    maud! {
      article {
        header {
          h1 { "Get your image!" }
        }
        "Hello World!"
      }
    }
}