// #![allow(unused)] // For beginning only.
use crate::prelude::*;

mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    let paths = utils::folder_walk::get_pdf_files("./input/")?;

    // print all paths
    for path in &paths {
        println!("{} ", path);
    }

    // merge all pdf files
    utils::merge_pdf::merge_pdf_files(paths, "./output/")?;

    Ok(())
}
