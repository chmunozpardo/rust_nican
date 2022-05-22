extern crate bindgen;
extern crate gcc;

fn main() {
    println!("cargo:rustc-link-search=native=C:/Program Files (x86)/National Instruments/NI-CAN/MS Visual C/");
    println!("cargo:rustc-link-lib=static=nicanmsc");
}
