use crate::bool_logic::{BinaryOp, PropExpr, UnaryOp};
use std::sync::atomic::{AtomicUsize, Ordering};

/// 将任意逻辑表达式转换为 CNF 形式
pub fn convert_cnf(val: PropExpr) -> PropExpr {
    PropExpr::chained_and(
        flatten(conv_cnf(elim_not(elim_impl_eq(val))))
            .into_iter()
            .map(PropExpr::chained_or)
            .collect(),
    )
}

/// (1) 消除表达式的推理和等价运算符
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
            (!lhs.clone() | rhs.clone()) & (lhs | !rhs)
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
        PropExpr::Variable(_) => val,
    }
}

/// (2) 消除表达式中非最底层（紧邻变量）的非运算符，从而形成 NNF
fn elim_not(val: PropExpr) -> PropExpr {
    match val {
        // modification case
        PropExpr::Unary(UnaryOp::Negation, rhs) => match *rhs {
            // double negation = value
            PropExpr::Unary(UnaryOp::Negation, rhs) => elim_not(*rhs),
            // apply De Morgan laws
            PropExpr::Binary(lhs, BinaryOp::Conjunction, rhs) => elim_not(!*lhs) | elim_not(!*rhs),
            PropExpr::Binary(lhs, BinaryOp::Disjunction, rhs) => elim_not(!*lhs) & elim_not(!*rhs),
            PropExpr::ChainedBinary(BinaryOp::Conjunction, vals) => PropExpr::ChainedBinary(
                BinaryOp::Disjunction,
                vals.into_iter().map(|v| elim_not(!v)).collect(),
            ),
            PropExpr::ChainedBinary(BinaryOp::Disjunction, vals) => PropExpr::ChainedBinary(
                BinaryOp::Conjunction,
                vals.into_iter().map(|v| elim_not(!v)).collect(),
            ),
            // other binary should be handled before
            PropExpr::Binary(_, _, _) => unreachable!(),
            PropExpr::ChainedBinary(_, _) => unreachable!(),
            // variable
            var @ PropExpr::Variable(_) => !var,
        },
        // recursion case
        #[allow(unreachable_patterns)] // guard this case for safety
        PropExpr::Unary(op, rhs) => match *rhs {
            _ => PropExpr::Unary(op, Box::new(elim_not(*rhs))),
        },
        PropExpr::Binary(lhs, op, rhs) => {
            PropExpr::Binary(Box::new(elim_not(*lhs)), op, Box::new(elim_not(*rhs)))
        }
        PropExpr::ChainedBinary(op, vals) => {
            PropExpr::ChainedBinary(op, vals.into_iter().map(|v| elim_not(v)).collect())
        }
        PropExpr::Variable(_) => val,
    }
}

/// (3) 将表达式转换为 CNF
///
/// 返回值是一系列析取式，彼此之间是合取关系
///
/// see https://www.cs.jhu.edu/~jason/tutorials/convert-to-CNF.html
fn conv_cnf(e: PropExpr) -> Vec<PropExpr> {
    //  CONVERT(φ):   // returns a CNF formula equivalent to φ

    // // Any syntactically valid propositional formula φ must fall into
    // // exactly one of the following 7 cases (that is, it is an instanceof
    // // one of the 7 subclasses of Formula).

    match e {
        // If φ is a variable, then:
        //    return φ.
        //    // this is a CNF formula consisting of 1 clause that contains 1 literal
        v @ PropExpr::Variable(_) => vec![v],
        // If φ has the form P ^ Q, then:
        //    CONVERT(P) must have the form P1 ^ P2 ^ ... ^ Pm, and
        //    CONVERT(Q) must have the form Q1 ^ Q2 ^ ... ^ Qn,
        //    where all the Pi and Qi are disjunctions of literals.
        //    So return P1 ^ P2 ^ ... ^ Pm ^ Q1 ^ Q2 ^ ... ^ Qn.
        PropExpr::Binary(lhs, BinaryOp::Conjunction, rhs) => {
            let mut left = conv_cnf(*lhs);
            let mut right = conv_cnf(*rhs);
            left.append(&mut right);
            left
        }
        PropExpr::ChainedBinary(BinaryOp::Conjunction, vals) => {
            vals.into_iter().flat_map(conv_cnf).collect()
        }
        // If φ has the form P v Q, then:
        //    CONVERT(P) must have the form P1 ^ P2 ^ ... ^ Pm, and
        //    CONVERT(Q) must have the form Q1 ^ Q2 ^ ... ^ Qn,
        //    where all the Pi and Qi are dijunctions of literals.
        //    So we need a CNF formula equivalent to
        //       (P1 ^ P2 ^ ... ^ Pm) v (Q1 ^ Q2 ^ ... ^ Qn).
        //    So return (P1 v Q1) ^ (P1 v Q2) ^ ... ^ (P1 v Qn)
        //            ^ (P2 v Q1) ^ (P2 v Q2) ^ ... ^ (P2 v Qn)
        //              ...
        //            ^ (Pm v Q1) ^ (Pm v Q2) ^ ... ^ (Pm v Qn)
        PropExpr::Binary(lhs, BinaryOp::Disjunction, rhs) => {
            let left = conv_cnf(*lhs);
            let right = conv_cnf(*rhs);
            let mut ret = vec![];

            if left.len() > 1 && right.len() > 1 {
                static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);

                use crate::bool_logic::Variable;
                let id = OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst);
                ret.append(&mut conv_cnf(elim_not(elim_impl_eq(
                    Variable::Phantom(id) >> PropExpr::chained_and(left),
                ))));
                ret.append(&mut conv_cnf(elim_not(elim_impl_eq(
                    !Variable::Phantom(id) >> PropExpr::chained_and(right),
                ))));
            } else {
                for l in left.iter() {
                    for r in right.iter() {
                        ret.push(l.clone() | r.clone());
                    }
                }
            }
            ret
        }
        PropExpr::ChainedBinary(BinaryOp::Disjunction, vals) => {
            if let [f, rest @ ..] = vals.as_slice() {
                if rest.len() > 0 {
                    conv_cnf(PropExpr::or(
                        f.clone(),
                        PropExpr::chained_and(conv_cnf(PropExpr::chained_or(
                            rest.into_iter().cloned().collect(),
                        ))),
                    ))
                } else {
                    vec![f.clone()]
                }
            } else {
                vec![]
            }
        }
        // If φ has the form ~(...), then:
        //   If φ has the form ~A for some variable A, then return φ.
        //   If φ has the form ~(~P), then return CONVERT(P).           // double negation
        //   If φ has the form ~(P ^ Q), then return CONVERT(~P v ~Q).  // de Morgan's Law
        //   If φ has the form ~(P v Q), then return CONVERT(~P ^ ~Q).  // de Morgan's Law
        // 只有情形 (a) 可能在此时出现
        u @ PropExpr::Unary(_, _) => vec![u],
        // If φ has the form P -> Q, then:
        //   Return CONVERT(~P v Q).   // equivalent
        // 这种情形在我们的代码中不可能出现

        // If φ has the form P <-> Q, then:
        //   Return CONVERT((P ^ Q) v (~P ^ ~Q)).
        // 这种情形在我们的代码中不可能出现
        PropExpr::Binary(_, _, _) => unreachable!(),
        PropExpr::ChainedBinary(_, _) => unreachable!(),
    }
}

/// (4) 将表达式的 CNF 进行平整
fn flatten(exprs: Vec<PropExpr>) -> Vec<Vec<PropExpr>> {
    exprs.into_iter().map(flatten_single).collect()
}

/// (4x) 平整单一表达式
fn flatten_single(expr: PropExpr) -> Vec<PropExpr> {
    match expr {
        PropExpr::Binary(lhs, BinaryOp::Disjunction, rhs) => vec![lhs, rhs]
            .into_iter()
            .flat_map(|v| flatten_single(*v))
            .collect(),
        PropExpr::ChainedBinary(BinaryOp::Disjunction, exprs) => {
            exprs.into_iter().flat_map(|v| flatten_single(v)).collect()
        }
        v @ PropExpr::Unary(UnaryOp::Negation, _) => vec![v],
        v @ PropExpr::Variable(_) => vec![v],
        PropExpr::ChainedBinary(_, exprs) => {
            if exprs.len() <= 1 {
                exprs
            } else {
                unreachable!()
            }
        }
        _ => unreachable!("{:?}", expr),
    }
}

#[cfg(test)]
mod test {
    mod elim_impl_eq {
        use super::super::*;
        use crate::bool_logic::Variable::And as V;

        #[test]
        fn plain() {
            // (a) A <- B
            assert_eq!(elim_impl_eq(V(1) << (V(2) | V(3))), V(1) | !(V(2) | V(3)),);
            // (b) A -> B
            assert_eq!(elim_impl_eq(V(1) >> (V(2) | V(3))), !V(1) | (V(2) | V(3)),);
            // (c) A <-> B
            assert_eq!(
                elim_impl_eq(PropExpr::biconditional(V(1).into(), V(2) | V(3))),
                (!V(1) | (V(2) | V(3))) & (V(1) | !(V(2) | V(3))),
            );
        }

        #[test]
        fn nested() {
            // A <- ((B <- C) & (D -> E) & F)
            assert_eq!(
                elim_impl_eq(
                    V(1) << PropExpr::chained_and(vec![V(2) << V(3), V(4) >> V(5), V(6).into()])
                ),
                V(1) | !PropExpr::chained_and(vec![V(2) | !V(3), !V(4) | V(5), V(6).into()])
            );
            // A <- (B | (C -> D))
            assert_eq!(
                elim_impl_eq(V(1) << (V(2) | (V(3) >> V(4)))),
                V(1) | !(V(2) | (!V(3) | V(4))),
            );
        }
    }

    mod elim_not {
        use super::super::*;
        use crate::bool_logic::Variable::And as V;

        #[test]
        fn plain() {
            // (a1) !!A
            assert_eq!(elim_not(!!V(1)), V(1).into());
            // (a2) !(A & B)
            assert_eq!(elim_not(!(V(1) & V(2))), !V(1) | !V(2));
            // (a3) !(A | B)
            assert_eq!(elim_not(!(V(1) | V(2))), !V(1) & !V(2));
            // (a4) !p
            assert_eq!(elim_not(!V(1)), !V(1));
        }

        #[test]
        fn nested() {
            // !(A & (!B & !(C | D)))
            assert_eq!(
                elim_not(!(V(1) & (!V(2) & !(V(3) | V(4))))),
                !V(1) | (V(2) | (V(3) | V(4)))
            );
        }
    }

    mod conv_cnf {
        use super::super::*;
        use crate::bool_logic::Variable::And as V;

        #[test]
        fn plain() {
            // (a) p
            assert_eq!(conv_cnf(V(1).into()), vec![V(1).into()]);
            // (b) A | B
            assert_eq!(conv_cnf(V(1) | V(2)), vec![V(1) | V(2)]);
            assert_eq!(
                conv_cnf(V(1) | (V(2) & V(3))),
                vec![V(1) | V(2), V(1) | V(3)]
            );
            assert_eq!(
                conv_cnf(PropExpr::chained_or(vec![
                    V(1).into(),
                    V(2) & V(3),
                    V(4) | V(5)
                ])),
                vec![V(1) | (V(2) | (V(4) | V(5))), V(1) | (V(3) | (V(4) | V(5)))]
            );
            // (c) A & B
            assert_eq!(
                conv_cnf(V(1) & (V(2) | V(3))),
                vec![V(1).into(), V(2) | V(3)]
            );
            // (d) !p
            assert_eq!(conv_cnf(!V(1)), vec![!V(1)]);
        }

        #[test]
        fn extra() {
            assert_eq!(
                conv_cnf((!V(0) | (!V(1) & !V(2))) & (V(0) | V(1) | V(2))),
                vec![!V(0) | !V(1), !V(0) | !V(2), V(0) | V(1) | V(2)]
            );
        }
    }

    mod flatten {
        use super::super::*;
        use crate::bool_logic::Variable::And as V;

        #[test]
        fn plain() {
            assert_eq!(flatten(vec![]), Vec::<Vec<PropExpr>>::new());
            assert_eq!(
                flatten(vec![V(1) | V(2)]),
                vec![vec![V(1).into(), V(2).into()]]
            );
            assert_eq!(
                flatten(vec![V(1) | V(2) | V(3)]),
                vec![vec![V(1).into(), V(2).into(), V(3).into()]]
            );
            assert_eq!(
                flatten(vec![V(1) | V(2), V(3) | !V(4) | V(5)]),
                vec![
                    vec![V(1).into(), V(2).into()],
                    vec![V(3).into(), !V(4), V(5).into()]
                ]
            );
        }
    }

    mod convert_cnf {
        use super::super::*;
        use crate::bool_logic::Variable::Always as V;

        #[test]
        fn extra() {
            let expr = PropExpr::biconditional(V(0).into(), !V(1) & !V(2));
            assert_eq!(
                elim_impl_eq(expr.clone()),
                ((!V(0) | (!V(1) & !V(2))) & (V(0) | !(!V(1) & !V(2))))
            );
            assert_eq!(
                convert_cnf(expr.clone()),
                PropExpr::chained_and(vec![
                    PropExpr::chained_or(vec![!V(0), !V(1)]),
                    PropExpr::chained_or(vec![!V(0), !V(2)]),
                    PropExpr::chained_or(vec![V(0).into(), V(1).into(), V(2).into()]),
                ])
            );
        }
    }
}
