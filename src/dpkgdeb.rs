/*
  dpkg-deb implementation in Rust.
  Copyright 2016 Sam Saint-Pettersen.

  Released as original dpkg-deb under the
  GNU General Public License and additionally
  the MIT License; see GPL-LICENSE and MIT-LICENSE.
*/

extern crate tatar;
extern crate ark;
extern crate regex;
extern crate rustc_serialize;
use self::tatar::Tatar;
use self::ark::Ark;
use regex::Regex;
use self::rustc_serialize::json::Json;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use std::path::Path;
use std::process::exit;

static DELIMITER: char = '_';

#[derive(Debug, Clone)]
struct Package {
    package: String,
    version: String,
}

impl Package {
    fn new(package: &str, version: &str) -> Package {
        Package {
            package: package.to_owned(),
            version: version.to_owned(),
        }
    }
}

fn read_ctrl_file(control: &str) -> Package {
    let mut package = String::new();
    let mut version = String::new();
    if !Path::new(control).exists() {
        println!("Cannot open `control` file to create Debian archive:\n{}", control);
        exit(2)
    }
    let f = File::open(control).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = format!("{}", line.unwrap());
        let mut p = Regex::new(r"Package").unwrap();
        if p.is_match(&l) {
            let mut split = l.split(": ");
            let fv: Vec<&str> = split.collect();
            package = fv[1].to_owned();
        }
        p = Regex::new(r"Version").unwrap();
        if p.is_match(&l) {
            let mut split = l.split(": ");
            let fv: Vec<&str> = split.collect();
            version = fv[1].to_owned();
        }
    }
    Package::new(&package, &version)
}

fn create_ctrl_archive(pkg: Package) -> Package {
    let rt = format!("{}{}{}", pkg.package, DELIMITER, pkg.version);
    // TODO: Create control.tar.gz using Tatar library here.
    pkg
}

fn create_data_archive(pkg: Package) {
    let rt = format!("{}{}{}", pkg.package, DELIMITER, pkg.version);
    // TODO Create data.tar.gz using Tatar library here.
}

pub fn build_debian_archive(src: &str, pn: &str, verbose: bool) {
    let pkg = create_ctrl_archive(read_ctrl_file(&format!("{}/DEBIAN/control", src)));
    create_data_archive(pkg.clone());
    println!("{:#?}", pkg); // TODO: Remove this.
}

pub fn view_contents_archive(deb: &str) {

}

pub fn view_info_archive(deb: &str) {

}

pub fn generate_debian_staging_1(json: &str) {

}

pub fn generate_debian_staging_2(json: &str, verbose: bool) -> String {
    "!TODO".to_owned()
}
