use super::Variable;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PropExpr {
    /// unary expression
    Unary(UnaryOp, Box<PropExpr>),
    /// binary expression
    Binary(Box<PropExpr>, BinaryOp, Box<PropExpr>),
    /// chained binary expression
    ChainedBinary(BinaryOp, Vec<PropExpr>),
    /// boolean variable
    Variable(Variable),
}

impl PropExpr {
    pub fn not(expr: PropExpr) -> Self {
        PropExpr::Unary(UnaryOp::Negation, Box::new(expr))
    }

    pub fn and(lhs: PropExpr, rhs: PropExpr) -> Self {
        PropExpr::Binary(Box::new(lhs), BinaryOp::Conjunction, Box::new(rhs))
    }
    pub fn or(lhs: PropExpr, rhs: PropExpr) -> Self {
        PropExpr::Binary(Box::new(lhs), BinaryOp::Disjunction, Box::new(rhs))
    }
    pub fn implication(lhs: PropExpr, rhs: PropExpr) -> Self {
        PropExpr::Binary(Box::new(lhs), BinaryOp::MaterialImplication, Box::new(rhs))
    }
    pub fn converse_implication(lhs: PropExpr, rhs: PropExpr) -> Self {
        PropExpr::Binary(Box::new(lhs), BinaryOp::ConverseImplication, Box::new(rhs))
    }
    pub fn biconditional(lhs: PropExpr, rhs: PropExpr) -> Self {
        PropExpr::Binary(Box::new(lhs), BinaryOp::BiConditional, Box::new(rhs))
    }
    pub fn var(var: Variable) -> Self {
        PropExpr::Variable(var)
    }

    pub fn chained_and(exprs: Vec<PropExpr>) -> Self {
        if exprs.len() == 0 {
            panic!("empty list of sub exprs");
        }
        PropExpr::ChainedBinary(BinaryOp::Conjunction, exprs)
    }
    pub fn chained_or(exprs: Vec<PropExpr>) -> Self {
        if exprs.len() == 0 {
            panic!("empty list of sub exprs");
        }
        PropExpr::ChainedBinary(BinaryOp::Disjunction, exprs)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum UnaryOp {
    /// `!a` invertion of expression
    Negation,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BinaryOp {
    /// `a ^ b` conjuction of two expression
    Conjunction,
    /// `a v b` disjunction of two expression
    Disjunction,
    /// `p -> q`
    MaterialImplication,
    /// `p <- q`
    ConverseImplication,
    /// `p <-> q`
    BiConditional,
}

impl Into<PropExpr> for Variable {
    fn into(self) -> PropExpr {
        PropExpr::var(self)
    }
}

#[cfg(test)]
impl PropExpr {
    pub fn _validate(&self, model: &[Variable]) -> bool {
        match self {
            PropExpr::Unary(op, expr) => match op {
                UnaryOp::Negation => !expr._validate(model),
            },
            PropExpr::Binary(lhs, op, rhs) => match op {
                BinaryOp::Conjunction => lhs._validate(model) && rhs._validate(model),
                BinaryOp::Disjunction => lhs._validate(model) || rhs._validate(model),
                BinaryOp::MaterialImplication => !lhs._validate(model) || rhs._validate(model),
                BinaryOp::ConverseImplication => lhs._validate(model) || !rhs._validate(model),
                BinaryOp::BiConditional => lhs._validate(model) == rhs._validate(model),
            },
            PropExpr::ChainedBinary(op, exprs) => {
                let mut ret = match op {
                    BinaryOp::Conjunction => true,
                    BinaryOp::Disjunction => false,
                    _ => unreachable!(),
                };
                for expr in exprs.iter() {
                    match op {
                        BinaryOp::Conjunction => ret = ret && expr._validate(model),
                        BinaryOp::Disjunction => ret = ret || expr._validate(model),
                        _ => unreachable!(),
                    }
                }
                ret
            }
            PropExpr::Variable(var) => model.contains(var),
        }
    }
}

macro_rules! make_op {
    ($trait: ident, $tfn: ident, $fn: ident) => {
        impl std::ops::$trait for PropExpr {
            type Output = PropExpr;
            fn $tfn(self, rhs: Self) -> Self::Output {
                PropExpr::$fn(self, rhs)
            }
        }
        impl std::ops::$trait<Variable> for PropExpr {
            type Output = PropExpr;
            fn $tfn(self, rhs: Variable) -> Self::Output {
                PropExpr::$fn(self, rhs.into())
            }
        }
        impl std::ops::$trait for Variable {
            type Output = PropExpr;
            fn $tfn(self, rhs: Self) -> Self::Output {
                PropExpr::$fn(self.into(), rhs.into())
            }
        }
        impl std::ops::$trait<PropExpr> for Variable {
            type Output = PropExpr;
            fn $tfn(self, rhs: PropExpr) -> Self::Output {
                PropExpr::$fn(self.into(), rhs)
            }
        }
    };
}

make_op!(BitAnd, bitand, and);
make_op!(BitOr, bitor, or);
make_op!(Shl, shl, converse_implication);
make_op!(Shr, shr, implication);
impl std::ops::Not for PropExpr {
    type Output = PropExpr;
    fn not(self) -> Self::Output {
        PropExpr::not(self)
    }
}
impl std::ops::Not for Variable {
    type Output = PropExpr;
    fn not(self) -> Self::Output {
        PropExpr::not(self.into())
    }
}
