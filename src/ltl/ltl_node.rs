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
