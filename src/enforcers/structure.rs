use super::Enforcer;
use crate::bool_logic::{PropExpr, Variable};
use crate::context::Context;

/// 确保求解结果符合 AFA 结构，i.e. 对于每个节点，要求其属于 AFA 节点类型中的一种
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct AFASkTypeEnforcer(usize);

impl AFASkTypeEnforcer {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl Enforcer for AFASkTypeEnforcer {
    fn rules(&self, _ctx: &Context) -> Vec<PropExpr> {
        let id = self.0;
        let vars = vec![
            Variable::And(id),
            Variable::Or(id),
            Variable::Next(id),
            Variable::WNext(id),
            Variable::Until(id),
            Variable::Release(id),
            Variable::Eventually(id),
            Variable::Always(id),
            Variable::Literal(id),
        ];
        let mut sub_routes = vec![];
        // for each possible u
        for u in vars.iter() {
            // create a chained_or where only u is not neg
            // e.g. And & !Or & !Next & ...
            sub_routes.push(PropExpr::chained_and(
                vars.iter()
                    .map(|v| if v == u { v.clone().into() } else { !v.clone() })
                    .collect(),
            ));
        }
        vec![PropExpr::chained_or(sub_routes)]
    }
}

/// 用于确保求解结果符合 AFA 结构，i.e. 对于原子、单目和双目运算符，
/// 确保 LeftChild 和 RightChild 变量符合要求
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct AFASpecificStructureEnforcer(Variable);

impl AFASpecificStructureEnforcer {
    pub fn new(ty: Variable) -> Self {
        Self(ty)
    }
}

impl Enforcer for AFASpecificStructureEnforcer {
    fn rules(&self, ctx: &Context) -> Vec<PropExpr> {
        let mut rules = Vec::new();
        let ty = &self.0;
        if ty.is_binary() {
            return rules;
        }
        let vars = (((self.0).skeleton_id() + 1)..ctx.max_skeletons())
            .map(|v| ((self.0).skeleton_id(), v))
            .map(|(l, r)| (Variable::LeftChild(l, r), Variable::RightChild(l, r)))
            .filter_map(|var| {
                if ty.is_atom() {
                    // for atom skeleton, should not have no subtree
                    Some(PropExpr::and(!var.0, !var.1))
                } else if ty.is_unary() {
                    // ty is unary, should not have right subtree
                    Some(!var.1)
                } else {
                    // ty is binary, where both subtree is possible
                    unreachable!()
                }
            })
            .collect();
        rules.push(PropExpr::biconditional(
            self.0.into(),
            PropExpr::chained_and(vars),
        ));
        rules
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::context::Context;

    #[test]
    fn sk_type() {
        let ctx = Context::with_bound(6);
        let id = 5;
        let rule = AFASkTypeEnforcer::new(id)
            .rules(&ctx)
            .pop()
            .expect("should generate exactly one rule");
        let vars = vec![
            Variable::And(id),
            Variable::Or(id),
            Variable::Next(id),
            Variable::WNext(id),
            Variable::Until(id),
            Variable::Release(id),
            Variable::Eventually(id),
            Variable::Always(id),
            Variable::Literal(id),
        ];
        assert!(!rule._validate(&vec![]), "no assign should fail");
        for v in vars.iter() {
            let model = vec![v.clone()];
            assert!(rule._validate(&model), "single assign should pass");
        }
        for v in vars.iter().skip(1) {
            let model = vec![v.clone(), vars.first().unwrap().clone()];
            assert!(!rule._validate(&model), "double assign should fail");
        }
    }

    #[test]
    fn and() {
        let ctx = Context::with_bound(3);
        assert_eq!(
            AFASpecificStructureEnforcer::new(Variable::And(1))
                .rules(&ctx)
                .len(),
            0,
            "AND do not have constraints"
        )
    }

    #[test]
    fn next() {
        let ctx = Context::with_bound(3);
        let rule = AFASpecificStructureEnforcer::new(Variable::Next(1))
            .rules(&ctx)
            .pop()
            .expect("should have exactly one rule");

        assert!(
            rule._validate(&vec![Variable::Next(1), Variable::LeftChild(1, 2)]),
            "NEXT can have left child"
        );
        assert!(
            !rule._validate(&vec![Variable::Next(1), Variable::RightChild(1, 2)]),
            "NEXT can not have right child"
        );
        assert!(
            rule._validate(&vec![Variable::And(1), Variable::RightChild(1, 2)]),
            "not NEXT can have right child"
        );
    }

    #[test]
    fn lit() {
        let ctx = Context::with_bound(4);
        let rule = AFASpecificStructureEnforcer::new(Variable::Literal(1))
            .rules(&ctx)
            .pop()
            .expect("should have exactly one rule");

        assert!(
            !rule._validate(&vec![Variable::Literal(1), Variable::LeftChild(1, 2)]),
            "LIT can not have left child id=2"
        );
        assert!(
            !rule._validate(&vec![Variable::Literal(1), Variable::RightChild(1, 2)]),
            "LIT can not have right child id=2"
        );
        assert!(
            !rule._validate(&vec![Variable::Literal(1), Variable::LeftChild(1, 3)]),
            "LIT can not have left child id=3"
        );
        assert!(
            !rule._validate(&vec![Variable::Literal(1), Variable::RightChild(1, 3)]),
            "LIT can not have right child id=3"
        );
        assert!(
            rule._validate(&vec![Variable::And(1), Variable::LeftChild(1, 2)]),
            "not LIT can have left child"
        );
    }
}
