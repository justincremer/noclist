extern crate noclist;

use noclist::checksum;

fn main() {
    let token = "F47D071E3E94B0A6BCCC44B47CD8CA44";
    let path = "/user";
    let checksum = checksum(token, path);
}
