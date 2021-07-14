pub use dialoguer;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input};
pub use dialoguer_macro::Dialogue;
use std::io;

pub trait Dialogue {
    fn compose(prompt: &str) -> io::Result<Self>
    where
        Self: Sized;
}

macro_rules! impl_dialogue {
    ($typ: ty) => {
        impl Dialogue for $typ {
            fn compose(prompt: &str) -> io::Result<Self> {
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(prompt)
                    .interact()
            }
        }
    };
}

impl_dialogue!(String);
impl_dialogue!(i8);
impl_dialogue!(i16);
impl_dialogue!(i32);
impl_dialogue!(i64);
impl_dialogue!(u8);
impl_dialogue!(u16);
impl_dialogue!(u32);
impl_dialogue!(u64);
impl_dialogue!(f32);
impl_dialogue!(f64);
impl_dialogue!(usize);
impl_dialogue!(isize);

impl Dialogue for bool {
    fn compose(prompt: &str) -> io::Result<Self>
    where
        Self: Sized,
    {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact()
    }
}
