use super::Context;

pub struct Example;

impl Example {
    /// 获得例子编号
    pub fn id(&self) -> usize {
        todo!()
    }
    /// 获得序列长度；若其为 `N`，则时间编号为 `0 <= i < N`
    pub fn size(&self) -> usize {
        todo!()
    }
    /// 获得指定时间对应的变量
    pub fn get(&self, _t: usize) -> usize {
        todo!()
    }
    /// 确认其是否包含特定的变量
    pub fn contains(&self, _v: usize) -> bool {
        todo!()
    }
    /// 获得关联的上下文
    pub fn context(&self) -> &Context {
        todo!()
    }
}
