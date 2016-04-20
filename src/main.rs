// A tool to auto-generate characters, worlds etc for SBURB-Sessions.
// Just for messing around a bit, mostly.
// May add functionality which adjusts factors based on player input and 
// more-or-less proven relations between them.
// Also, maybe show the player their colored costume if they wish so?
//
// For the planets, I'm going to need a pool of usable words to choose from as there
// isn't a given set from which to choose. The only rule seems to be that Space
// players get a land of x and Frogs. Of course, the other planets themes are also
// related to their (Class)spects, but it will probably be hard to generate fitting
// planet names.
extern crate rand;

use std::env;

mod classpects;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        if let Ok(n) = arg1.parse::<u32>(){
            if n < 13 && n > 0 {
                let v = classpects::n_classpects(n);
                for i in 0 .. v.len() { 
                    println!("Player {}: {}", i+1, v[i]);
                }
            } else { println!("Too many (or zero) players!"); }
        } else { println!("Please pass a number as first argument!"); }
    } else { println!("You must provide the number of players as argument!"); }
}
