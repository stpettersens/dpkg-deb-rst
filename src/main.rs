/*
  dpkg-deb implementation in Rust.
  Copyright 2016 Sam Saint-Pettersen.

  Released as original dpkg-deb under the
  GNU General Public License and additionally
  the MIT License; see GPL-LICENSE and MIT-LICENSE.
*/

mod dpkgdeb;
extern crate clioptions;
extern crate tatar;
extern crate ark;
extern crate dos2unix;
extern crate regex;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate yaml_rust;
extern crate xmlJSON;
extern crate inflector;
use clioptions::CliOptions;
use regex::Regex;
use std::process::exit;

fn display_error(program: &str, err: &str) {
    println!("dpkg-deb-rst: error: {}.", err);
    display_usage(program, 2);
}

fn display_usage(program: &str, code: i32) {
    println!("\nUsage: {} [<option> ...] <command>", program);
    println!("\nStandard commands:");
    println!("  -b|--build <directory> [<deb>]  Build an archive.");
    println!("  -c|--contents <deb>             List contents.");
    println!("  -I|--info <deb>                 Show info to stdout.");
    println!("\nExtended commands:");
    println!("  -s|--stage <pkg.json>           Stage package structure from JSON pkg file.");
    println!("  -s|--stage <pkg.toml>           Stage package structure from TOML pkg file.");
    println!("  -s|--stage <pkg.yaml>           Stage package structure from YAML pkg file.");
    println!("  -b|--build <pkg.json>  [<deb>]  Build an archive from JSON pkg file.");
    println!("  -b|--build <pkg.toml>  [<deb>]  Build an archive from TOML pkg file.");
    println!("  -b|--build <pkg.yaml>  [<deb>]  Build an archive from YAML pkg file.\n");
    exit(code);
}

fn main() {
    let cli = CliOptions::new("dpkg-deb-rst");
    let program = cli.get_program();

    if cli.get_num() > 1 {
        for (i, a) in cli.get_args().iter().enumerate() {
            match a.trim() {
                "-h" | "--help" => display_usage(&program, 0),
                "-b" | "--build" => {
                    let mut src = cli.next_argument(i);
                    let pn = cli.next_argument(i + 1);
                    if !src.is_empty() {
                        let json = Regex::new(r".json$").unwrap();
                        if json.is_match(&src) {
                            src = dpkgdeb::generate_debian_staging_from_json(&src, false);
                        }
                        let toml = Regex::new(r".t(o)*ml$").unwrap();
                        if toml.is_match(&src) {
                            src = dpkgdeb::generate_debian_staging_from_toml(&src, false);
                        }
                        let yaml = Regex::new(r".y(a)*ml$").unwrap();
                        if yaml.is_match(&src) {
                            src = dpkgdeb::generate_debian_staging_from_yaml(&src, false);
                        }
                        let xml = Regex::new(r".xml$").unwrap();
                        if xml.is_match(&src) {
                            src = dpkgdeb::generate_debian_staging_from_xml(&src, false);
                        }
                        dpkgdeb::build_debian_archive(&src, &pn, true);
                    } else {
                        display_error(&program, "--build needs a <directory/pkg.{json|toml|yaml}> argument");
                    }
                },
                "-c" | "--contents" => dpkgdeb::view_contents_archive(&cli.next_argument(i)),
                "-I" | "--info" => dpkgdeb::view_info_archive(&cli.next_argument(i)),
                "-s" | "--stage" => dpkgdeb::generate_debian_staging_void(&cli.next_argument(i)),
                _ => continue,
            }
        }
    } else {
        display_error(&program, "need an action operation");
    }
}
