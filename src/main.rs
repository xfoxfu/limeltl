#![allow(dead_code)]

mod bool_logic;
mod context;
mod enforcers;
mod sat;
mod utils;

use bool_logic::{PropExpr, Variable};
use enforcers::*;
use sat::convert_cnf;

fn main() {
    let input =
        serde_json::from_reader::<_, context::Input>(std::fs::File::open("input.json").unwrap())
            .unwrap();
    let mut ctx: context::Context = input.into();
    for (word, id) in ctx.words() {
        println!("{} -> {}", word, id);
    }
    for ex in ctx.examples() {
        println!("{:?}", ex);
    }
    for n in 3..4 {
        ctx.set_size_bound(n);
        let rules = ContextEnforcer.rules(&ctx);

        let mut conv = sat::SATConverter::new();

        for rule in rules.into_iter() {
            println!("c {:?}", rule);
            let rule_cnf = convert_cnf(rule);
            conv.add_clause(rule_cnf);
            // println!("{:?}", conv.formula.iter().last().unwrap());
            // varisat::dimacs::write_dimacs_clauses(
            //     &mut std::io::stdout(),
            //     std::iter::once(conv.formula.iter().last().unwrap()),
            // )
            // .unwrap();
        }
        let lit_pos = conv.get_var(&Variable::Exactly(true)).positive();
        let lit_neg = conv.get_var(&Variable::Exactly(false)).negative();
        let lits = vec![
            conv.get_var(&Variable::Until(0)).positive(),
            // conv.get_var(&Variable::LeftChild(0, 1)).positive(),
            // conv.get_var(&Variable::RightChild(0, 2)).positive(),
            // conv.get_var(&Variable::Literal(1)).positive(),
            // conv.get_var(&Variable::Literal(2)).positive(),
            // conv.get_var(&Variable::Word(1, 0, true)).positive(),
            // conv.get_var(&Variable::Word(2, 1, true)).positive(),
        ];
        let (vars, formula) = conv.finish();

        let mut solver = varisat::Solver::new();
        solver.add_formula(&formula);
        solver.assume(&[lit_pos, lit_neg]);
        solver.assume(&lits);

        let result = solver.solve().unwrap();
        println!("n = {}, SAT = {}", n, result);

        if result {
            for v in vars
                .iter()
                .filter(|(_, l)| solver.model().unwrap().contains(&l.positive()))
                .filter_map(|(v, _)| match v {
                    Variable::Run(_, _, _) => None,
                    Variable::Phantom(_) => None,
                    Variable::Exactly(_) => None,
                    s => Some(s),
                })
            {
                println!("assign {:?} = true", v,);
            }
        }
    }
}
