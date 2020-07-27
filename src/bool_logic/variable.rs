#[derive(Eq, PartialEq, Copy, Clone, Hash)]
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
    /// Literal `.0` is word (positive? `.2`)`.1`
    Word(usize, usize, bool),
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

impl std::fmt::Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::And(s) => f.write_fmt(format_args!("AND({})", s)),
            Variable::Or(s) => f.write_fmt(format_args!("OR({})", s)),
            Variable::Next(s) => f.write_fmt(format_args!("NEXT({})", s)),
            Variable::WNext(s) => f.write_fmt(format_args!("WNEXT({})", s)),
            Variable::Until(s) => f.write_fmt(format_args!("UNTIL({})", s)),
            Variable::Release(s) => f.write_fmt(format_args!("RELEASE({})", s)),
            Variable::Eventually(s) => f.write_fmt(format_args!("EVENTUALLY({})", s)),
            Variable::Always(s) => f.write_fmt(format_args!("ALWAYS({})", s)),
            Variable::Literal(s) => f.write_fmt(format_args!("LIT({})", s)),
            Variable::Run(e, t, s) => f.write_fmt(format_args!("RUN({}, {}, {})", e, t, s)),
            Variable::LeftChild(s, s1) => f.write_fmt(format_args!("A({}, {})", s, s1)),
            Variable::RightChild(s, s1) => f.write_fmt(format_args!("B({}, {})", s, s1)),
            Variable::Word(s, v, p) => f.write_fmt(format_args!(
                "L({}, {}{})",
                s,
                if *p { "+" } else { "-" },
                v
            )),
            Variable::Exactly(v) => f.write_fmt(format_args!("{}", v)),
        }
    }
}
