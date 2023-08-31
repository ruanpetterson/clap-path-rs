use clap::Parser;
use clap_path::{Json, TypedFile};
use serde_json::Value;

#[derive(Debug, Parser)]
pub struct Cmd {
    file: TypedFile<Value, Json>,
}

fn main() {
    let cmd = Cmd::parse();
    dbg!(cmd.file.into_inner());
}
