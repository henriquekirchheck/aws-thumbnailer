use hypertext::prelude::*;

pub struct Js(pub &'static str);

impl Renderable for Js {
    fn render_to(&self, output: &mut String) {
        maud! {
            script src=(self.0) {}
        }
        .render_to(output);
    }
}
