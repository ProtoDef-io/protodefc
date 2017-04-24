extern crate peg;

fn main() {
    peg::cargo_build("src/frontend/protocol_spec/ast/pds.rustpeg");
}
