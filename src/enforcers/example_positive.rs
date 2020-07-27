use super::Enforcer;
use crate::bool_logic::{PropExpr, Variable};
use crate::context::{Context, Example};

fn make_rule(
    ctx: &Context,
    ex: &Example,
    ty: Variable,
    s1: usize,
    s2: usize,
    t: usize,
) -> Vec<PropExpr> {
    let e = ex.id();
    let max_e = ctx.example_count();
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
        Variable::Literal(s) => (0..ctx.word_count())
            .map(|v| {
                if !ex.contains_at(t, v) {
                    Exactly(false) << (Run(e, t, s) & Literal(s) & Word(s, v))
                } else {
                    Exactly(false) << (Run(e, t, s) & Literal(s) & !Word(s, v))
                }
            })
            .collect(),
        _ => unreachable!(), // 无其它规则
    }
}

/// 确保给定类型的子树的正例规则
#[derive(Debug, Copy, Clone)]
pub struct PositiveExampleEnforcer<'a>(Variable, &'a Example);

impl<'a> PositiveExampleEnforcer<'a> {
    /// 构造正例的 Enforcer
    pub fn new(ty: Variable, ex: &'a Example) -> Self {
        Self(ty, ex)
    }
}

impl<'a> Enforcer for PositiveExampleEnforcer<'a> {
    fn rules(&self, ctx: &Context) -> Vec<PropExpr> {
        let mut ret = vec![];
        for s1 in (self.0.skeleton_id() + 1)..ctx.max_skeletons() {
            for s2 in (self.0.skeleton_id() + 2)..ctx.max_skeletons() {
                for t in 0..(self.1.size()) {
                    ret.append(&mut make_rule(ctx, self.1, self.0, s1, s2, t));
                }
            }
        }
        ret
    }
}

// TODO: tests
