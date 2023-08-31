use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;

use clap::builder::{PathBufValueParser, TypedValueParser};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::Error;
use serde::de::DeserializeOwned;

use crate::{TypedFile, TypedFileParser};

#[derive(Clone, Copy, Debug)]
pub struct Json;
impl crate::sealed::Format for Json {}

impl<T> TypedValueParser for TypedFileParser<T, Json>
where
    T: Clone + Send + Sync + 'static,
    T: DeserializeOwned,
{
    type Value = TypedFile<T, Json>;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, Error> {
        let path = PathBufValueParser::new().parse_ref(cmd, arg, value)?;
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(Self::Value {
            inner: serde_json::from_reader(reader).map_err(|e| {
                let mut error = Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
                error.insert(
                    ContextKind::InvalidValue,
                    ContextValue::String(e.to_string()),
                );
                error
            })?,
            _f: PhantomData,
        })
    }
}
