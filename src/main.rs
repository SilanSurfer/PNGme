mod chunk;
mod chunk_type;
mod error;
mod png;
mod arg;

use crate::arg::{get_cli_args, PngMeCliArgs, EncodeArgs, DecodeArgs, PrintArgs, RemoveArgs};

fn main() -> Result<(), error::PngMeError> {
    let matches = get_cli_args();

    if let Some(ref matches) = matches.subcommand_matches("decode") {
        let filename = matches.value_of("filename").unwrap();
        let chunk_type = matches.value_of("type").unwrap();
        println!("Decode operation was executed with params: [{}] {}", filename, chunk_type);
    }

    if let Some(ref matches) = matches.subcommand_matches("remove") {
        let filename = matches.value_of("filename").unwrap();
        let chunk_type = matches.value_of("type").unwrap();
        println!("Remove operation was executed with params: [{}] {}", filename, chunk_type);
    }

    if let Some(ref matches) = matches.subcommand_matches("print") {
        let filename = matches.value_of("filename").unwrap();
        println!("Print operation was executed with params: [{}]", filename);
    }
}
