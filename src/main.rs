mod arg;
mod chunk;
mod chunk_type;
mod error;
mod png;

use crate::arg::{get_cli_args, DecodeArgs, EncodeArgs, PngMeCliArgs, PrintArgs, RemoveArgs};

fn main() -> Result<(), error::PngMeError> {
    let matches = get_cli_args();
    let args = PngMeCliArgs::new(matches);

    Ok(())
}
