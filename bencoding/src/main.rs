// The goal of this code is to correctly decode bencoded
// input. How this will actually be integrated into
// the large effort is TBD, however for the purpose
// of getting started the program will take a bencoded
// file and return decoded output either to stdout or
// to an outfile.
//
// bencoding:
// https://wiki.theory.org/BitTorrentSpecification#Bencoding

use std::env;
use std::fs;
use std::str;

fn char_int_checker(char: &str) -> bool {
    return char.parse::<u8>().is_ok();
}

fn length_builder(data_vector: &Vec<&str>, i: usize) -> (usize, usize) { //first element is string length, second is offset
    let mut num_str: String = String::new();
    let mut offset: usize = 0;
    while char_int_checker(data_vector[i + offset]) == true {
        num_str.push_str(data_vector[i + offset]);
        offset += 1;
    }
    let num = num_str.parse::<usize>().expect("Oops, I really need to learn how this actually works. :)");
    return (num, num + num_str.len() + 1);
}

fn integer_validator(num_str: String) -> bool {
    //Validate integer here
    return false;
}

fn integer_extractor(data_vector: &Vec<&str>, i: usize) -> (i64, usize) {
    let mut offset: usize = 0; //offset is 0 here because i increments in the match block before this function is called
    let mut num_str: String = String::new();

    while data_vector[i + offset] != "e" {
        num_str.push_str(data_vector[i + offset]);
        offset += 1;
    }
    //TODO: validate integers
    let num = num_str.parse::<i64>().expect("Really, handle errors like you aren't a child.");
    return (num, offset + 1);
}

fn string_extractor(data_vector: &Vec<&str>, i: usize, j:usize) -> String {
    let mut offset: usize = 1;
    let mut val_str: String = String::new();

    while i + offset < j {
        val_str.push_str(data_vector[i + offset]);
        offset += 1;
    }

    return val_str;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    println!("Opening file {}...", file_name);

    //TODO: Properly handle things like errors, etc.... lmao
    let file_contents = fs::read(file_name)
        .expect("Failed to read the file.");

    //TODO: Properly handle the raw bytes of the pieces field.
    //For now just get going on the pretty stuff at the start.
    let lossy_contents = String::from_utf8_lossy(&file_contents);
    let char_vec = lossy_contents.split("").collect::<Vec<&str>>();
    //Use while loop because the index will shift during decoding
    let mut i: usize = 0;
    while i <= char_vec.len() - 1 {
        match char_vec[i] {
            "d" => {
                //Dictionary logic goes here
                i += 1;
                continue;
            },
            "i" => {
                //integer logic goes here
                i += 1;
                let int_and_offset = integer_extractor(&char_vec, i);
                println!("Extracted integer value is: {:?}", int_and_offset.0);
                i += int_and_offset.1;
                continue;
            },
            "l" => {
                //list logic goes here
                i += 1;
                continue;
            },
            "e" => {
                //the top of stack component is complete
                println!("yes {:?}", i);
                i += 1;
                continue;
            },
            _ => {
                if char_int_checker(char_vec[i]) == true {
                    let extracted_length = length_builder(&char_vec, i);
                    let value_length = extracted_length.0;
                    let i_offset = extracted_length.1;
                    let value_string = string_extractor(&char_vec, i + 1, i + i_offset);
                    println!("string value is: {:?}", value_string);
                    i += i_offset;
                    continue;
                }
                i += 1;
            }
        }
    }


}
