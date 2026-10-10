
mod main_basic;
mod parsing_basic;
mod exec_basic;

pub fn main_basic(fichier: String) {
    let mut x = 5;
    x += 1;
    println!("x2 is {}", x);
    println!("fichier: {}", fichier);
    main_basic::parse_basic_file(fichier).expect("Erreur pour lire le fichier");
}