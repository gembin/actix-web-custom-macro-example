pub mod middleware;
pub mod macros {
    pub use actix_web_custom_macro::*;
}

#[derive(Debug)]
pub struct Foo {
    bar: String,
}

impl Foo {
    pub fn new<S: Into<String>>(bar: S) -> Self {
        Self {
            bar: bar.into(),
        }
    }
}
