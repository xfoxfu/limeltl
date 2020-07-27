#![allow(dead_code)]

mod bool_logic;
mod context;
mod enforcers;
mod sat;
mod utils;

use bool_logic::PropExpr;
use enforcers::Enforcer;

fn main() {
    let input =
        serde_json::from_reader::<_, context::Input>(std::fs::File::open("input.json").unwrap())
            .unwrap();
    let mut ctx: context::Context = input.into();
    for ex in ctx.examples() {
        println!("{:?}", ex);
    }
    ctx.set_size_bound(1);

    println!(
        "{:?}",
        PropExpr::chained_and(enforcers::ContextEnforcer.rules(&ctx))
    );
}
