extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::error::PngMeError;
use crate::png;
use crate::chunk;
use crate::chunk_type;
use std::fs;
use std::convert::TryFrom;
use std::str::FromStr;

pub enum PngMeCliArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

pub struct EncodeArgs {
    pub filename: String,
    pub chunk_type: String,
    pub msg: String,
    pub output_file: Option<String>,
}

pub struct DecodeArgs {
    pub filename: String,
    pub chunk_type: String,
}

pub struct RemoveArgs {
    pub filename: String,
    pub chunk_type: String,
}

pub struct PrintArgs {
    pub filename: String,
}


impl PngMeCliArgs {
    pub fn new(matches: ArgMatches) -> Result<PngMeCliArgs, PngMeError> {
        match matches.subcommand() {
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
            (_, _) => Err(PngMeError::InvalidCliArguments),
        }
    }

    pub fn encode(args: EncodeArgs) -> Result<(), PngMeError> {
        let file_contents = fs::read(args.filename).map_err(|e| PngMeError::IoError(e))?;
        let mut png_data = png::Png::try_from(file_contents.as_slice())?;
        let msg_data = args.msg.as_bytes().to_vec();
        png_data.append_chunk(chunk::Chunk::new(chunk_type::ChunkType::from_str(&args.chunk_type)?, msg_data));
        if let Some(output_filename) = args.output_file {
            fs::write(output_filename, png_data.as_bytes()).map_err(|e| PngMeError::IoError(e))?;
        }
        Ok(())
    }
}

pub fn get_cli_args<'a>() -> ArgMatches<'a> {
    let matches = App::new("PNGme")
        .version("0.1.0")
        .author("SilanSurfer <szwarc.adam@gmail.com>")
        .about("Let's you modify PNG file to include secret message")
        .subcommand(
            SubCommand::with_name("encode")
                .about("Encodes string in PNG file")
                .arg(
                    Arg::with_name("filename")
                        .help("Name of the file to encode message")
                        .required(true),
                )
                .arg(Arg::with_name("type").help("Chunk type").required(true))
                .arg(
                    Arg::with_name("message")
                        .help("Message to encode")
                        .default_value(""),
                ),
        )
        .subcommand(
            SubCommand::with_name("decode")
                .about("Decodes chunk with type provided")
                .arg(
                    Arg::with_name("filename")
                        .help("Name of the file to encode message")
                        .required(true),
                )
                .arg(Arg::with_name("type").help("Chunk type").required(true)),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Removes chunk with type provided")
                .arg(
                    Arg::with_name("filename")
                        .help("Name of the file to encode message")
                        .required(true),
                )
                .arg(Arg::with_name("type").help("Chunk type").required(true)),
        )
        .subcommand(
            SubCommand::with_name("print").about("Prints PNG file").arg(
                Arg::with_name("filename")
                    .help("Name of the file to encode message")
                    .required(true),
            ),
        )
        .get_matches();
    return matches;
}
