// JSON to GPX
// Rewrite of my python command line program in Rust
// Takes in JSON either directly or via URL and
// creates GPX files.

// This file is all our http actions

mod http {

    pub fn get( url: &str ) -> Result< &str, Err > {
        Ok()
    }
}
