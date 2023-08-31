mod sealed {
    pub trait Format {}
}

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "json")]
pub use json::Json;

#[cfg(feature = "toml")]
mod toml;
#[cfg(feature = "toml")]
pub use crate::toml::Toml;

#[derive(Clone, Copy, Debug)]
pub struct TypedFile<T, F: sealed::Format> {
    inner: T,
    _f: std::marker::PhantomData<F>,
}

impl<T, F: sealed::Format> TypedFile<T, F> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TypedFileParser<T, F: sealed::Format> {
    _t: std::marker::PhantomData<T>,
    _f: std::marker::PhantomData<F>,
}

impl<T, F: sealed::Format> Default for TypedFileParser<T, F> {
    fn default() -> Self {
        Self {
            _t: Default::default(),
            _f: Default::default(),
        }
    }
}

impl<T, F> clap::builder::ValueParserFactory for TypedFile<T, F>
where
    T: Clone + Send + Sync + 'static,
    F: sealed::Format,
{
    type Parser = TypedFileParser<T, F>;

    fn value_parser() -> Self::Parser {
        TypedFileParser::<T, F> {
            _t: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}
