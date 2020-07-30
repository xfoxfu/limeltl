use super::LtlNode;
use crate::bool_logic::Variable;
use crate::context::Context;

pub struct Model<'a, 'b> {
    ctx: &'a Context,
    pos_vars: &'b [Variable],
}

impl<'a, 'b> Model<'a, 'b> {
    pub fn new(ctx: &'a Context, pos_vars: &'b [Variable]) -> Self {
        Self { ctx, pos_vars }
    }
}

impl<'a, 'b> Model<'a, 'b> {
    pub fn make_ltl(&self, id: usize) -> LtlNode {
        make_ltl(self, id)
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
