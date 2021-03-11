use clap::{App, Arg, ArgMatches, SubCommand};
use serde_json::Value;
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

fn main() {
    if let Err(e) = run() {
        println!("{:?}", e.to_string());
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let matches = App::new("UVTT")
        .version("0.1.0")
        .about("Extracts and inserts UVTT images")
        .subcommand(SubCommand::with_name("extract")
            .about("Extracts a UVTT image")
            .arg(Arg::with_name("in")
                .short("i")
                .help("UVTT filename containing the image to extract")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("out")
                .short("o")
                .help("Filename for the extracted image")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("insert")
            .about("Inserts an image into a UVTT")
            .arg(Arg::with_name("in")
                .short("i")
                .help("UVTT filename to insert the image into")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("image")
                .short("m")
                .help("Image filename to insert into the UVTT")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("out")
                .short("o")
                .help("Filename for the UVTT with the inserted image")
                .takes_value(true)))
        .get_matches();

    match matches.subcommand() {
        ("extract", Some(ext)) => sub_extract(ext),
        ("insert", Some(ins)) => sub_insert(ins),
        _ => Ok(()),
    }
}

fn sub_extract(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let uvtt = matches.value_of("in").unwrap();
    let image = if let Some(out) = matches.value_of("out") {
        out.to_string()
    } else {
        let stem = Path::new(uvtt).file_stem().and_then(OsStr::to_str).unwrap();
        format!("{}.png", stem)
    };
    extract(uvtt, &image)
}

fn sub_insert(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let uvtt_in = matches.value_of("in").unwrap();
    let image = matches.value_of("image").unwrap();
    let uvtt_out = if let Some(out) = matches.value_of("out") {
        out.to_string()
    } else {
        uvtt_in.to_string()
    };
    insert(uvtt_in, image, &uvtt_out)
}

fn extract(uvtt: &str, image: &str) -> Result<(), Box<dyn Error>> {
    let image_filename = image;
    let uvtt = std::fs::read_to_string(uvtt)?;
    let uvtt: Value = serde_json::from_str(&uvtt)?;
    let image = &uvtt["image"];
    let image = base64::decode(image.as_str().unwrap())?;
    std::fs::write(image_filename, image)?;
    Ok(())
}

fn insert(uvtt_in: &str, image: &str, uvtt_out: &str) -> Result<(), Box<dyn Error>> {
    let image = std::fs::read(image)?;
    let image = base64::encode(image);
    let image = Value::String(image);
    let uvtt_in = std::fs::read_to_string(uvtt_in)?;
    let mut uvtt_in: Value = serde_json::from_str(&uvtt_in)?;
    uvtt_in["image"] = image;
    let uvtt_in = serde_json::to_string_pretty(&uvtt_in)?;
    std::fs::write(uvtt_out, uvtt_in)?;
    Ok(())
}
