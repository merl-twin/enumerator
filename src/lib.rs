pub use enumerator_derive::EnumCode;

pub trait EnumCode {
    fn code(&self) -> std::borrow::Cow<'static, str>;

    fn prefix() -> Option<&'static str>;
    fn enumerate() -> CodeEnumerator;
}

pub struct CodeEnumerator(std::vec::IntoIter<std::borrow::Cow<'static, str>>);
impl Iterator for CodeEnumerator {
    type Item = std::borrow::Cow<'static, str>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
impl From<Vec<std::borrow::Cow<'static, str>>> for CodeEnumerator {
    fn from(v: Vec<std::borrow::Cow<'static, str>>) -> CodeEnumerator {
        CodeEnumerator(v.into_iter())
    }
}
