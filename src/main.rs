use std::env;

mod basic;



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let nom_fichier=args[1].clone();
        println!("Premier argument : {}", nom_fichier);

        if nom_fichier.to_lowercase().ends_with(".bas") {
            basic::main_basic(nom_fichier);
        }
    } else {
        println!("Pas d'argument");
    }


}