//! 将 CNF 形式的逻辑表达式转换为 SAT 求解器所需的格式

use crate::bool_logic::{BinaryOp, PropExpr, UnaryOp, Variable};
use std::collections::HashMap;
use varisat::{CnfFormula, ExtendFormula, Lit, Var};

#[derive(Debug)]
pub struct SATConverter {
    pub vars: HashMap<Variable, Var>, // TODO: pub for debug use
    pub formula: CnfFormula,          // TODO: pub for debug use
}

impl SATConverter {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            formula: CnfFormula::new(),
        }
    }
}

impl SATConverter {
    /// 获取变量
    pub fn get_var(&mut self, v: &Variable) -> &Var {
        if !self.vars.contains_key(v) {
            let r = self.formula.new_var();
            self.vars.insert(v.clone(), r);
        }
        self.vars.get(v).unwrap()
    }

    /// 添加合取字句，要求字句必须是析取字句的合取，可以通过 `sat::covert_cnf` 获得。
    pub fn add_clause(&mut self, expr: PropExpr) {
        if let PropExpr::ChainedBinary(BinaryOp::Conjunction, clauses) = expr {
            // 解析到子句
            for clause in clauses.into_iter() {
                if let PropExpr::ChainedBinary(BinaryOp::Disjunction, vars) = clause.clone() {
                    // 构造变量
                    let lits: Vec<Lit> = vars
                        .into_iter()
                        .map(|v| match v {
                            PropExpr::Unary(UnaryOp::Negation, e) => match *e {
                                PropExpr::Variable(v) => self.get_var(&v).negative(),
                                _ => panic!("input has nested expr inside negation"),
                            },
                            PropExpr::Variable(v) => self.get_var(&v).positive(),
                            _ => panic!("input has nested structure"),
                        })
                        .collect();
                    if lits.len() > 0 {
                        println!("{:?}\n{:?}", &clause, &lits);
                        self.formula.add_clause(&lits);
                    }
                } else {
                    panic!("input is not a conjunction of disjunction")
                }
            }
        } else {
            panic!("input is not a conjunction")
        }
    }

    /// 结束构建，返回 `CnfFormula`
    pub fn finish(self) -> CnfFormula {
        self.formula
    }
}
