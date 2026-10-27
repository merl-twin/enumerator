# Recursive enum variant enumerator

Provides EnumCode trait that gives every enum variant its own string value.

## Features

- Tree-like code for every variant
- Derive macro
- Recursive processing
- Iterator over all code values
- Can be easely combined with rust_i18n (or in another situations, when you need String code for the variants)

[`Enumerator`]: https://github.com/merl-twin/enumerator

## Examples

### Easy use

```
 use enumerator::EnumCode;

 #[derive(EnumCode)]
 enum Error {
     Io,
     Syntax,
 }

 let err = Error::Syntax;

 assert_eq!("Syntax", err.code());

```


### Easy derive

```
 use enumerator::EnumCode;

 #[derive(EnumCode)]
 enum Io {
     Open{ name: String },
     Close,
     Init(usize),
 }

 #[derive(EnumCode)]
 #[ecode(code = "MyError")]
 enum Error {
     #[ecode(recursive)]
     Io(Io),
     Syntax,
 }

 let err = Error::Io(Io::Init(100));

 assert_eq!("MyError.Io.Init", err.code());

```

### Easy use in different situations

 Easy to use even for complex enums

```
 use enumerator::EnumCode;

 #[derive(EnumCode)]
 #[ecode(code = "StandAlone")]
 struct StandAloneError<T> {
     data: T,
 }

 #[derive(EnumCode)]
 enum Io {
     Open{ name: String },
     Close,
     Init(usize),
 }

 #[derive(EnumCode)]
 enum FlatError {
     TupleVariant(usize),
     StructVariant{ value: String },
     UnitVariant,
 }

 #[derive(EnumCode)]
 #[ecode(code = "MyError")]
 enum Error {
     #[ecode(recursive)]
     Io(Io),
     Syntax,

     #[ecode(flatten)]
     FlattenVariants(FlatError),

     #[ecode(flatten)]
     PlaceholderVariant(StandAloneError<()>),
 }

 let err = Error::PlaceholderVariant(StandAloneError{ data: () });
 assert_eq!("MyError.StandAlone", err.code());


 let err = Error::FlattenVariants(FlatError::StructVariant{ value: "any".to_string() });
 assert_eq!("MyError.StructVariant", err.code());
```
