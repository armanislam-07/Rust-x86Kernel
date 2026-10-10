use std::{env, path::PathBuf};

fn main()
{
    let kernel = PathBuf::from(env::var_os("CARGO_BIN_FILE_H123_h123").unwrap(),);
 let image = PathBuf::from(env::var_os("OUT_DIR").unwrap())
        .join("kernel-bios.img");

    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&image)
        .unwrap();

    println!("cargo:rustc-env=KERNEL_IMAGE={}", image.display());
}
