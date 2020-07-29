//! LTL 公式

use crate::bool_logic::Variable;
use crate::context::Context;

#[derive(Debug, Eq, PartialEq)]
pub enum LtlNode {
    /// `a ^ b`
    And(Box<LtlNode>, Box<LtlNode>),
    /// `a v b`
    Or(Box<LtlNode>, Box<LtlNode>),
    /// `X a`
    Next(Box<LtlNode>),
    /// `N a`
    WNext(Box<LtlNode>),
    /// `a U b`
    Until(Box<LtlNode>, Box<LtlNode>),
    /// `a R b`
    Release(Box<LtlNode>, Box<LtlNode>),
    /// `F a`
    Eventually(Box<LtlNode>),
    /// `G a`
    Always(Box<LtlNode>),
    /// (`!`?) `p`
    Literal(bool, String),
}

impl std::fmt::Display for LtlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LtlNode::And(lhs, rhs) => f.write_fmt(format_args!("({} & {})", lhs, rhs)),
            LtlNode::Or(lhs, rhs) => f.write_fmt(format_args!("({} | {})", lhs, rhs)),
            LtlNode::Next(lhs) => f.write_fmt(format_args!("(X {})", lhs)),
            LtlNode::WNext(lhs) => f.write_fmt(format_args!("(N {})", lhs)),
            LtlNode::Until(lhs, rhs) => f.write_fmt(format_args!("({} U {})", lhs, rhs)),
            LtlNode::Release(lhs, rhs) => f.write_fmt(format_args!("({} R {})", lhs, rhs)),
            LtlNode::Eventually(lhs) => f.write_fmt(format_args!("(F {})", lhs)),
            LtlNode::Always(lhs) => f.write_fmt(format_args!("(G {})", lhs)),
            LtlNode::Literal(pos, name) => {
                if *pos {
                    f.write_fmt(format_args!("({})", name))
                } else {
                    f.write_fmt(format_args!("(!({}))", name))
                }
            }
        }
    }
}

pub struct Model<'a, 'b> {
    ctx: &'a Context,
    pos_vars: &'b [Variable],
}

impl<'a, 'b> Model<'a, 'b> {
    pub fn new(ctx: &'a Context, pos_vars: &'b [Variable]) -> Self {
        Self { ctx, pos_vars }
    }
}

pub fn make_ltl(model: &Model, id: usize) -> LtlNode {
    let sk_type = model
        .pos_vars
        .iter()
        .find(|v| (v.is_atom() || v.is_unary() || v.is_binary()) && v.skeleton_id() == id)
        .expect("求解结果不正确，缺少节点类型信息");
    let left = model
        .pos_vars
        .iter()
        .filter_map(|v| match v {
            Variable::LeftChild(s, s1) if *s == id => Some(*s1),
            _ => None,
        })
        .next();
    let right = model
        .pos_vars
        .iter()
        .filter_map(|v| match v {
            Variable::RightChild(s, s1) if *s == id => Some(*s1),
            _ => None,
        })
        .next();
    match sk_type {
        Variable::And(_) => LtlNode::And(
            Box::new(make_ltl(model, left.expect("未找到子树"))),
            Box::new(make_ltl(model, right.expect("未找到子树"))),
        ),
        Variable::Or(_) => LtlNode::Or(
            Box::new(make_ltl(model, left.expect("未找到子树"))),
            Box::new(make_ltl(model, right.expect("未找到子树"))),
        ),
        Variable::Next(_) => LtlNode::Next(Box::new(make_ltl(model, left.expect("未找到子树")))),
        Variable::WNext(_) => LtlNode::WNext(Box::new(make_ltl(model, left.expect("未找到子树")))),
        Variable::Until(_) => LtlNode::Until(
            Box::new(make_ltl(model, left.expect("未找到子树"))),
            Box::new(make_ltl(model, right.expect("未找到子树"))),
        ),
        Variable::Release(_) => LtlNode::Release(
            Box::new(make_ltl(model, left.expect("未找到子树"))),
            Box::new(make_ltl(model, right.expect("未找到子树"))),
        ),
        Variable::Eventually(_) => {
            LtlNode::Eventually(Box::new(make_ltl(model, left.expect("未找到子树"))))
        }
        Variable::Always(_) => {
            LtlNode::Always(Box::new(make_ltl(model, left.expect("未找到子树"))))
        }
        Variable::Literal(_) => {
            let lit = model
                .pos_vars
                .iter()
                .filter_map(|v| match v {
                    Variable::Word(s, w, p) if *s == id => Some((*p, *w)),
                    _ => None,
                })
                .next()
                .expect("未找到字面量信息");
            LtlNode::Literal(
                lit.0,
                model
                    .ctx
                    .words()
                    .iter()
                    .filter_map(|(k, v)| if *v == lit.1 { Some(k) } else { None })
                    .next()
                    .expect("意外的变量")
                    .clone(),
            )
        }
        _ => unreachable!(),
    }
}
