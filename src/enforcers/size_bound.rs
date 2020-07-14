use super::Enforcer;
use crate::bool_logic::{PropExpr, Variable};
use crate::context::Context;

/// 避免节点重用
pub struct SizeBoundEnforcer(usize, usize);

impl SizeBoundEnforcer {
    fn new(child: usize, bound: usize) -> Self {
        Self(child, bound)
    }
}

impl Enforcer for SizeBoundEnforcer {
    fn rules(&self, ctx: &Context) -> Vec<PropExpr> {
        let mut ret = vec![];
        for i in 0..self.1 {
            for j in 0..self.1 {
                if i == j || i == self.0 || j == self.0 {
                    continue;
                }
                ret.push(PropExpr::or(
                    PropExpr::var(Variable::LeftChild(i, self.0), true),
                    PropExpr::var(Variable::LeftChild(j, self.0), true),
                ));
                ret.push(PropExpr::or(
                    PropExpr::var(Variable::RightChild(i, self.0), true),
                    PropExpr::var(Variable::RightChild(j, self.0), true),
                ));
                ret.push(PropExpr::or(
                    PropExpr::var(Variable::LeftChild(i, self.0), true),
                    PropExpr::var(Variable::RightChild(j, self.0), true),
                ));
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_reuse() {
        let ctx = Context::new(4);
        let rules = PropExpr::chained_and(SizeBoundEnforcer::new(2, 4).rules(&ctx));
        println!("{:?}", rules);
        assert!(!rules._validate(&vec![Variable::LeftChild(1, 2), Variable::LeftChild(3, 2)]));
        assert!(!rules._validate(&vec![Variable::LeftChild(1, 2), Variable::RightChild(3, 2)]));
        assert!(!rules._validate(&vec![
            Variable::RightChild(1, 2),
            Variable::RightChild(3, 2)
        ]));
        assert!(rules._validate(&vec![Variable::LeftChild(1, 3), Variable::LeftChild(2, 3)]));
        assert!(rules._validate(&vec![Variable::LeftChild(1, 2), Variable::RightChild(1, 3)]));
        assert!(rules._validate(&vec![Variable::RightChild(1, 2)]));
    }
}
