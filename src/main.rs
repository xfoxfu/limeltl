mod bool_logic;
mod context;
mod enforcers;
mod ltl;
mod options;
mod sat;
mod utils;

use bool_logic::Variable;
use clap::derive::Clap;
use context::Context;
use enforcers::{ContextEnforcer, Enforcer};
use options::Opts;
use sat::convert_cnf;
use std::io::Write;

fn solve_iter(ctx: &Context, opts: &Opts, output: &mut impl Write) -> Result<(), ()> {
    // 构造规则
    let rules = ContextEnforcer.rules(&ctx);

    if opts.fmt_expr {
        for rule in rules.iter() {
            writeln!(output, "{:?}", rule).expect("写入规则失败");
        }
    }

    // 生成 CNF
    let mut solver = maple::Solver::new();
    let mut conv = sat::SATConverter::new(&mut solver);
    for rule in rules.into_iter() {
        let rule_cnf = convert_cnf(rule);
        conv.add_clause(rule_cnf);
    }
    let lit_pos = conv.get_var(&Variable::Exactly(true)).clone();
    let lit_neg = !conv.get_var(&Variable::Exactly(false)).clone();
    let vars = conv.finish();

    if opts.fmt_cnf {
        // maple::dimacs::write_dimacs(output, &formula).expect("写入规则失败");
    }

    // 求解
    let result = solver.solve_under_assumptions(vec![lit_pos, lit_neg]);

    if opts.fmt_res {
        writeln!(
            output,
            "n = {}, SAT = {}",
            ctx.max_skeletons(),
            result.is_ok()
        )
        .expect("写入失败");
        for word in ctx.words() {
            writeln!(output, "word {} => {}", word.0, word.1).unwrap();
        }
        if let Ok(ref model) = result {
            for v in vars
                .iter()
                .filter(|(_, l)| model.value(l.clone()) == true)
                .filter_map(|(v, _)| match v {
                    Variable::Run(_, _, _) => None,
                    Variable::Phantom(_) => None,
                    Variable::Exactly(_) => None,
                    s => Some(s),
                })
            {
                writeln!(output, "{:?} = true", v).expect("写入失败");
            }
        }
    }

    // 没有结果
    if !result.is_ok() {
        eprintln!("SAT 求解不可满足，n = {:?}", ctx.max_skeletons());
        return Err(());
    } else {
        eprintln!("SAT 求解可满足，n = {:?}", ctx.max_skeletons());
    }

    // 生成语法树
    let model = result.unwrap();
    let pos_vars: Vec<Variable> = vars
        .iter()
        .filter(|(_, l)| model.value(l.clone()) == true)
        .map(|(v, _)| v.clone())
        .collect();
    let model = ltl::Model::new(&ctx, &pos_vars);
    let ltl = model.make_ltl(0);

    if opts.fmt_ltl() || opts.fmt_both {
        writeln!(output, "{}", ltl).expect("写入失败");
    }

    if opts.fmt_tuple || opts.fmt_both {
        writeln!(output, "{}", ltl::TupleLtlNode(&ltl)).expect("写入失败");
    }

    Ok(())
}

fn main() -> Result<(), &'static str> {
    let opts: Opts = Opts::parse();

    // 读取输入
    let input =
        serde_json::from_reader::<Box<dyn std::io::Read>, context::Input>(if opts.input != "-" {
            Box::new(std::fs::File::open(opts.input.as_str()).expect("无法打开输入文件"))
        } else {
            Box::new(std::io::stdin())
        })
        .expect("无法解析 JSON");
    // 打开输出文件
    let mut output: Box<dyn std::io::Write> = if opts.output != "-" {
        Box::new(std::fs::File::create(opts.output.as_str()).expect("无法打开输出文件"))
    } else {
        Box::new(std::io::stdout())
    };

    // 解析输入
    let mut ctx: context::Context = input.into();

    for n in 2..=opts.size {
        ctx.set_size_bound(n);
        if solve_iter(&ctx, &opts, &mut output).is_ok() {
            return Ok(());
        }
    }

    Err("无法在给定限制内求解")
}
