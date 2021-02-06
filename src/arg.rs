extern crate clap;
use crate::error::PngMeError;
use clap::{App, Arg, ArgMatches, SubCommand};

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
                output_file: encode_args.value_of("output").map(|val| val.to_string()),
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
                )
                .arg(Arg::with_name("output").help("Output file").required(false)),
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
