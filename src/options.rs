// (Full example with detailed comments in examples/01d_quick_example.rs)
//
// This example demonstrates clap's full 'custom derive' style of creating arguments which is the
// simplest method of use, but sacrifices some flexibility.
use clap::{ArgGroup, Clap};

/// 根据输入序列学习 LTL 公式
///
/// 基于 https://aaai.org/ojs/index.php/ICAPS/article/view/3529
#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "FU Yuze <i@xfox.me>")]
#[clap(group = ArgGroup::new("format"))]
pub struct Opts {
    /// 观察到的现象输入 JSON 文件名
    pub input: String,
    /// 输出文件名
    pub output: String,
    /// 生成逻辑表达式
    #[clap(short = "e", long = "expr", group = "format")]
    pub fmt_expr: bool,
    /// 生成 DIMACS CNF 文件
    #[clap(short = "c", long = "cnf", group = "format")]
    pub fmt_cnf: bool,
    /// 生成求解结果
    #[clap(short = "r", long = "result", group = "format")]
    pub fmt_res: bool,
    // /// 生成 LTLf 公式（默认）
    // #[clap(short = "l", long = "ltl", group = "format")]
    // output_ltl: bool,
    /// 生成 Python 表达的元组
    #[clap(short = "t", long = "tuple", group = "format")]
    pub fmt_tuple: bool,
    /// 同时生成 LTLf 和元组，每个一行
    #[clap(short = "b", long = "both", group = "format")]
    pub fmt_both: bool,
    /// 指定生成 AFA 大小
    #[clap(short = "n", long = "size", required = true)]
    pub size: usize,
}

impl Opts {
    pub fn fmt_ltl(&self) -> bool {
        !self.fmt_expr && !self.fmt_cnf && !self.fmt_res && !self.fmt_tuple && !self.fmt_both
    }
}
