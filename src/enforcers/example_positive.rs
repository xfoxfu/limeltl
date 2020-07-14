use super::Enforcer;
use crate::bool_logic::{PropExpr, Variable};
use crate::context::{Context, Example};

fn make_rule<F: Fn() -> Variable>(
    ex: Example,
    ty: Variable,
    s1: usize,
    s2: usize,
    t: usize,
    max_e: usize,
) -> Vec<PropExpr> {
    let e = ex.id();
    use Variable::*;
    match ty {
        Variable::And(s) => {
            if t == max_e {
                vec![]
            } else {
                vec![
                    Run(e, t, s) << (Run(e, t, s) & Variable::And(s) & LeftChild(s, s1)),
                    Run(e, t, s2) << (Run(e, t, s) & Variable::And(s) & RightChild(s, s2)),
                ]
            }
        }
        Variable::Or(s) => vec![
            (Run(e, t, s1) | Run(e, t, s2))
                << (Run(e, t, s) & Or(s) & LeftChild(s, s1) & RightChild(s, s2)),
        ],
        Variable::Next(s) => vec![
            (if t < max_e {
                Run(e, t + 1, s)
            } else {
                Exactly(false)
            }) << (Run(e, t, s) & Next(s) & LeftChild(s, s1)),
        ],
        Variable::WNext(s) => vec![
            (if t < max_e {
                Run(e, t + 1, s)
            } else {
                Exactly(true)
            }) << (Run(e, t, s) & WNext(s) & LeftChild(s, s1)),
        ],
        Variable::Until(s) => vec![if t < max_e {
            (Run(e, t, s2) | (Run(e, t + 1, s) & Run(e, t, s1)))
                << (Run(e, t, s) & Until(s) & LeftChild(s, s1) & RightChild(s, s2))
        } else {
            Run(e, t, s2) << (Run(e, t, s) & Until(s) & RightChild(s, s2))
        }],
        Variable::Release(s) => std::iter::once(Some(
            Run(e, t, s2) << (Run(e, t, s) & Release(s) & RightChild(s, s2)),
        ))
        .chain(std::iter::once(if t < max_e {
            Some(
                (Run(e, t, s2) | Run(e, t + 1, s))
                    << (Run(e, t, s) & Release(s) & LeftChild(s, s1) & RightChild(s, s2)),
            )
        } else {
            None
        }))
        .filter_map(|x| x)
        .collect(),
        Variable::Eventually(s) => vec![
            (if t < max_e {
                Run(e, t, s1) | Run(e, t + 1, s)
            } else {
                Run(e, t, s1).into()
            }) << (Run(e, t, s) & Eventually(s) & LeftChild(s, s1)),
        ],
        Variable::Always(s) => std::iter::once(Some(
            Run(e, t, s1) << (Run(e, t, s) & Always(s) & LeftChild(s, s1)),
        ))
        .chain(std::iter::once(if t < max_e {
            Some(Run(e, t + 1, s) << (Run(e, t, s) & Always(s) & LeftChild(s, s1)))
        } else {
            None
        }))
        .filter_map(|x| x)
        .collect(),
        Variable::Literal(s) => (0..ex.context().var_count())
            .map(|v| {
                if !ex.contains(v) {
                    Exactly(false) << (Run(e, t, s) & Literal(s) & LiteralValue(s, v))
                } else {
                    Exactly(false) << (Run(e, t, s) & Literal(s) & !LiteralValue(s, v))
                }
            })
            .collect(),
        _ => unreachable!(), // 无其它规则
    }
}

/// 确保给定类型的子树的正例规则
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct PositiveExampleEnforcer(Variable, usize, usize);

impl PositiveExampleEnforcer {
    /// 构造 LTL Enforcer
    ///
    /// `ty` - 当前节点类型变量
    ///
    /// `n` - 总节点数
    ///
    /// `vars` - 样例中变量数量
    pub fn new(ty: Variable, n: usize, vars: usize) -> Self {
        Self(ty, n, vars)
    }
}

impl Enforcer for PositiveExampleEnforcer {
    fn rules(&self, ctx: &Context) -> Vec<PropExpr> {
        unimplemented!()
    }
}

// TODO: tests
