mod gen_constants;

fn main() {
    println!("cargo:rerun-if-changed=build");
    gen_constants::generate_constants().unwrap();
}
