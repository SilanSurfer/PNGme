use crate::arg;
use crate::error;
use crate::png;
use crate::chunk;
use crate::chunk_type;
use std::fs;
use std::convert::TryFrom;
use std::str::FromStr;

pub fn execute(args: arg::PngMeCliArgs) -> Result<(), error::PngMeError> {
    match args {
        arg::PngMeCliArgs::Encode(arg) => encode(arg)?,
        arg::PngMeCliArgs::Decode(arg) => decode(arg)?,
        arg::PngMeCliArgs::Remove(arg) => remove(arg)?,
        arg::PngMeCliArgs::Print(arg) => print(arg)?,
    }
    Ok(())
}

fn encode(args: arg::EncodeArgs) -> Result<(), error::PngMeError> {
    let file_contents = fs::read(args.filename).map_err(|e| error::PngMeError::IoError(e))?;
    let mut png_data = png::Png::try_from(file_contents.as_slice())?;
    let msg_data = args.msg.as_bytes().to_vec();
    png_data.append_chunk(chunk::Chunk::new(chunk_type::ChunkType::from_str(&args.chunk_type)?, msg_data));
    if let Some(output_filename) = args.output_file {
        fs::write(output_filename, png_data.as_bytes()).map_err(|e| error::PngMeError::IoError(e))?;
    }
    Ok(())
}

fn decode(args: arg::DecodeArgs) -> Result<(), error::PngMeError> {
    Ok(())
}

fn remove(args: arg::RemoveArgs) -> Result<(), error::PngMeError> {
    Ok(())
}

fn print(args: arg::PrintArgs) -> Result<(), error::PngMeError> {
    Ok(())
}