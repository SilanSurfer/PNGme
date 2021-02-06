mod arg;
mod chunk;
mod chunk_type;
mod error;
mod operation;
mod png;

use crate::arg::{get_cli_args, PngMeCliArgs};
use crate::operation::run;

fn main() -> Result<(), error::PngMeError> {
    let args = PngMeCliArgs::new(get_cli_args())?;
    run(args)
}
