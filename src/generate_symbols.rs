use log::{info, trace, warn};
// use regex::bytes::RegexSet;
use regex::Regex;
use std::fs::{File, self};
use std::io::copy;
use std::io::{self, Read};
use std::path::Path;
use std::process;
use std::io::{Write, BufReader, BufRead, Error};


const URL_SRC: &str = "http://milde.users.sourceforge.net/LUCR/Math/data/unimathsymbols.txt";

pub async fn run() -> io::Result<()> {
    let response = match reqwest::get(URL_SRC).await {
        Ok(r) => r,
        Err(error) => panic!("GET Request Error: {:?}", error),
    };

    let mut dl = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("latex_src.bin");
        info!("Latex symbol source file: {}", fname);
        let fname = format!("{}/{}", std::env!("CARGO_MANIFEST_DIR").to_string(), fname);
        info!("Downloaded to: '{:?}'", fname);
        File::create(fname)?
    };
    let content = match response.text().await {
        Ok(c) => {
            info!("Content read: {}", c);
            c
        }
        Err(error) => {
            warn!("Problem reading contents: {:?}", error);
            panic!()
        }
    };
    copy(&mut content.as_bytes(), &mut dl)?;
    Ok(())
}

#[tokio::main]
pub async fn main() {
    trace!("Starting generation of latex symbols");
    if let Err(err) = run().await {
        eprintln!("{}", err);
        process::exit(1);
    }
}

pub fn generate_symbols() {
    let re1 = Regex::new(r"^[\d]\w+\^(.)").unwrap();
    let re2 = Regex::new(r"(\\\w+)\{?(\\?\w+)(\}?)").unwrap();
    let mut latex_symbol_file = File::open(format!(
        "{}/{}",
        std::env!("CARGO_MANIFEST_DIR"),
        "unimathsymbols.txt"
    ))
    .expect("File may not exist!");
    let mut contents = String::new();
    match latex_symbol_file.read_to_string(&mut contents) {
        Ok(c) => c,
        Err(error) => panic!("Problem reading contents: {:?}", error),
    };
    let path = format!("{}/{}", std::env!("CARGO_MANIFEST_DIR"), "lua/cmp_latex_symbols/items.lua");
    if Path::new(&path).exists() {
        fs::remove_file(Path::new(&path)).unwrap();
    };
    let mut output_file = File::create(path).unwrap();
    write!(output_file, "local symbols = {{\n").unwrap();
    for line in contents.lines() {
        for cap1 in re1.captures_iter(&line) {
            for cap2 in re2.captures_iter(&line) {
                let symbol = cap1.get(0).unwrap().as_str().chars().last().unwrap();
                let word = cap2.get(0).unwrap().as_str();
                if symbol != "\\".to_string().chars().last().unwrap() {
                    write!(output_file, "{}", format!(
                    "\t{{word=\"\\{}\", label=\"\\{} {}\", insertText=\"{}\", filterText=\"\\{}\" }},\n",
                    word.replace("{\\", "{\\\\"), 
                    word.replace("{\\", "{\\\\"), 
                    symbol, 
                    symbol, 
                    word.replace("{\\", "{\\\\")
                )).unwrap();
                }
            }
        }
    }
    write!(output_file, "}}\nreturn symbols").unwrap();
}
