
pub use std::sync::{Arc, Mutex};

pub use cursive::{utils::markup::StyledString, views::*, Cursive, view::*};
pub use crate::math::vector::BinaryVector;
pub use crate::math::matrix::GenMatrix;
pub use crate::input::input_fields::*;
pub use crate::input::text_area_v2::*;
pub use crate::parameters::*;
pub use crate::channel::reed_muller::*;
pub use crate::channel::channel::*;

pub fn error_popup<S: Into<StyledString>, P: Into<StyledString>>(terminal: &mut Cursive, title: S, message: P) {
    terminal.add_layer(Dialog::around(TextView::new(message))
        .title(title)
        .button("Gerai", |term| {
            term.pop_layer();
        })
    );
}
