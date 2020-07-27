//! 输入 JSON 解析。
//!
//! 不能够出现重复变量的位置（单词表、例子中单个时间点变量取值）若有重复变量将被忽略。
//!
//! # Example
//!
//! ```json
//! {
//!     "vocab": ["p", "q", "r"],
//!     "traces_pos": [
//!         [["p"], ["p"], ["q"]],
//!         [["p"], ["q"]],
//!         [["q", "r"]],
//!         [["p", "r"], ["q"]]
//!     ],
//!     "traces_neg": [
//!         [["p"], ["r"], ["q"]],
//!         [["p"], ["r"]],
//!         [["r"], ["q"]]
//!     ]
//! }
//! ```

use super::{Context, Word};
use serde::Deserialize;
use std::{collections::HashSet, convert::TryFrom};

/// 输入的直接表示
#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Input {
    /// 可用变量列表
    vocab: HashSet<String>,
    /// 正例
    traces_pos: Vec<Vec<HashSet<String>>>,
    /// 反例
    traces_neg: Vec<Vec<HashSet<String>>>,
}

impl TryFrom<&str> for Input {
    type Error = serde_json::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}

impl Into<Context> for Input {
    fn into(self) -> Context {
        let mut ctx = Context::new();
        for word in self.vocab {
            ctx.get_word_id(word); // 通过获取单词序号来创建单词
        }
        for pos_ex in self.traces_pos.into_iter() {
            let seq = pos_ex
                .into_iter()
                .map(|t| {
                    t.into_iter()
                        .map(|s| Word::new_unchecked(ctx.get_word_id(s)))
                        .collect()
                })
                .collect();
            ctx.add_example(seq, true);
        }
        for neg_ex in self.traces_neg.into_iter() {
            let seq = neg_ex
                .into_iter()
                .map(|t| {
                    t.into_iter()
                        .map(|s| Word::new_unchecked(ctx.get_word_id(s)))
                        .collect()
                })
                .collect();
            ctx.add_example(seq, false);
        }
        ctx
    }
}
