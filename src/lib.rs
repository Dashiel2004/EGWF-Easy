#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]
mod kazuya;
#[skyline::main(name = "ewgf_easy")]
pub fn main() {
    kazuya::install();
}

