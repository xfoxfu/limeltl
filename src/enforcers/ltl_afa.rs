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
}

impl Enforcer for LTLSubtreeEnforcer {
    fn rules(&self, ctx: &Context) -> Vec<PropExpr> {
        let mut rules = Vec::new();
        let ty = &self.0;
        let n = ctx.max_skeletons();
        let word_cnt = ctx.word_count();
        // has left subtree
        if ty.is_unary() {
            rules.push(
                self.0
                    >> super::one_of(
                        ((self.0.skeleton_id() + 1)..n)
                            .map(|s1| Variable::LeftChild(self.0.skeleton_id(), s1)),
                    ),
            );
        }
        // has right subtree
        if ty.is_binary() {
            rules.push(
                self.0
                    >> super::one_of(
                        ((self.0.skeleton_id() + 1)..(n - 1))
                            .map(|s1| Variable::LeftChild(self.0.skeleton_id(), s1)),
                    ),
            );
            rules.push(
                self.0
                    >> super::one_of(
                        ((self.0.skeleton_id() + 2)..n)
                            .map(|s2| Variable::RightChild(self.0.skeleton_id(), s2)),
                    ),
            );
        }
        // is literal
        if ty.is_atom() {
            debug_assert!(if let Variable::Literal(_) = self.0 {
                true
            } else {
                false
            });

            // TODO: add negative literal word
            rules.push({
                let range = 0..word_cnt;
                let i = self.0.skeleton_id();
                use itertools::Itertools;

                self.0
                    >> super::one_of(
                        [true, false]
                            .iter()
                            .cartesian_product(range)
                            .map(|(p, w)| Variable::Word(i, w, *p)),
                    )
            });
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
