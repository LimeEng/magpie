use std::{env, fs::File, io::Write, path::Path};

#[allow(dead_code)]
mod common;
mod shift_rays;

pub fn generate_constants() -> std::io::Result<()> {
    let common = include_str!("./common.rs").to_string();
    let shift_rays = shift_rays::generate();

    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("gen.rs");
    let mut output = File::create(&path).unwrap();

    writeln!(&mut output, "{}", common)?;
    writeln!(&mut output, "{}", shift_rays)?;
    Ok(())
}
