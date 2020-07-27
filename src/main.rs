#![allow(dead_code)]

mod bool_logic;
mod context;
mod enforcers;
mod sat;
mod utils;

fn main() {
    let input =
        serde_json::from_reader::<_, context::Input>(std::fs::File::open("input.json").unwrap())
            .unwrap();
    let ctx: context::Context = input.into();
    for ex in ctx.examples() {
        println!("{:?}", ex);
    }
}
