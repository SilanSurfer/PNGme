mod arg;
mod chunk;
mod chunk_type;
mod error;
mod png;
mod operation;

use crate::operation::execute;
use crate::arg::{get_cli_args, PngMeCliArgs};

fn main() -> Result<(), error::PngMeError> {
    let matches = get_cli_args();
    let args = PngMeCliArgs::new(matches)?;
    execute(args)
}
