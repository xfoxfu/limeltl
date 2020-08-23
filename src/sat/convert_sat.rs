//! 将 CNF 形式的逻辑表达式转换为 SAT 求解器所需的格式

use crate::bool_logic::{BinaryOp, PropExpr, UnaryOp, Variable};
use minisat::{Bool, Solver};
use std::{collections::HashMap, ops::Not};

#[derive(Debug)]
pub struct SATConverter<'a> {
    pub vars: HashMap<Variable, Bool>, // TODO: pub for debug use
    // pub formula: CnfFormula,          // TODO: pub for debug use
    solver: &'a mut Solver,
}

impl<'a> SATConverter<'a> {
    pub fn new(solver: &'a mut Solver) -> Self {
        Self {
            vars: HashMap::new(),
            solver,
        }
    }
}

impl<'a> SATConverter<'a> {
    /// 获取变量
    pub fn get_var(&mut self, v: &Variable) -> &Bool {
        if !self.vars.contains_key(v) {
            let r = self.solver.new_lit();
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
                    let lits: Vec<Bool> = vars
                        .into_iter()
                        .map(|v| match v {
                            PropExpr::Unary(UnaryOp::Negation, e) => match *e {
                                PropExpr::Variable(v) => self.get_var(&v).clone().not(),
                                _ => panic!("input has nested expr inside negation"),
                            },
                            PropExpr::Variable(v) => self.get_var(&v).clone(),
                            _ => panic!("input has nested structure"),
                        })
                        .collect();
                    if lits.len() > 0 {
                        self.solver.add_clause(lits.into_iter());
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
    pub fn finish(self) -> HashMap<Variable, Bool> {
        self.vars
    }
}
