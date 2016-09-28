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
extern crate regex;
extern crate rustc_serialize;
extern crate timer;
extern crate chrono;
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
    println!("  -s|--stage <pkg.json>           Stage package structure from JSON file.");
    println!("  -b|--build <pkg.json>  [<deb>]  Build an archive from JSON file.\n");
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
                            src = dpkgdeb::generate_debian_staging(&src, false);
                        }
                        // dpkgdeb::build_debian_archive(&src, &pn, true);
                    } else {
                        display_error(&program, "--build needs a <directory/pkg.json> argument");
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
