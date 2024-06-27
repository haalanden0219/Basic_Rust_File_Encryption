pub mod helpers;
extern crate rand // external crates

use rand::prelude::*;
use std::io::{Result , Read};
use std::fs::File;

//how do I run this within in the same cargo container?

fn main() {
    let filename : String = helpers::get_file_name();   
    
    match read_file_and_get_seed(&filename)  { //how do I get the string to return
        Ok(_) => println!("Successfully read from {}", filename), //Ok() 
        Err(e) => eprintln!("Error reading from {}: {}", filename , e) // Err()
    }
    println!("{}" , encrypted_string);
}
    // read file to get seed - will need to convert this into the right type

    //read file to get encrypted string??- I may return a massive string and decide to split it up


    //write to file with decrypted message - may use last function


fn read_file_and_get_seed (filename : String)  -> Result<()> { //lets first get the seed and then see if we can also get the decrypted messgae as well
    let mut file = File::open(filename)?; //question mark is apart of error checking
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(()) //meaning of this??


}