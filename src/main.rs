#![allow(dead_code)]

mod bool_logic;
mod context;
mod enforcers;
mod sat;
mod utils;

use bool_logic::{PropExpr, Variable};
use enforcers::*;

fn main() {
    let input =
        serde_json::from_reader::<_, context::Input>(std::fs::File::open("input.json").unwrap())
            .unwrap();
    let mut ctx: context::Context = input.into();
    for ex in ctx.examples() {
        println!("{:?}", ex);
    }
    // for i in 2..5 {
    ctx.set_size_bound(3);

    let assign = {
        use crate::bool_logic::Variable::*;
        vec![
            Until(0),
            Literal(1),
            Literal(2),
            LeftChild(0, 1),
            RightChild(0, 2),
            Word(1, ctx.get_word_id("q".to_owned()), true),
            Word(2, ctx.get_word_id("p".to_owned()), true),
        ]
    };
    let rules = {
        const SK_TYPES: &[fn(usize) -> Variable] = &[
            Variable::Literal,
            Variable::And,
            Variable::Or,
            Variable::Until,
            Variable::Release,
            Variable::Eventually,
            Variable::Next,
            Variable::WNext,
            Variable::Always,
        ];

        let n = ctx.max_skeletons();
        let mut ret = vec![];
        // AFASkTypeEnforcer
        // println!("running at AFASkTypeEnforcer");
        // for i in 0..n {
        //     ret.append(&mut AFASkTypeEnforcer::new(i).rules_cnf(&ctx));
        // }
        // AFASpecificStructureEnforcer
        println!("running at AFASpecificStructureEnforcer");
        for i in 0..n {
            for ty in SK_TYPES {
                ret.append(&mut AFASpecificStructureEnforcer::new(ty(i)).rules_cnf(&ctx));
                println!(
                    "Expr validate {:?} {}",
                    &assign,
                    PropExpr::chained_and(ret.clone())._validate(&assign)
                );
            }
        }

        // SizeBoundEnforcer
        println!("running at SizeBoundEnforcer");
        for i in 0..n {
            ret.append(&mut SizeBoundEnforcer::new(i).rules_cnf(&ctx));
            println!(
                "Expr validate {:?} {}",
                &assign,
                PropExpr::chained_and(ret.clone())._validate(&assign)
            );
        }

        // LTLSubtreeEnforcer
        println!("running at LTLSubtreeEnforcer");
        for i in 0..n {
            for ty in SK_TYPES {
                ret.append(&mut LTLSubtreeEnforcer::new(ty(i)).rules_cnf(&ctx));
                println!(
                    "Expr validate {:?} {}",
                    &assign,
                    PropExpr::chained_and(ret.clone())._validate(&assign)
                );
            }
        }

        // LTLSizeEnforcer
        println!("running at LTLSizeEnforcer");
        ret.append(&mut LTLSizeEnforcer::new().rules_cnf(&ctx));
        println!(
            "Expr validate {:?} {}",
            &assign,
            PropExpr::chained_and(ret.clone())._validate(&assign)
        );
        // PositiveExampleEnforcer
        println!("running at PositiveExampleEnforcer");
        for i in 0..n {
            for ty in SK_TYPES {
                for e in ctx.examples() {
                    if e.is_pos() {
                        ret.append(&mut PositiveExampleEnforcer::new(ty(i), e).rules_cnf(&ctx));
                    }
                }
            }
        }
        println!(
            "Expr validate {:?} {}",
            &assign,
            PropExpr::chained_and(ret.clone())._validate(&assign)
        );

        ret
    };
    println!(
        "final validate {:?} {}",
        &assign,
        PropExpr::chained_and(rules.clone())._validate(&assign)
    );

    let mut conv = sat::SATConverter::new();
    let expr = PropExpr::chained_and(rules);
    let sat_asgn = assign
        .iter()
        .map(|v| conv.get_var(v).positive())
        .collect::<Vec<varisat::Lit>>();

    conv.add_clause(expr);
    println!("SAT CNF Lits = {:?}", &conv.vars);
    let vars = conv.vars.clone();
    let formula = conv.finish();

    let mut solver = varisat::Solver::new();
    solver.add_formula(&formula);
    solver.assume(&sat_asgn);
    println!("SAT CNF Lits = {:?}", formula.var_count());
    println!("result {} = {:?} {:?}", 3, solver.solve(), solver.model());

    for v in vars
        .iter()
        .filter(|(v, l)| solver.model().unwrap().contains(&l.positive()))
        .map(|(v, l)| v)
    {
        println!("assign {:?} = true", v,);
    }
}
