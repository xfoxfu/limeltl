#![allow(dead_code)]

mod bool_logic;
mod context;
mod enforcers;
mod sat;
mod utils;

use bool_logic::PropExpr;
use enforcers::*;

fn main() {
    let input =
        serde_json::from_reader::<_, context::Input>(std::fs::File::open("input.json").unwrap())
            .unwrap();
    let mut ctx: context::Context = input.into();
    for ex in ctx.examples() {
        println!("{:?}", ex);
    }
    for n in 2..5 {
        ctx.set_size_bound(n);
        let rules = ContextEnforcer.rules_cnf(&ctx);

        let mut conv = sat::SATConverter::new();
        let expr = PropExpr::chained_and(rules);

        conv.add_clause(expr);
        let (vars, formula) = conv.finish();

        let mut solver = varisat::Solver::new();
        solver.add_formula(&formula);
        println!("n = {}, SAT = {}", n, solver.solve().unwrap());

        for v in vars
            .iter()
            .filter(|(_, l)| solver.model().unwrap().contains(&l.positive()))
            .map(|(v, _)| v)
        {
            println!("assign {:?} = true", v,);
        }
    }
}
