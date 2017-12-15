// JSON to GPX
// Rewrite of my python command line program in Rust
// Takes in JSON either directly or via URL and
// creates GPX files.

// This allows you to use external macros
#[macro_use]
extern crate clap;

use clap::{ Arg, App };

fn main() {

    let matches = App::new( "JSON to GPX" )
        .author( crate_authors!( "\n" ) )
        .version( crate_version!() )
        .about( "Creates a GPX file from a JSON array or URL to a JSON array." )
        .arg(Arg::with_name("url")
             .help("url to json array")
             .index(1)
             .required(true))
        .get_matches();

    println!("Doing real work with file: {}", matches.value_of("url").unwrap() );

    
}
