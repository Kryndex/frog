#[link(name     = "frog"
       , vers   = "0.0"
       , author = "Benjamin Roux"
       , uuid   = "D4B7F5F0-6B78-4DF0-9CB5-EB65DDB43857")];
#[crate_type = "lib"];
#[warn(non_camel_case_types)]

extern mod extra;

pub mod dna;
pub mod ga;
