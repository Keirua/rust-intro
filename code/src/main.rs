use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

extern crate clap;

use clap::{App, Arg};

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn main() {
    let parameters = App::new("rust-demo")
                        .version("1.0")
                        .about("Search for strings in file")
                        .arg(Arg::with_name("pattern")
                                    .help("The string we are looking for")
                                    .index(1)
                                    .required(true))
                        .arg(Arg::with_name("file")
                                    .help("The file we want to open")
                                    .index(2)
                                    .required(true))
                        .get_matches();

    let filename = parameters.value_of("file").unwrap();
    let query = parameters.value_of("pattern").unwrap();

    println!("Reading {}", filename);

    let mut f = match File::open(&filename) {
        Err(why) => panic!("couldn't open {}: {}", filename,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    println!("Matches for {}\n", query);
    
    let matches = search(query, &contents);
    for current_match in matches {
        println!("{}", current_match);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let contents = "\
C'est pas l'homme qui prend la mer
C'est la mer qui prend l'homme
Moi la mer elle m'a pris
Je m'souviens, un mardi";

        assert_eq!(
            vec!["Je m'souviens, un mardi"],
            search("mardi", contents)
        );
    }
}