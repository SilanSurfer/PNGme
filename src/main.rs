mod chunk;
mod chunk_type;
mod error;
mod png;
mod arg;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
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
    if let Some(ref matches) = matches.subcommand_matches("encode") {
        let filename = matches.value_of("filename").unwrap();
        let chunk_type = matches.value_of("type").unwrap();
        let message = matches.value_of("message").unwrap();
        println!("Encode operation was executed with params: [{}] {}, {}", filename, chunk_type, message);
    }

    if let Some(ref matches) = matches.subcommand_matches("decode") {
        let filename = matches.value_of("filename").unwrap();
        let chunk_type = matches.value_of("type").unwrap();
        println!("Decode operation was executed with params: [{}] {}", filename, chunk_type);
    }

    if let Some(ref matches) = matches.subcommand_matches("remove") {
        let filename = matches.value_of("filename").unwrap();
        let chunk_type = matches.value_of("type").unwrap();
        println!("Remove operation was executed with params: [{}] {}", filename, chunk_type);
    }

    if let Some(ref matches) = matches.subcommand_matches("print") {
        let filename = matches.value_of("filename").unwrap();
        println!("Print operation was executed with params: [{}]", filename);
    }
}
