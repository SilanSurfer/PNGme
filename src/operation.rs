use crate::arg;
use crate::error;


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