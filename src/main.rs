mod bool_logic;
mod context;
mod enforcers;
mod ltl;
mod options;
mod sat;
mod utils;

use bool_logic::Variable;
use clap::derive::Clap;
use enforcers::{ContextEnforcer, Enforcer};
use options::Opts;
use sat::convert_cnf;
use std::io::Write;

fn main() {
    let opts: Opts = Opts::parse();

    // 读取输入
    let input = serde_json::from_reader::<_, context::Input>(
        std::fs::File::open(opts.input).expect("无法打开输入文件"),
    )
    .expect("无法解析 JSON");
    // 打开输出文件
    let mut output: Box<dyn std::io::Write> = if opts.output != "-" {
        Box::new(std::fs::File::create(opts.output).expect("无法打开输出文件"))
    } else {
        Box::new(std::io::stdout())
    };

    // 解析输入
    let mut ctx: context::Context = input.into();

    // 构造规则
    // TODO: 使用逐次迭代 n
    ctx.set_size_bound(opts.size);
    let rules = ContextEnforcer.rules(&ctx);

    if opts.fmt_expr {
        for rule in rules {
            writeln!(output, "{:?}", rule).expect("写入规则失败");
        }
        return;
    }

    // 生成 CNF
    let mut conv = sat::SATConverter::new();
    for rule in rules.into_iter() {
        let rule_cnf = convert_cnf(rule);
        conv.add_clause(rule_cnf);
    }
    let lit_pos = conv.get_var(&Variable::Exactly(true)).positive();
    let lit_neg = conv.get_var(&Variable::Exactly(false)).negative();
    let (vars, formula) = conv.finish();

    if opts.fmt_cnf {
        varisat::dimacs::write_dimacs(&mut output, &formula).expect("写入规则失败");
        return;
    }

    // 求解
    let mut solver = varisat::Solver::new();
    solver.add_formula(&formula);
    solver.assume(&[lit_pos, lit_neg]);
    let result = solver.solve().expect("无法进行求解");

    if opts.fmt_res {
        writeln!(output, "n = {}, SAT = {}", opts.size, result).expect("写入失败");
        for word in ctx.words() {
            println!("word {} => {}", word.0, word.1);
        }
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
                writeln!(output, "{:?} = true", v).expect("写入失败");
            }
        }
        return;
    }

    // 没有结果
    if !result {
        eprintln!("SAT 求解不可满足，max_n = {:?}", opts.size);
        return;
    }

    // 生成语法树
    let pos_vars: Vec<Variable> = vars
        .iter()
        .filter(|(_, l)| solver.model().unwrap().contains(&l.positive()))
        .map(|(v, _)| v.clone())
        .collect();
    let model = ltl::Model::new(&ctx, &pos_vars);
    let ltl = model.make_ltl(0);

    if !opts.fmt_tuple {
        writeln!(output, "{}", ltl).expect("写入失败");
        return;
    }

    writeln!(output, "{}", ltl::TupleLtlNode(&ltl)).expect("写入失败");
}
