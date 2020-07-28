/// 输入样例的一个变量
#[derive(Eq, PartialEq, Hash)]
pub struct Word(usize);

impl Word {
    pub fn new_unchecked(id: usize) -> Self {
        Self(id)
    }

    #[allow(dead_code)]
    pub fn new(context: &super::Context, id: usize) -> Result<Self, ()> {
        if id < context.word_count() {
            Ok(Self(id))
        } else {
            Err(())
        }
    }
}

impl std::fmt::Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("v{}", self.0))
    }
}
