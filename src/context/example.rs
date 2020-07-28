use super::Word;
use std::collections::HashSet;

pub struct Example {
    id: usize,
    sequence: Vec<HashSet<Word>>,
    is_positive: bool,
}

impl Example {
    pub fn new(id: usize, sequence: Vec<HashSet<Word>>, is_positive: bool) -> Self {
        Self {
            id,
            sequence,
            is_positive,
        }
    }
}

impl Example {
    /// 获得例子编号
    pub fn id(&self) -> usize {
        self.id
    }
    /// 获得序列长度；若其为 `N`，则时间编号为 `0 <= i < N`
    pub fn size(&self) -> usize {
        self.sequence.len()
    }
    /// 确定在给定时间是否包含特定的变量
    pub fn contains_at(&self, t: usize, v: usize) -> bool {
        self.sequence[t].contains(&Word::new_unchecked(v))
    }
    /// 返回其为正例还是反例
    pub fn is_pos(&self) -> bool {
        self.is_positive
    }
}

impl PartialEq for Example {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Debug for Example {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.is_positive { "+" } else { "-" })
            .and_then(|_| {
                let mut dt = f.debug_tuple("Example");
                dt.field(&self.id);
                for v in self.sequence.iter() {
                    dt.field(v);
                }
                dt.finish()
            })
    }
}
