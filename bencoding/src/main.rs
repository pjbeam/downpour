// The goal of this code is to correctly parse bencoded
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

fn number_builder(data_vector: &Vec<&str>, i: usize) -> usize {
    let mut num_str: String = String::new();
    let mut offset: usize = 0;
    while char_int_checker(data_vector[i + offset]) == true {
        num_str.push_str(data_vector[i]);
        offset += 1;
    }
    println!("{:?}", data_vector[i + offset]);
    let num = num_str.parse::<usize>().expect("Oops, I really need to learn how this actually works. :)");
    return num;

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
        //Skip spaces and newlines:
        if char_vec[i] == "" || char_vec[i] == "\n" {
            i += 1;
            continue;
        }
        if char_int_checker(char_vec[i]) == true {
            let i_offset = number_builder(&char_vec, i);
            print!("{:?}\n", i_offset);
            i += i_offset + 1;
            continue;
        }
        i += 1;
    }


}
