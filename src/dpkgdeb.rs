/*
  dpkg-deb implementation in Rust.
  Copyright 2016 Sam Saint-Pettersen.

  Released as original dpkg-deb under the
  GNU General Public License and additionally
  the MIT License; see GPL-LICENSE and MIT-LICENSE.
*/

use tatar::Tatar;
use ark::Ark;
use regex::Regex;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::io::{BufRead, BufReader, Write};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::process::exit;

static DELIMITER: char = '_';

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
struct Package {
    package: String,
    version: String,
    section: String,
    priority: String,
    architecture: String,
    maintainer: String,
    description: String,
    _files: Vec<String>,
}

impl Package {
    fn new(package: &str, version: &str) -> Package {
        Package {
            package: package.to_owned(),
            version: version.to_owned(),
            section: String::new(),
            priority: String::new(),
            architecture: String::new(),
            maintainer: String::new(),
            description: String::new(),
            _files: Vec::new(),
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
            let split = l.split(": ");
            let fv: Vec<&str> = split.collect();
            package = fv[1].to_owned();
        }
        p = Regex::new(r"Version").unwrap();
        if p.is_match(&l) {
            let split = l.split(": ");
            let fv: Vec<&str> = split.collect();
            version = fv[1].to_owned();
        }
    }
    Package::new(&package, &version)
}

fn create_ctrl_vector(ctrls: String) -> Vec<String> {
    let split = ctrls.split(",");
    let ctrlv: Vec<&str> = split.collect();
    let mut ctrl = Vec::new();
    for c in ctrlv {
        let p = Regex::new(r"_files").unwrap();
        if p.is_match(&c) {
            break;
        }
        let split = c.split(":");
        let kv: Vec<&str> = split.collect();
        ctrl.push(format!("{}{}: {}", &kv[0][1..2].to_uppercase(), 
        &kv[0][2..kv[0].len() - 1], &kv[1][1..kv[1].len() - 1]));
    }
    ctrl[0] = format!("P{}", &ctrl[0][2..ctrl[0].len()]);
    ctrl.push("".to_owned());
    ctrl
}

fn create_ctrl_archive(pkg: Package) -> Package {
    let rt = format!("{}{}{}", pkg.package, DELIMITER, pkg.version);
    Tatar::create_single_tar("control.tar.gz", &format!("{}/DEBIAN/control", rt)); // TODO: tar -> tar.gz.
    pkg
}

fn create_data_archive(pkg: Package) {
    let rt = format!("{}{}{}", pkg.package, DELIMITER, pkg.version);
    Tatar::create_single_tar("data.tar.gz", "opt"); // TODO: tar -> tar.gz.
}

fn create_deb_archive(pkg: Package, verbose: bool) -> i32 {
    let deb = format!("{}{}{}", pkg.package, DELIMITER, pkg.version);
    let contents = vec!["debian-binary", "control.tar.gz", "data.tar.gz"];
    if verbose {
        println!("dpkg-deb-rst: building package '{}' in '{}'.", pkg.package, deb);
    }
    let mut w = File::create("debian-binary").unwrap();
    let _ = w.write_all("2.0\n".as_bytes());
    //Ark::create_archive(deb, contents);
    0
}

fn clean_up() {
    // TODO.
}

pub fn build_debian_archive(src: &str, pn: &str, verbose: bool) {
    let pkg = create_ctrl_archive(read_ctrl_file(&format!("{}/DEBIAN/control", src)));
    //create_data_archive(pkg.clone());
    if create_deb_archive(pkg.clone(), verbose) == 0 {
        exit(0);
    } else {
        exit(2);
    }
}

pub fn view_contents_archive(deb: &str) {
    //Ark::unpack_archive(deb);
    //Tatar::extract_tar("data.tar.gz"); // TODO: tar -> tar.gz.
    //Tatar::list_tar("data.tar.gz"); // TODO: tar -> tar.gz.
    println!("Not yet implemented.");
}

pub fn view_info_archive(deb: &str) {
    //Ark::unpack_archive(deb);
    println!("Not yet implemented.");
}

pub fn generate_debian_staging_void(json: &str) {
    generate_debian_staging(json, true);
}

pub fn generate_debian_staging(json: &str, verbose: bool) -> String {
    let mut lines = Vec::new();
    let f = File::open(json).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        lines.push(line.unwrap());
    }
    let manifest = Json::from_str(&lines.join("")).unwrap();
    let pkg: Package = json::decode(&manifest.to_string()).unwrap();

    if pkg.package.is_empty() || pkg.version.is_empty() || pkg._files[0].is_empty() {
        println!("At least package name, version and _files must be defined.");
        exit(2);
    }

    if verbose {
        println!("dpkg-deb-rst: staging '{}'.", pkg.package);
    }

    let fpkg = format!("{}{}{}", pkg.package, DELIMITER, pkg.version);
    let dpath = format!("{}/DEBIAN", fpkg);
    let _ = fs::create_dir_all(dpath.clone());

    let ctrl = create_ctrl_vector(json::encode(&pkg).unwrap());
    let mut w = File::create(format!("{}/control", dpath)).unwrap();
    let _ = w.write_all(ctrl.join("\n").as_bytes());

    let mut inn = Vec::new();
    let mut out = Vec::new();
    for f in pkg._files {
        let split = f.split(":");
        let target: Vec<&str> = split.collect();
        inn.push(format!("{}", target[0]));
        out.push(format!("{}/{}", fpkg, target[1]));
    }

    for fd in out.clone() {
        let split = fd.split("/");
        let dirs: Vec<&str> = split.collect();
        let mut fdp = String::new();
        for (i, d) in dirs.iter().enumerate() {
            if i == dirs.len() - 1 {
                break;
            }
            fdp = format!("{}/{}", fdp, d);
        }
        fdp = format!("{}", &fdp[1..fdp.len()]);
        let _ = fs::create_dir_all(fdp);
    }

    for i in 0..inn.len() {
        let _ = fs::copy(inn[i].clone(), out[i].clone());
    }
    thread::sleep_ms(3000);
    fpkg
}
