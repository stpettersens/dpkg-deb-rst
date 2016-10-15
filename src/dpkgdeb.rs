/*
  dpkg-deb implementation in Rust.
  Copyright 2016 Sam Saint-Pettersen.

  Released as original dpkg-deb under the
  GNU General Public License and additionally
  the MIT License; see GPL-LICENSE and MIT-LICENSE.
*/

use tatar::Tatar;
use ark::Ark;
use dos2unix::Dos2Unix;
use regex::Regex;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use toml;
use yaml_rust::YamlLoader;
use xmlJSON::XmlDocument;
use inflector::Inflector;
use std::io::{BufRead, BufReader, Read, Write};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::str::FromStr;
use std::process::exit;

static DELIMITER: char = '_';

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
struct Package {
    package: String,
    version: String,
    section: String,
    priority: String,
    architecture: String,
    installed_size: String,
    maintainer: String,
    description: String,
    _files: Vec<String>,
}

impl Package {
    fn new(package: &str, version: &str, section: &str, priority: &str,
    architecture: &str, installed_size: &str, maintainer: &str, description: &str) -> Package {
        Package {
            package: package.to_owned(),
            version: version.to_owned(),
            section: section.to_owned(),
            priority: priority.to_owned(),
            architecture: architecture.to_owned(),
            installed_size: installed_size.to_owned(),
            maintainer: maintainer.to_owned(),
            description: description.to_owned(),
            _files: Vec::new(),
        }
    }
}

fn get_field_value(line: &str) -> String {
    let split = line.split(": ");
    let fv: Vec<&str> = split.collect();
    fv[1].to_owned()
}

fn read_ctrl_file(control: &str) -> Package {
    let mut package = String::new();
    let mut version = String::new();
    let mut section = String::new();
    let mut priority = String::new();
    let mut architecture = String::new();
    let mut installed_size = String::new();
    let mut maintainer = String::new();
    let mut description = String::new();
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
            package = get_field_value(&l);
        }
        p = Regex::new(r"Version").unwrap();
        if p.is_match(&l) {
            version = get_field_value(&l);
        }
        p = Regex::new(r"Section").unwrap();
        if p.is_match(&l) {
            section = get_field_value(&l);
        }
        p = Regex::new(r"Priority").unwrap();
        if p.is_match(&l) {
            priority = get_field_value(&l);
        }
        p = Regex::new(r"Architecture").unwrap();
        if p.is_match(&l) {
            architecture = get_field_value(&l);
        }
        p = Regex::new(r"Installed-Size").unwrap();
        if p.is_match(&l) {
            installed_size = get_field_value(&l);
        }
        p = Regex::new(r"Maintainer").unwrap();
        if p.is_match(&l) {
            maintainer = get_field_value(&l);
        }
        p = Regex::new(r"Description").unwrap();
        if p.is_match(&l) {
            description = get_field_value(&l);
        }
    }
    Package::new(&package, &version, &section, &priority,
    &architecture, &installed_size, &maintainer, &description)
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
        ctrl.push(format!("{}: {}",
        &kv[0][1..kv[0].len() - 1].to_string().to_train_case(),
        &kv[1][1..kv[1].len() - 1]));
    }
    ctrl[0] = format!("{}{}", &ctrl[0][1..2].to_uppercase(), &ctrl[0][2..ctrl[0].len()]);
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
    generate_debian_staging_from_json(json, true);
}

pub fn generate_debian_staging_from_toml(tomlf: &str, verbose: bool) -> String {
    let mut lines = String::new();
    let mut file = File::open(tomlf).unwrap();
    let _ = file.read_to_string(&mut lines);
    let ptoml = toml::Parser::new(&lines).parse().unwrap();
    let values = ptoml.get("package").unwrap();
    let files = values.as_table().unwrap().get("_files").unwrap();
    let fv = files.as_slice().unwrap().to_vec();
    let mut files = Vec::new();
    for f in fv {
        files.push(format!("{}", f));
    }
    let pkg = Package {
        package: values.as_table().unwrap().get("name").unwrap().as_str().unwrap().to_owned(),
        version: values.as_table().unwrap().get("version").unwrap().as_str().unwrap().to_owned(),
        section: values.as_table().unwrap().get("section").unwrap().as_str().unwrap().to_owned(),
        priority: values.as_table().unwrap().get("priority").unwrap().as_str().unwrap().to_owned(),
        architecture: values.as_table().unwrap().get("architecture").unwrap().as_str().unwrap().to_owned(),
        installed_size: values.as_table().unwrap().get("installed_size").unwrap().as_str().unwrap().to_owned(),
        maintainer: values.as_table().unwrap().get("maintainer").unwrap().as_str().unwrap().to_owned(),
        description: values.as_table().unwrap().get("description").unwrap().as_str().unwrap().to_owned(),
        _files: files,
    };
    generate_common_debian_staging(pkg, true, verbose)
}

pub fn generate_debian_staging_from_yaml(yaml: &str, verbose: bool) -> String {
    let mut lines = String::new();
    let mut file = File::open(yaml).unwrap();
    let _ = file.read_to_string(&mut lines);
    let docs = YamlLoader::load_from_str(&lines).unwrap();
    let doc = &docs[0];
    let fv = doc["_files"].as_vec().unwrap();
    let mut files = Vec::new();
    for f in fv {
        files.push(format!("{}", f.as_str().unwrap()));
    }
    let pkg = Package {
        package: doc["package"].as_str().unwrap().to_owned(),
        version: doc["version"].as_str().unwrap().to_owned(),
        section: doc["section"].as_str().unwrap().to_owned(),
        priority: doc["priority"].as_str().unwrap().to_owned(),
        architecture: doc["architecture"].as_str().unwrap().to_owned(),
        installed_size: doc["installed_size"].as_str().unwrap().to_owned(),
        maintainer: doc["maintainer"].as_str().unwrap().to_owned(),
        description: doc["description"].as_str().unwrap().to_owned(),
        _files: files,
    };
    generate_common_debian_staging(pkg, false, verbose)
}

pub fn generate_debian_staging_from_xml(xml: &str, verbose: bool) -> String {
    let data = XmlDocument::from_str(&xml);
    println!("{:?}", data);
    String::new()
}

pub fn generate_debian_staging_from_json(json: &str, verbose: bool) -> String {
    let mut lines = String::new();
    let mut file = File::open(json).unwrap();
    let _ = file.read_to_string(&mut lines);
    let manifest = Json::from_str(&lines).unwrap();
    let pkg: Package = json::decode(&manifest.to_string()).unwrap();
    generate_common_debian_staging(pkg, false, verbose)
}

fn generate_common_debian_staging(pkg: Package, is_toml: bool, verbose: bool) -> String {
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
        let innn = inn[i].clone();
        let outt = out[i].clone();
        if is_toml {
            let i = format!("{}", &innn[1..innn.len()]);
            let o = format!("{}", &outt[0..outt.len() - 1]);
            let _ = fs::copy(i, o);
        } else {
            let _ = fs::copy(innn, outt);
        }
    }

    for o in out {
        if is_toml {
            let out = format!("{}", &o[0..o.len() - 1]);
            Dos2Unix::convert(&out, false);
        } else {
            Dos2Unix::convert(&o, false);
        }
    }

    thread::sleep_ms(3000);
    fpkg
}
