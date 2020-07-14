use super::Enforcer;
use crate::bool_logic::{PropExpr, Variable};
use crate::context::Context;

/// 保证 AFA 结构能够生成 LTL_f 公式，检查其存在符合要求的子树
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LTLSubtreeEnforcer(Variable, usize, usize);

impl LTLSubtreeEnforcer {
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

    fn subtree_rule<I, F>(&self, range: I, f: F) -> PropExpr
    where
        I: Iterator<Item = usize> + Clone,
        F: Fn(usize, usize) -> Variable,
    {
        let i = self.0.skeleton_id();
        let vars = range
            .clone()
            .map(|j| {
                PropExpr::chained_and(
                    std::iter::once(PropExpr::var(f(i, j), false))
                        .chain(
                            range
                                .clone()
                                .filter(|v| *v != j)
                                .map(|k| PropExpr::var(f(i, k), true)),
                        )
                        .collect(),
                )
            })
            .collect();
        PropExpr::biconditional(PropExpr::var(self.0, false), PropExpr::chained_or(vars))
    }
}

impl Enforcer for LTLSubtreeEnforcer {
    fn rules(&self, ctx: &Context) -> Vec<PropExpr> {
        let mut rules = Vec::new();
        let ty = &self.0;
        // has left subtree
        if ty.is_unary() || ty.is_binary() {
            rules.push(self.subtree_rule((self.0.skeleton_id() + 1)..self.1, Variable::LeftChild));
        }
        // has right subtree
        if ty.is_binary() {
            rules.push(self.subtree_rule((self.0.skeleton_id() + 1)..self.1, Variable::RightChild));
        }
        // is literal
        if ty.is_atom() {
            debug_assert!(if let Variable::Literal(_) = self.0 {
                true
            } else {
                false
            });

            rules.push(self.subtree_rule(0..self.2, Variable::LiteralValue));
        }
        rules
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn has_left_subtree() {
        let ctx = Context::new(4);
        let rules = PropExpr::chained_and(
            LTLSubtreeEnforcer::new(Variable::Eventually(2), 4, 2).rules(&ctx),
        );
        assert!(!rules._validate(&vec![Variable::Eventually(2)]));
        assert!(!rules._validate(&vec![Variable::Eventually(2), Variable::LeftChild(2, 1)]));
        assert!(rules._validate(&vec![Variable::Eventually(2), Variable::LeftChild(2, 3)]));
    }
}
