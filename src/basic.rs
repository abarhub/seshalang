
mod main_basic;
mod parsing_basic;
mod exec_basic;
mod lexer_basic;
mod fonction_basic;
mod struct_basic;

pub fn main_basic(fichier: String) {
    main_basic::parse_basic_file(fichier).expect("Erreur pour lire le fichier");
}