use libcnb;
use libcnb::build::GenericBuildContext;

use rust_cnb_starter::messages;
use std::collections::HashMap;

fn main() {
    libcnb::build::cnb_runtime_build(build);
}

fn build(context: GenericBuildContext) -> Result<(), std::io::Error> {
    println!("{}", messages::BUILD_MESSAGE);
    println!("App source @ {:?}", context.app_dir);

    Ok(())
}
