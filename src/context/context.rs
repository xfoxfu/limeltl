use crate::bool_logic::PropExpr;

pub struct Context {
    max_skeletons: usize,
}

impl Context {
    pub fn new(max_skeletons: usize) -> Self {
        Self { max_skeletons }
    }
}

impl Context {
    /// Skeleton 的最大数量限制；假定其为 `N`，则可行的 Skeleton 编号为 `0 <= i < N`
    pub fn max_skeletons(&self) -> usize {
        self.max_skeletons
    }
    /// 例子中变量的最大数量；假定其为 `N`，则可行的例子变量编号为 `0 <= i < N`
    pub fn var_count(&self) -> usize {
        todo!()
    }
    /// 例子数量；假定其为 `N`，则可行的例子编号为 `0 <= i < N`
    pub fn example_count(&self) -> usize {
        todo!()
    }
    /// 向求解中增加新的表达式，使其与原有表达式构成合取关系
    pub fn apply_expr(&mut self, _expr: PropExpr) {
        todo!()
    }
}

// pub struct Context {
//     vars: Vec<Variable>,
//     max_skeletons: usize,
//     expressions: Vec<PropExpr>,
// }

// impl Context {
//     pub fn new(n: usize) -> Self {
//         Self {
//             vars: Vec::new(),
//             max_skeletons: n,
//             expressions: Vec::new(),
//         }
//     }

//     pub fn add_expr(&mut self, expr: PropExpr) {
//         self.expressions.push(expr)
//     }

//     pub fn generate_cnf(&self) -> PropExpr {
//         unimplemented!()
//     }
// }

// #[cfg(test)]
// impl Context {
//     pub fn _validate(&self, model: Vec<Variable>) -> bool {
//         for expr in self.expressions.iter() {
//             if expr._validate(&model) == false {
//                 return false;
//             }
//         }
//         true
//     }
// }
