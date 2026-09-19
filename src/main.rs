use std::env;
use crate::main_basic::main_basic;

mod main_basic;

fn main() {
    let mut x = 5;
    x += 1;
    println!("x is {}", x);

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let nom_fichier=args[1].clone();
        println!("Premier argument : {}", nom_fichier);

        if nom_fichier.ends_with(".bas") {
            main_basic(nom_fichier);
        }
    } else {
        println!("Pas d'argument");
    }


}