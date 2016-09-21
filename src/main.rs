/*
  dpkg-deb implementation in Rust.
  Copyright 2016 Sam Saint-Pettersen.

  Released as original dpkg-deb under the
  GNU General Public License and additionally
  the MIT License; see GPL-LICENSE and MIT-LICENSE.
*/

mod dpkgdeb;
extern crate clioptions;
use clioptions::CliOptions;

fn display_error(program: &str, err: &str) {
    println!("Error: {}.", err);
    display_usage(&program, 2);
}

fn display_version() {

}

fn display_usage(program: &str, code: i32) {

}

fn main() {
    let cli = CliOptions::new("dpkg-deb-rst");
    let program = cli.get_program();

    if cli.get_num() > 1 {
        for (i, a) in cli.get_args().iter().enumerate() {
            match a.trim() {
                "-h" | "--help" => display_usage(&program, 0),
                "-v" | "--version" => display_version(),
                _ => continue,
            }
        }
    }
}
