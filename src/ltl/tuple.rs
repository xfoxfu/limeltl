use super::LtlNode;

pub struct TupleLtlNode<'a>(pub &'a LtlNode);

impl<'a> std::fmt::Display for TupleLtlNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            LtlNode::And(lhs, rhs) => f.write_fmt(format_args!(
                "('&', {}, {})",
                TupleLtlNode(&*lhs),
                TupleLtlNode(&*rhs),
            )),
            LtlNode::Or(lhs, rhs) => f.write_fmt(format_args!(
                "('|', {}, {})",
                TupleLtlNode(&*lhs),
                TupleLtlNode(&*rhs),
            )),
            LtlNode::Next(lhs) => f.write_fmt(format_args!("('X', {})", TupleLtlNode(&*lhs))),
            LtlNode::WNext(lhs) => f.write_fmt(format_args!("('N', {})", TupleLtlNode(&*lhs))),
            LtlNode::Until(lhs, rhs) => f.write_fmt(format_args!(
                "('U', {}, {})",
                TupleLtlNode(&*lhs),
                TupleLtlNode(&*rhs),
            )),
            LtlNode::Release(lhs, rhs) => f.write_fmt(format_args!(
                "('R', {}, {})",
                TupleLtlNode(&*lhs),
                TupleLtlNode(&*rhs),
            )),
            LtlNode::Eventually(lhs) => f.write_fmt(format_args!("('F', {})", TupleLtlNode(&*lhs))),
            LtlNode::Always(lhs) => f.write_fmt(format_args!("('G', {})", TupleLtlNode(&*lhs))),
            LtlNode::Literal(pos, name) => {
                if *pos {
                    f.write_fmt(format_args!("'{}'", name))
                } else {
                    f.write_fmt(format_args!("('!', '{}')", name))
                }
            }
        }
    }
}
