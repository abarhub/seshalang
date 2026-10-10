use std::env;

mod basic;



fn main() {
    let mut x = 5;
    x += 1;
    println!("x is {}", x);

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let nom_fichier=args[1].clone();
        println!("Premier argument : {}", nom_fichier);

        if nom_fichier.ends_with(".bas") {
            basic::main_basic(nom_fichier);
        }
    } else {
        println!("Pas d'argument");
    }


}