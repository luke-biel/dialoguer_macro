#![allow(dead_code)]

use dialoguer_trait::Dialogue;

#[derive(Dialogue)]
struct Types {
    #[dialogue(prompt = "u8")]
    an_u8: u8,
    #[dialogue(prompt = "u16")]
    an_u16: u16,
    #[dialogue(prompt = "u32")]
    an_u32: u32,
    #[dialogue(prompt = "u64")]
    an_u64: u64,
    #[dialogue(prompt = "i8")]
    an_i8: i8,
    #[dialogue(prompt = "i16")]
    an_i16: i16,
    #[dialogue(prompt = "i32")]
    an_i32: i32,
    #[dialogue(prompt = "i64")]
    an_i64: i64,
    #[dialogue(prompt = "f32")]
    a_f32: f32,
    #[dialogue(prompt = "f64")]
    a_f64: f64,
    #[dialogue(prompt = "usize")]
    an_usize: usize,
    #[dialogue(prompt = "isize")]
    an_isize: isize,
    #[dialogue(prompt = "bool")]
    a_bool: bool,
    #[dialogue(prompt = "string")]
    a_string: String,
}

fn main() {
    Types::compose("Walk-through through all supported types!").unwrap();
}
