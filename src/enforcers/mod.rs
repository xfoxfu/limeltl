//! 利用已有的例子或内容生成逻辑表达式以确保其符合约束。
//!
//! `Enforcer` 是对于规则生成器的抽象。
//!
//! 1. `AFASkTypeEnforcer` 保证每个节点属于且只属于一种 AFA 类型
//! 2. `AFASpecificStructureEnforcer` 保证对应节点类型的子树数量符合要求
//! 3. `SizeBoundEnforcer` 保证不存在节点重用，从而保证 AFA 大小符合要求
//! 4. `LTLSubtreeEnforcer` 保证每个节点存在编号大于其自身的子树。
//! 5. `LTLSizeEnforcer` 检查在 `N-2`（`N-1`）以后不存在二目（单目）子树，从而确保尺寸限制
//! 6. `PositiveExampleEnforcer` 确保生成的结构接受正例
//! 7. <del>`NegativeExampleEnforcer` 确保生成的结构拒绝反例</del>

use crate::{bool_logic::PropExpr, context::Context};

mod afa_size;
mod example_positive;
mod ltl_afa;
mod size_bound;
mod structure;

pub use afa_size::LTLSizeEnforcer;
pub use ltl_afa::LTLSubtreeEnforcer;
pub use size_bound::SizeBoundEnforcer;
pub use structure::AFASkTypeEnforcer;
pub use structure::AFASpecificStructureEnforcer;

pub trait Enforcer {
    /// 生成规则
    fn rules(&self, ctx: &Context) -> Vec<PropExpr>;
}
