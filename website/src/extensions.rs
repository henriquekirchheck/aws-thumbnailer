use hypertext::{prelude::GlobalAttributes, validation::Attribute};

#[expect(non_upper_case_globals)]
#[allow(unused)]
pub trait AriaAttributes: GlobalAttributes {
    const aria_current: Attribute = Attribute;
}

impl<T: GlobalAttributes> AriaAttributes for T {}
