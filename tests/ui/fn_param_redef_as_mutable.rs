#![warn(clippy::fn_param_redef_as_mutable)]

fn foobar(a: Vec<bool>, b: u8) {
    let mut c = a;
    let d = b + 2;
}

fn main() {
}
