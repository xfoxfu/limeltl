#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Variable {
    /// skeleton `.0` is logic `a ^ b`
    And(usize),
    /// skeleton `.0` is logic `a v b`
    Or(usize),
    /// skeleton `.0` is logic `X a`
    Next(usize),
    /// skeleton `.0` is logic `N a`
    WNext(usize),
    /// skeleton `.0` is logic `a U b`
    Until(usize),
    /// skeleton `.0` is logic `a R b`
    Release(usize),
    /// skeleton `.0` is logic `F a`
    Eventually(usize),
    /// skeleton `.0` is logic `G a`
    Always(usize),
    /// skeleton `.0` is logic `p`
    Literal(usize),
    /// Run(e, t, s)
    Run(usize, usize, usize),
    /// `.0` has left child `.1`
    LeftChild(usize, usize),
    /// `.0` has right child `.1`
    RightChild(usize, usize),
    /// Literal `.0` is word `.1`
    Word(usize, usize),
    /// Exactly `true` or `false`
    Exactly(bool),
}

impl Variable {
    pub fn is_atom(&self) -> bool {
        if let Variable::Literal(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_unary(&self) -> bool {
        match self {
            Variable::Eventually(_)
            | Variable::Next(_)
            | Variable::WNext(_)
            | Variable::Always(_) => true,
            _ => false,
        }
    }
    pub fn is_binary(&self) -> bool {
        match self {
            Variable::And(_) | Variable::Or(_) | Variable::Until(_) | Variable::Release(_) => true,
            _ => false,
        }
    }
    pub fn skeleton_id(&self) -> usize {
        match self {
            Variable::And(id)
            | Variable::Or(id)
            | Variable::Next(id)
            | Variable::WNext(id)
            | Variable::Until(id)
            | Variable::Release(id)
            | Variable::Eventually(id)
            | Variable::Always(id)
            | Variable::Literal(id) => id.to_owned(),
            _ => panic!("variable {:?} is not skeleton", self),
        }
    }
}
