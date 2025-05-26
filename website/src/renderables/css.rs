use hypertext::prelude::*;

pub struct Css(pub &'static str);

impl Renderable for Css {
    fn render_to(&self, output: &mut String) {
        maud! {
            link rel="stylesheet" type="text/css" href=(self.0);
        }
        .render_to(output);
    }
}
