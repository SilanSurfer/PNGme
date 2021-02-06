use crate::arg;
use crate::chunk;
use crate::chunk_type;
use crate::error;
use crate::png;
use std::convert::TryFrom;
use std::fs;
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
    println!("{}", args);
    let file_contents = fs::read(&args.filename).map_err(|e| error::PngMeError::IoError(e))?;
    let mut png_data = png::Png::try_from(file_contents.as_slice())?;
    let msg_data = args.msg.as_bytes().to_vec();
    png_data.append_chunk(chunk::Chunk::new(
        chunk_type::ChunkType::from_str(&args.chunk_type)?,
        msg_data,
    ));

    let output = match args.output_file {
        Some(output_file) => output_file,
        None => args.filename,
    };
    fs::write(output, png_data.as_bytes()).map_err(|e| error::PngMeError::IoError(e))?;
    Ok(())
}

fn decode(args: arg::DecodeArgs) -> Result<(), error::PngMeError> {
    println!("{}", args);
    let file_contents = fs::read(args.filename).map_err(|e| error::PngMeError::IoError(e))?;
    let png_data = png::Png::try_from(file_contents.as_slice())?;
    if let Some(chunk) = png_data.chunk_by_type(&args.chunk_type) {
        let msg = chunk
            .data_as_string()
            .map_err(|e| error::PngMeError::FromUtf8ConversionError(e))?;
        println!("Message in chunk type {} is:\n{}", &args.chunk_type, msg);
    } else {
        println!("Couldn't find chunk type {} in file", &args.chunk_type);
    }
    Ok(())
}

fn remove(args: arg::RemoveArgs) -> Result<(), error::PngMeError> {
    println!("{}", args);
    let file_contents = fs::read(&args.filename).map_err(|e| error::PngMeError::IoError(e))?;
    let mut png_data = png::Png::try_from(file_contents.as_slice())?;
    match png_data.remove_chunk(&args.chunk_type) {
        Ok(_) => {
            println!("Chunk {} has been removed!", &args.chunk_type);
            fs::write(&args.filename, png_data.as_bytes())
                .map_err(|e| error::PngMeError::IoError(e))?;
        }
        Err(e) => {
            println!("Couldn't remove chunk {}", &args.chunk_type);
            return Err(e);
        }
    }
    Ok(())
}

fn print(args: arg::PrintArgs) -> Result<(), error::PngMeError> {
    println!("{}", args);
    let file_contents = fs::read(&args.filename).map_err(|e| error::PngMeError::IoError(e))?;
    let png_data = png::Png::try_from(file_contents.as_slice())?;
    println!("{}", png_data);
    Ok(())
}
