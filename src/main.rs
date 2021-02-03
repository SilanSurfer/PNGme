mod arg;
mod chunk;
mod chunk_type;
mod error;
mod operation;
mod png;

use crate::arg::{get_cli_args, PngMeCliArgs};
use crate::operation::execute;

fn main() -> Result<(), error::PngMeError> {
    let matches = get_cli_args();
    let args = PngMeCliArgs::new(matches)?;
    execute(args)
}
