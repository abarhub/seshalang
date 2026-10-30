use std::env;

mod basic;

const DEBUG_PRINT: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let nom_fichier = args[1].clone();
        if DEBUG_PRINT {
            println!("Premier argument : {}", nom_fichier);
        }
        if nom_fichier.to_lowercase().ends_with(".bas") {
            basic::main_basic(nom_fichier);
        }
    } else {
        println!("Pas d'argument");
    }
}
