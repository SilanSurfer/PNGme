mod arg;
mod chunk;
mod chunk_type;
mod error;
mod png;

use crate::arg::{get_cli_args, DecodeArgs, EncodeArgs, PngMeCliArgs, PrintArgs, RemoveArgs};

fn main() -> Result<(), error::PngMeError> {
    let matches = get_cli_args();

    let args = match matches.subcommand() {
        ("encode", Some(encode_args)) => Ok(PngMeCliArgs::Encode(EncodeArgs {
            filename: encode_args.value_of("filename").unwrap().to_string(),
            chunk_type: encode_args.value_of("type").unwrap().to_string(),
            msg: encode_args.value_of("message").unwrap().to_string(),
            output_file: None,
        })),
        ("decode", Some(decode_args)) => Ok(PngMeCliArgs::Decode(DecodeArgs {
            filename: decode_args.value_of("filename").unwrap().to_string(),
            chunk_type: decode_args.value_of("type").unwrap().to_string(),
        })),
        ("remove", Some(remove_args)) => Ok(PngMeCliArgs::Remove(RemoveArgs {
            filename: remove_args.value_of("filename").unwrap().to_string(),
            chunk_type: remove_args.value_of("type").unwrap().to_string(),
        })),
        ("print", Some(print_args)) => Ok(PngMeCliArgs::Print(PrintArgs {
            filename: print_args.value_of("filename").unwrap().to_string(),
        })),
        (_, _) => Err(error::PngMeError::InvalidCliArguments),
    }?;

    Ok(())
}
