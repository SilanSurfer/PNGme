
extern crate clap;
use clap::{App, Arg, SubCommand, ArgMatches};


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

pub fn get_cli_args<'a>() -> ArgMatches<'a> {
    let matches = App::new("PNGme")
    .version("0.1.0")
    .author("SilanSurfer <szwarc.adam@gmail.com>")
    .about("Let's you modify PNG file to include secret message")
    .subcommand(SubCommand::with_name("encode")
        .about("Encodes string in PNG file")
        .arg(
            Arg::with_name("filename")
                .help("Name of the file to encode message")
                .required(true),
        )
        .arg(
            Arg::with_name("type")
                .help("Chunk type")
                .required(true)
        )
        .arg(
            Arg::with_name("message")
                .help("Message to encode")
                .default_value("")
        )
    )
    .subcommand(SubCommand::with_name("decode")
        .about("Decodes chunk with type provided")
        .arg(
            Arg::with_name("filename")
                .help("Name of the file to encode message")
                .required(true),
        )
        .arg(
            Arg::with_name("type")
                .help("Chunk type")
                .required(true)
        )
    )
    .subcommand(SubCommand::with_name("remove")
        .about("Removes chunk with type provided")
        .arg(
            Arg::with_name("filename")
                .help("Name of the file to encode message")
                .required(true),
        )
        .arg(
            Arg::with_name("type")
                .help("Chunk type")
                .required(true)
        )
    )
    .subcommand(SubCommand::with_name("print")
        .about("Prints PNG file")
        .arg(
            Arg::with_name("filename")
                .help("Name of the file to encode message")
                .required(true),
        )
    )
    .get_matches();
    return matches;
}