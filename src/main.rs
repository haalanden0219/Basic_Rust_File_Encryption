use rand::prelude::*; 
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::{Write , Result , stdin};

fn main() {
    let filename : String = get_file_name();
    let user_input : String = get_input();
    let user_input_length = user_input.len();

    
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH);
    let in_s = since_the_epoch.clone().expect("REASON").as_secs() as u64;
    let seed = StdRng::seed_from_u64(in_s); 

    let in_s_string = in_s.to_string();

    let encrypted_string = get_seed(user_input_length.try_into().unwrap() , user_input ,seed);

    match write_to_file(&filename , &in_s_string , &encrypted_string)  { //U an borrowing ownership to use it in another function.
        Ok(_) => println!("Successfully wrote to {}", filename),
        Err(e) => eprintln!("Error writing to {}: {}", filename , e)
    }
    println!("{}" , encrypted_string);
}

fn get_file_name() -> String {
    let mut filename = String::new();
    println!("Enter Name of Encrypted File: ");
    stdin().read_line(&mut filename).expect("Failed to readline"); 
    let filename_stripped = filename.split('\n').collect();
    filename_stripped //need to slice off the \n

}

fn get_input() -> String {
    let mut input = String::new(); 
    println!("Enter a Message to Encrypt: ");
    stdin().read_line(&mut input).expect("Failed to readline"); 
    input
}

fn get_seed(input_length : u32 , user_input : String, mut seed : StdRng ) -> String {
    
    let mut encrypted_string = String::new();

    for i in 0..input_length {
        let random_number = seed.gen_range(1..1000); 
        let input_char = user_input.chars().nth(i as usize).expect("Index out of bounds");
        let encrypted_char = (random_number ^ input_char as u32) as u8 as char; //why is u8 commonly used with char?

        encrypted_string.push(encrypted_char);
    }
    
    encrypted_string
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