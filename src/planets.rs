use std::io::prelude::*;
use std::fs::File;
use rand::random;

// TODO: Capitalize the nouns, make function to abbreviate the planetname
pub fn planetname() -> String {
    // Open the file with the nounlist and put all lines in a vector
    let mut compfile = File::open("nounlist.txt").unwrap();
    let mut stringbuffer = String::new();
    compfile.read_to_string(&mut stringbuffer).unwrap();
    let linevec: Vec<&str> = stringbuffer.split('\n').collect();

    let le: usize = linevec.len();
    let mut name: String = "land of ".to_string();
    name.push_str(linevec[random::<usize>() % le]);
    name.push_str(" and ");
    name.push_str(linevec[random::<usize>() % le]);
    name
}
