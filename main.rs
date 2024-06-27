pub mod helpers;
use rand::prelude::*; 
use std::env::args;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::{Write , Result , stdin};

const DECRYPT_FLAG : &str = "decrypt";

fn main() {
    let args : Vec<String> = args().collect(); //args is a vector of strings
    let query= &args[1];
    
    let filename : String = helpers::get_file_name();
    let user_input : String = get_input();
    let user_input_length = user_input.len();
    
    match query.as_str() {

        DECRYPT_FLAG  => println!("Going to decrypt"),

        //get length of encrypted message,
        //unicode for the epoch time as a numerical value (more efficient), this will be a set size.
        _ => { println!("Going to decrypt");
        let start = SystemTime::now();
        let since_the_epoch = start
        .duration_since(UNIX_EPOCH);
        let in_s = since_the_epoch.clone().expect("REASON").as_secs() as u64; //look into usize
        let seed = StdRng::seed_from_u64(in_s); 

        let in_s_string = in_s.to_string();

        let encrypted_string = helpers::get_seed(user_input_length.try_into().unwrap() , user_input ,seed);

        match write_to_file(&filename , &in_s_string , &encrypted_string)  { //Use result type for the error checking 
            Ok(_) => println!("Successfully wrote to {}", filename), 
            Err(e) => eprintln!("Error writing to {}: {}", filename , e) 
        }
        println!("{}" , encrypted_string);   
        }

    }

}



fn get_input() -> String {
    let mut input = String::new(); 
    println!("Enter a Message to Encrypt: ");
    stdin().read_line(&mut input).expect("Failed to readline"); 
    input
}



fn write_to_file (filename : &str , timestamp : &str, secret_message : &str) -> Result<()> { //what is with the return type? Note: I needed the specific library for this.
    let mut f = File::create(filename)?;
    f.write_all(timestamp.as_bytes())?; //what is witht the question mark in the syntax?
    f.write_all(b"\n")?;
    f.write_all(secret_message.as_bytes())?;

    f.flush()?;

    f.sync_data()?;
    Ok(())

}