use super::Enforcer;
use crate::bool_logic::{PropExpr, Variable};
use crate::context::Context;

/// 保证 AFA 结构能够生成 LTL_f 公式，检查其在 `N - 2` 以后没有二目结构
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LTLSizeEnforcer(usize);

impl LTLSizeEnforcer {
    /// `n` - 总节点数
    pub fn new(n: usize) -> Self {
        Self(n)
    }
}

impl Enforcer for LTLSizeEnforcer {
    fn rules(&self, ctx: &Context) -> Vec<PropExpr> {
        ((self.0 - 2)..=self.0)
            .flat_map(|i| {
                (&[
                    Variable::And,
                    Variable::Or,
                    Variable::Until,
                    Variable::Release,
                ])
                    .into_iter()
                    .map(move |f| !f(i.clone()))
            })
            .chain(((self.0 - 1)..=self.0).flat_map(|i| {
                (&[
                    Variable::Eventually,
                    Variable::Next,
                    Variable::WNext,
                    Variable::Always,
                ])
                    .into_iter()
                    .map(move |f| !f(i.clone()))
            }))
            .collect()
    }
}

// TODO: tests
