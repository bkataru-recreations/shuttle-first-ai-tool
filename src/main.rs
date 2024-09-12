use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

use csv::Reader;
use llm_chain::{executor, parameters, prompt, step::Step};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
}
