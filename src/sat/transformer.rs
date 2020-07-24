use crate::bool_logic::{BinaryOp, PropExpr, UnaryOp, Variable};

fn elim_impl_eq(val: PropExpr) -> PropExpr {
    match val {
        // modification case
        PropExpr::Binary(lhs, BinaryOp::ConverseImplication, rhs) => {
            elim_impl_eq(*lhs) | !elim_impl_eq(*rhs)
        }
        PropExpr::Binary(lhs, BinaryOp::MaterialImplication, rhs) => {
            !elim_impl_eq(*lhs) | elim_impl_eq(*rhs)
        }
        PropExpr::Binary(lhs, BinaryOp::BiConditional, rhs) => {
            let lhs = elim_impl_eq(*lhs);
            let rhs = elim_impl_eq(*rhs);
            (lhs.clone() & rhs.clone()) | (!lhs & !rhs)
        }
        // recursion case
        PropExpr::Unary(op, rhs) => match *rhs {
            _ => PropExpr::Unary(op, Box::new(elim_impl_eq(*rhs))),
        },
        PropExpr::Binary(lhs, op, rhs) => PropExpr::Binary(
            Box::new(elim_impl_eq(*lhs)),
            op,
            Box::new(elim_impl_eq(*rhs)),
        ),
        PropExpr::ChainedBinary(op, vals) => {
            PropExpr::ChainedBinary(op, vals.into_iter().map(|v| elim_impl_eq(v)).collect())
        }
        _ => val,
    }
}

fn elim_not(val: PropExpr) -> PropExpr {
    match val {
        // modification case
        PropExpr::Unary(UnaryOp::Negation, rhs) => match *rhs {
            // double negation = value
            PropExpr::Unary(UnaryOp::Negation, rhs) => elim_not(*rhs),
            // apply De Morgan laws
            PropExpr::Binary(lhs, BinaryOp::Conjunction, rhs) => !*lhs | !*rhs,
            PropExpr::Binary(lhs, BinaryOp::Disjunction, rhs) => !*lhs & !*rhs,
            PropExpr::ChainedBinary(BinaryOp::Conjunction, vals) => PropExpr::ChainedBinary(
                BinaryOp::Disjunction,
                vals.into_iter().map(|v| !v).collect(),
            ),
            PropExpr::ChainedBinary(BinaryOp::Disjunction, vals) => PropExpr::ChainedBinary(
                BinaryOp::Conjunction,
                vals.into_iter().map(|v| !v).collect(),
            ),
            // other binary should be handled before
            PropExpr::Binary(_, _, _) => unreachable!(),
            PropExpr::ChainedBinary(_, _) => unreachable!(),
            // variable
            var @ PropExpr::Variable(_) => var,
        },
        // recursion case
        #[allow(unreachable_patterns)] // guard this case for safety
        PropExpr::Unary(op, rhs) => match *rhs {
            _ => PropExpr::Unary(op, Box::new(elim_impl_eq(*rhs))),
        },
        PropExpr::Binary(lhs, op, rhs) => {
            PropExpr::Binary(Box::new(elim_not(*lhs)), op, Box::new(elim_not(*rhs)))
        }
        PropExpr::ChainedBinary(op, vals) => {
            PropExpr::ChainedBinary(op, vals.into_iter().map(|v| elim_not(v)).collect())
        }
        _ => val,
    }
}

#[cfg(test)]
mod test {
    mod elim_impl_eq {
        use super::super::*;

        #[test]
        fn plain() {
            assert_eq!(
                elim_impl_eq(Variable::And(1) << (Variable::And(2) | Variable::And(3)),),
                Variable::And(1) | !(Variable::And(2) | Variable::And(3)),
            );
            assert_eq!(
                elim_impl_eq(Variable::And(1) >> (Variable::And(2) | Variable::And(3)),),
                !Variable::And(1) | (Variable::And(2) | Variable::And(3)),
            );
            assert_eq!(
                elim_impl_eq(PropExpr::biconditional(
                    Variable::And(1).into(),
                    Variable::And(2) | Variable::And(3),
                )),
                (Variable::And(1) & (Variable::And(2) | Variable::And(3)))
                    | (!Variable::And(1) & !(Variable::And(2) | Variable::And(3))),
            );
        }

        #[test]
        fn nested() {
            assert_eq!(
                elim_impl_eq(
                    Variable::And(1) << (Variable::And(2) | (Variable::And(3) << Variable::And(4))),
                ),
                Variable::And(1) | !(Variable::And(2) | (!Variable::And(3) | Variable::And(4))),
            );
        }
    }

    mod elim_not {
        use super::super::*;
    }
}
