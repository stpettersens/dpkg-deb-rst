/*
  dpkg-deb implementation in Rust.
  Copyright 2016 Sam Saint-Pettersen.

  Released as original dpkg-deb under the
  GNU General Public License and additionally
  the MIT License; see GPL-LICENSE and MIT-LICENSE.
*/

extern crate tatar;
extern crate ark;
extern crate rustc_serialize;
use self::tatar::Tatar;
use self::ark::Ark;
use self::rustc_serialize::json::Json;

static DELIMITER: char = '_';

struct Package {
    package: String,
    version: String,
}

fn read_ctrl_file(control: &str) -> &str {
    control
}

fn create_ctrl_archive(pkg: &str) -> &str {
    pkg
}

pub fn build_debian_archive(src: &str, pn: &str, verbose: bool) {

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
