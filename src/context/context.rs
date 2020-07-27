use super::{Example, Word};
use std::collections::{HashMap, HashSet};

pub struct Context {
    max_skeletons: usize,
    vocab: HashMap<String, usize>,
    examples: Vec<Example>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            max_skeletons: 0,
            vocab: HashMap::new(),
            examples: Vec::new(),
        }
    }

    pub fn with_bound(max_skeletons: usize) -> Self {
        let mut c = Self::new();
        c.max_skeletons = max_skeletons;
        c
    }
}

impl Context {
    /// Skeleton 的最大数量限制；假定其为 `N`，则可行的 Skeleton 编号为 `0 <= i < N`
    pub fn max_skeletons(&self) -> usize {
        self.max_skeletons
    }
    /// 例子中变量的最大数量；假定其为 `N`，则可行的例子变量编号为 `0 <= i < N`
    pub fn word_count(&self) -> usize {
        self.vocab.len()
    }
    /// 例子数量；假定其为 `N`，则可行的例子编号为 `0 <= i < N`
    pub fn example_count(&self) -> usize {
        self.examples.len()
    }
    /// 根据单词的名称获得单词的序号
    pub fn get_word_id(&mut self, word: String) -> usize {
        if let Some(id) = self.vocab.get(&word) {
            return id.to_owned();
        }
        self.vocab.insert(word, self.vocab.len());
        self.vocab.len() - 1
    }
    /// 添加例子，会自动创建对应的单词
    pub fn add_example(&mut self, sequence: Vec<HashSet<Word>>, is_positive: bool) {
        self.examples
            .push(Example::new(self.examples.len(), sequence, is_positive))
    }
    /// 获得所有例子
    pub fn examples(&self) -> impl Iterator<Item = &Example> {
        self.examples.iter()
    }
    /// 获得所有单词
    pub fn words(&self) -> &HashMap<String, usize> {
        &self.vocab
    }
    /// 设置尺寸限制
    pub fn set_size_bound(&mut self, bound: usize) {
        self.max_skeletons = bound;
    }
}
