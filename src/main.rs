extern crate rand;

use std::env;

mod classpects;
mod planets;

fn main() {
    //if let Some(arg1) = env::args().nth(1) {
        //if let Ok(n) = arg1.parse::<u32>(){
            //if n < 13 && n > 0 {
                //let v = classpects::n_classpects(n);
                //for i in 0 .. v.len() { 
                    //println!("Player {}: {}", i+1, v[i]);
                //}
            //} else { println!("Too many (or zero) players!"); }
        //} else { println!("Please pass a number as first argument!"); }
    //} else { println!("You must provide the number of players as argument!"); }
    println!("Your planets name is {}", planets::planetname());
}
