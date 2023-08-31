use std::fs;
use std::marker::PhantomData;

use clap::builder::{PathBufValueParser, TypedValueParser};
use serde::de::DeserializeOwned;

use crate::{TypedFile, TypedFileParser};

#[derive(Clone, Copy, Debug)]
pub struct Toml;
impl crate::sealed::Format for Toml {}

impl<T> TypedValueParser for TypedFileParser<T, Toml>
where
    T: Clone + Send + Sync + 'static,
    T: DeserializeOwned,
{
    type Value = TypedFile<T, Toml>;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let path = PathBufValueParser::new().parse_ref(cmd, arg, value)?;
        let contents = fs::read(path)?;

        Ok(Self::Value {
            inner: toml::from_str(std::str::from_utf8(&contents).unwrap()).unwrap(),
            _f: PhantomData,
        })
    }
}
