use anyhow::Result;
use crate::config::NewInputOptions;
use std::env;


pub fn new_pallet(directory: &str, input_option: NewInputOptions) -> Result<()> {
    let crate_pallet_path =env::current_dir()?.join(directory);
    println!("{:?} \n", crate_pallet_path);
    println!("{:?}", input_option);
    Ok(())
}