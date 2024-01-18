// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.

// Pour cet exo la, j'ai tout simplement suivi ce qu'il y a ecrit au dessus, je devais utiliser le module std::time et y implenter
// SystemTime and UNIX_EPOCH. Pour ce qui est de comment l'ecrire, je me suis refere sur des site internet avec des modules differents
// mais la logique reste la meme.

// TODO: Complete this use statement
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
