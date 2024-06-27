use rand::Rng;
use rand::prelude::*; 
use std::io::stdin;

//create data type for cipher text data
//this can store the cipher text, epoch, key
//implement a parse/decode.

pub fn get_seed(input_length : u32 , user_input : String, mut seed : StdRng ) -> String { //this will be reused again.
    
    let mut encrypted_string = String::new();

    for i in 0..input_length {
        let random_number = seed.gen_range(1..1000); 
        let input_char = user_input.chars().nth(i as usize).expect("Index out of bounds");
        let encrypted_char = (random_number ^ input_char as u32) as u8 as char; //why is u8 commonly used with char?

        encrypted_string.push(encrypted_char);
    }
    
    encrypted_string
}

pub fn get_file_name() -> String { 
    let mut filename = String::new();
    println!("Enter Name of Encrypted File: ");
    stdin().read_line(&mut filename).expect("Failed to readline"); 
    let filename_stripped = filename.split('\n').collect();
    filename_stripped 

}