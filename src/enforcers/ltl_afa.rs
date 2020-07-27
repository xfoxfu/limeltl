use super::Enforcer;
use crate::bool_logic::{PropExpr, Variable};
use crate::context::Context;

/// 保证 AFA 结构能够生成 LTL_f 公式，检查其存在符合要求的子树
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LTLSubtreeEnforcer(Variable);

impl LTLSubtreeEnforcer {
    /// 构造 LTL Enforcer
    ///
    /// `ty` - 当前节点类型变量
    ///
    /// `n` - 总节点数
    ///
    /// `vars` - 样例中变量数量
    pub fn new(ty: Variable) -> Self {
        Self(ty)
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
                    std::iter::once(f(i, j).into())
                        .chain(range.clone().filter(|v| *v != j).map(|k| !f(i, k)))
                        .collect(),
                )
            })
            .collect();
        PropExpr::biconditional(self.0.into(), PropExpr::chained_or(vars))
    }
}

impl Enforcer for LTLSubtreeEnforcer {
    fn rules(&self, ctx: &Context) -> Vec<PropExpr> {
        let mut rules = Vec::new();
        let ty = &self.0;
        let n = ctx.max_skeletons();
        let word_cnt = ctx.word_count();
        // has left subtree
        if ty.is_unary() || ty.is_binary() {
            rules.push(self.subtree_rule((self.0.skeleton_id() + 1)..n, Variable::LeftChild));
        }
        // has right subtree
        if ty.is_binary() {
            rules.push(self.subtree_rule((self.0.skeleton_id() + 1)..n, Variable::RightChild));
        }
        // is literal
        if ty.is_atom() {
            debug_assert!(if let Variable::Literal(_) = self.0 {
                true
            } else {
                false
            });

            rules.push(self.subtree_rule(0..word_cnt, Variable::Word));
        }
        rules
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn has_left_subtree() {
        let ctx = Context::with_bound(4);
        let rules =
            PropExpr::chained_and(LTLSubtreeEnforcer::new(Variable::Eventually(2)).rules(&ctx));
        assert!(!rules._validate(&vec![Variable::Eventually(2)]));
        assert!(!rules._validate(&vec![Variable::Eventually(2), Variable::LeftChild(2, 1)]));
        assert!(rules._validate(&vec![Variable::Eventually(2), Variable::LeftChild(2, 3)]));
    }
}
