//! # Recursive enum variant enumerator
//!
//! Provides EnumCode trait that gives every enum variant its own string value.
//!
//! ## Features
//!
//! - Tree-like code for every variant
//! - Derive macro
//! - Recursive processing
//! - Iterator over all code values
//! - Can be easely combined with rust_i18n (or in another situations, when you need String code for the variants)
//!
//! [`Enumerator`]: https://github.com/merl-twin/enumerator
//!
//! ## Examples
//!
//! ### Easy use
//!
//! ```
//! use enumerator::EnumCode;
//!
//! #[derive(EnumCode)]
//! enum Error {
//!     Io,
//!     Syntax,
//! }
//!
//! let err = Error::Syntax;
//!
//! assert_eq!("Syntax", err.code());
//!
//! ```
//!
//!
//! ### Easy derive
//!
//! ```
//! use enumerator::EnumCode;
//!
//! #[derive(EnumCode)]
//! enum Io {
//!     Open{ name: String },
//!     Close,
//!     Init(usize),
//! }
//!
//! #[derive(EnumCode)]
//! #[ecode(code = "MyError")]
//! enum Error {
//!     #[ecode(recursive)]
//!     Io(Io),
//!     Syntax,
//! }
//!
//! let err = Error::Io(Io::Init(100));
//!
//! assert_eq!("MyError.Io.Init", err.code());
//!
//! ```
//! ### Easy use in different situations
//!
//! Easy to use even for complex enums
//!
//! ```
//! use enumerator::EnumCode;
//!
//! #[derive(EnumCode)]
//! #[ecode(code = "StandAlone")]
//! struct StandAloneError<T> {
//!     data: T,
//! }
//!
//! #[derive(EnumCode)]
//! enum Io {
//!     Open{ name: String },
//!     Close,
//!     Init(usize),
//! }
//!
//! #[derive(EnumCode)]
//! enum FlatError {
//!     TupleVariant(usize),
//!     StructVariant{ value: String },
//!     UnitVariant,
//! }
//!
//! #[derive(EnumCode)]
//! #[ecode(code = "MyError")]
//! enum Error {
//!     #[ecode(recursive)]
//!     Io(Io),
//!     Syntax,
//!
//!     #[ecode(flatten)]
//!     FlattenVariants(FlatError),
//!
//!     #[ecode(flatten)]
//!     PlaceholderVariant(StandAloneError<()>),
//! }
//!
//! let err = Error::PlaceholderVariant(StandAloneError{ data: () });
//! assert_eq!("MyError.StandAlone", err.code());
//!
//!
//! let err = Error::FlattenVariants(FlatError::StructVariant{ value: "any".to_string() });
//! assert_eq!("MyError.StructVariant", err.code());
//!
//! ```

pub use enumerator_derive::EnumCode;

/// Main trait for the code genearation
pub trait EnumCode {
    /// Return code for the type instance
    ///
    /// ```
    /// use enumerator::EnumCode;
    ///
    /// #[derive(EnumCode)]
    /// enum Error {
    ///     Io,
    ///     Syntax,
    /// }
    ///
    /// let err = Error::Syntax;
    ///
    /// assert_eq!("Syntax", err.code());
    /// ```
    ///
    fn code(&self) -> std::borrow::Cow<'static, str>;

    /// Returns code prefix for this type, if there is any.
    ///
    /// ```
    /// use enumerator::EnumCode;
    ///
    /// #[derive(EnumCode)]
    /// enum Io {
    ///     Open{ name: String },
    ///     Close,
    ///     Init(usize),
    /// }
    ///
    /// #[derive(EnumCode)]
    /// #[ecode(code = "MyError")]
    /// enum Error {
    ///     Io(Io),
    ///     Syntax,
    /// }
    ///
    /// assert_eq!(None, Io::prefix());
    /// assert_eq!(Some("MyError"), Error::prefix());
    /// ```
    ///

    fn prefix() -> Option<&'static str>;

    /// Returns iterator for all values of the type
    ///
    /// ```
    /// use enumerator::EnumCode;
    ///
    /// #[derive(EnumCode)]
    /// #[ecode(code = "StandAlone")]
    /// struct StandAloneError<T> {
    ///     data: T,
    /// }
    ///
    /// #[derive(EnumCode)]
    /// enum Io {
    ///     Open{ name: String },
    ///     Close,
    ///     Init(usize),
    /// }
    ///
    /// #[derive(EnumCode)]
    /// enum FlatError {
    ///     TupleVariant(usize),
    ///     StructVariant{ value: String },
    ///     UnitVariant,
    /// }
    ///
    /// #[derive(EnumCode)]
    /// #[ecode(code = "MyError")]
    /// enum Error {
    ///     #[ecode(recursive)]
    ///     Io(Io),
    ///     Syntax,
    ///
    ///     #[ecode(flatten)]
    ///     FlattenVariants(FlatError),
    ///
    ///     #[ecode(flatten)]
    ///     PlaceholderVariant(StandAloneError<()>),
    /// }
    ///
    /// let data = Error::enumerate().collect::<Vec<_>>();
    ///
    /// assert_eq!(data.len(), 8);
    /// assert_eq!(data[0], "MyError.Io.Open");
    /// assert_eq!(data[1], "MyError.Io.Close");
    /// assert_eq!(data[2], "MyError.Io.Init");
    /// assert_eq!(data[3], "MyError.Syntax");
    /// assert_eq!(data[4], "MyError.TupleVariant");
    /// assert_eq!(data[5], "MyError.StructVariant");
    /// assert_eq!(data[6], "MyError.UnitVariant");
    /// assert_eq!(data[7], "MyError.StandAlone");
    ///
    /// ```
    ///

    fn enumerate() -> CodeEnumerator;
}

/// CodeEnumerator iterates over all values of the type
///
/// ```
/// use enumerator::EnumCode;
///
/// #[derive(EnumCode)]
/// #[ecode(code = "StandAlone")]
/// struct StandAloneError<T> {
///     data: T,
/// }
///
/// #[derive(EnumCode)]
/// enum Io {
///     Open{ name: String },
///     Close,
///     Init(usize),
/// }
///
/// #[derive(EnumCode)]
/// enum FlatError {
///     TupleVariant(usize),
///     StructVariant{ value: String },
///     UnitVariant,
/// }
///
/// #[derive(EnumCode)]
/// #[ecode(code = "MyError")]
/// enum Error {
///     #[ecode(recursive)]
///     Io(Io),
///     Syntax,
///
///     #[ecode(flatten)]
///     FlattenVariants(FlatError),
///
///     #[ecode(flatten)]
///     PlaceholderVariant(StandAloneError<()>),
/// }
///
/// let data = Error::enumerate().collect::<Vec<_>>();
///
/// assert_eq!(data.len(), 8);
/// assert_eq!(data[0], "MyError.Io.Open");
/// assert_eq!(data[1], "MyError.Io.Close");
/// assert_eq!(data[2], "MyError.Io.Init");
/// assert_eq!(data[3], "MyError.Syntax");
/// assert_eq!(data[4], "MyError.TupleVariant");
/// assert_eq!(data[5], "MyError.StructVariant");
/// assert_eq!(data[6], "MyError.UnitVariant");
/// assert_eq!(data[7], "MyError.StandAlone");
///
/// ```
///

pub struct CodeEnumerator(std::vec::IntoIter<std::borrow::Cow<'static, str>>);
impl Iterator for CodeEnumerator {
    type Item = std::borrow::Cow<'static, str>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[doc(hidden)]
impl From<Vec<std::borrow::Cow<'static, str>>> for CodeEnumerator {
    fn from(v: Vec<std::borrow::Cow<'static, str>>) -> CodeEnumerator {
        CodeEnumerator(v.into_iter())
    }
}
