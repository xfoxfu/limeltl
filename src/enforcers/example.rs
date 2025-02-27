use super::Enforcer;
use crate::bool_logic::{PropExpr, Variable};
use crate::context::{Context, Example};

fn get_maybe_dual(v: Variable, e: &Example) -> Variable {
    if e.is_pos() {
        v
    } else {
        match v {
            Variable::And(s) => Variable::Or(s),
            Variable::Or(s) => Variable::And(s),
            Variable::Next(s) => Variable::WNext(s),
            Variable::WNext(s) => Variable::Next(s),
            Variable::Until(s) => Variable::Release(s),
            Variable::Release(s) => Variable::Until(s),
            Variable::Eventually(s) => Variable::Always(s),
            Variable::Always(s) => Variable::Eventually(s),
            Variable::Literal(s) => Variable::Literal(s),
            _ => unreachable!(),
        }
    }
}

fn make_rule(
    ctx: &Context,
    ex: &Example,
    ty: Variable,
    s1: usize,
    s2: usize,
    t: usize,
) -> Vec<PropExpr> {
    let e = ex.id();
    let e_max_t = ex.size() - 1;
    use Variable::*;
    match ty {
        Variable::And(s) => {
            if t == e_max_t {
                vec![]
            } else {
                vec![
                    Run(e, t, s1) << (Run(e, t, s) & get_maybe_dual(And(s), ex) & LeftChild(s, s1)),
                    Run(e, t, s2)
                        << (Run(e, t, s) & get_maybe_dual(And(s), ex) & RightChild(s, s2)),
                ]
            }
        }
        Variable::Or(s) => vec![
            (Run(e, t, s1) | Run(e, t, s2))
                << (Run(e, t, s)
                    & get_maybe_dual(Or(s), ex)
                    & LeftChild(s, s1)
                    & RightChild(s, s2)),
        ],
        Variable::Next(s) => vec![
            (if t < e_max_t {
                Run(e, t + 1, s1)
            } else {
                Exactly(false)
            }) << (Run(e, t, s) & get_maybe_dual(Next(s), ex) & LeftChild(s, s1)),
        ],
        Variable::WNext(s) => vec![
            (if t < e_max_t {
                Run(e, t + 1, s1)
            } else {
                Exactly(true)
            }) << (Run(e, t, s) & get_maybe_dual(WNext(s), ex) & LeftChild(s, s1)),
        ],
        Variable::Until(s) => vec![if t < e_max_t {
            (Run(e, t, s2) | (Run(e, t + 1, s) & Run(e, t, s1)))
                << (Run(e, t, s)
                    & get_maybe_dual(Until(s), ex)
                    & LeftChild(s, s1)
                    & RightChild(s, s2))
        } else {
            Run(e, t, s2) << (Run(e, t, s) & get_maybe_dual(Until(s), ex) & RightChild(s, s2))
        }],
        Variable::Release(s) => std::iter::once(Some(
            Run(e, t, s2) << (Run(e, t, s) & get_maybe_dual(Release(s), ex) & RightChild(s, s2)),
        ))
        .chain(std::iter::once(if t < e_max_t {
            Some(
                (Run(e, t, s1) | Run(e, t + 1, s))
                    << (Run(e, t, s)
                        & get_maybe_dual(Release(s), ex)
                        & LeftChild(s, s1)
                        & RightChild(s, s2)),
            )
        } else {
            None
        }))
        .filter_map(|x| x)
        .collect(),
        Variable::Eventually(s) => vec![
            (if t < e_max_t {
                Run(e, t, s1) | Run(e, t + 1, s)
            } else {
                Run(e, t, s1).into()
            }) << (Run(e, t, s) & get_maybe_dual(Eventually(s), ex) & LeftChild(s, s1)),
        ],
        Variable::Always(s) => std::iter::once(Some(
            Run(e, t, s1) << (Run(e, t, s) & get_maybe_dual(Always(s), ex) & LeftChild(s, s1)),
        ))
        .chain(std::iter::once(if t < e_max_t {
            Some(
                Run(e, t + 1, s)
                    << (Run(e, t, s) & get_maybe_dual(Always(s), ex) & LeftChild(s, s1)),
            )
        } else {
            None
        }))
        .filter_map(|x| x)
        .collect(),
        Variable::Literal(s) => (0..ctx.word_count())
            .map(|v| {
                // P C P^C
                // 0 0   0
                // 0 1   1
                // 1 0   1
                // 1 1   0
                if ex.is_pos() ^ ex.contains_at(t, v) {
                    Exactly(false) << (Run(e, t, s) & Literal(s) & Word(s, v, true))
                } else {
                    Exactly(false) << (Run(e, t, s) & Literal(s) & Word(s, v, false))
                }
            })
            .collect(),
        _ => unreachable!(), // 无其它规则
    }
}

/// 确保给定类型的子树的正例规则
#[derive(Debug, Copy, Clone)]
pub struct ExampleEnforcer<'a>(Variable, &'a Example);

impl<'a> ExampleEnforcer<'a> {
    /// 构造正例的 Enforcer
    pub fn new(ty: Variable, ex: &'a Example) -> Self {
        Self(ty, ex)
    }
}

impl<'a> Enforcer for ExampleEnforcer<'a> {
    fn rules(&self, ctx: &Context) -> Vec<PropExpr> {
        let mut ret = vec![];
        ret.append(&mut vec![Variable::Run(self.1.id(), 0, 0).into()]);
        if self.0.is_atom() {
            for t in 0..(self.1.size()) {
                ret.append(&mut make_rule(
                    ctx, self.1, self.0, 0x7F7F7F7F, 0x7F7F7F7F, t,
                ));
            }
        }
        if self.0.is_unary() {
            for s1 in (self.0.skeleton_id() + 1)..ctx.max_skeletons() {
                for t in 0..(self.1.size()) {
                    ret.append(&mut make_rule(ctx, self.1, self.0, s1, 0x7F7F7F7F, t));
                }
            }
        }
        if self.0.is_binary() {
            for s1 in (self.0.skeleton_id() + 1)..ctx.max_skeletons() {
                for s2 in (self.0.skeleton_id() + 2)..ctx.max_skeletons() {
                    for t in 0..(self.1.size()) {
                        ret.append(&mut make_rule(ctx, self.1, self.0, s1, s2, t));
                    }
                }
            }
        }
        ret
    }
}

// TODO: tests
