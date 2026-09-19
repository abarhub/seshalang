use std::fs;

#[derive(Debug)]
enum TypeToken {
    Nombre,
    MotReserve,
    Identifiant,
    Separateur,
}

#[derive(Debug)]
struct Token {
    texte: String,
    nombre: u32,
    type_token: TypeToken,
}

const ETAT_AUTRE: i32 = 0;
const ETAT_NOMBRE: i32 = 1;
const ETAT_MOT: i32 = 2;
const ETAT_SEPARATEUR: i32 = 3;

pub fn main_basic(fichier: String) {
    let mut x = 5;
    x += 1;
    println!("x2 is {}", x);
    println!("fichier: {}", fichier);
    parse_basic(fichier).expect("Erreur pour lire le fichier");
}

fn parse_basic(fichier: String) -> std::io::Result<()> {
    let contenu = fs::read_to_string(fichier)?;

    let mut liste_tokens: Vec<Vec<Token>> = Vec::new();

    for ligne in contenu.lines() {
        let mut s = String::new();
        let mut nombre = 0;
        let mut etat = ETAT_AUTRE;

        let mut liste_tokens_ligne: Vec<Token> = Vec::new();

        for mot in ligne.chars() {
            if mot.is_numeric() {
                let n = mot.to_digit(10).unwrap();
                if etat != ETAT_NOMBRE {
                    if etat != ETAT_AUTRE {
                        let token = creationToken(s.clone(), nombre, etat);
                        liste_tokens_ligne.push(token);
                        s = "".to_string();
                        nombre = 0;
                    }
                    etat = ETAT_NOMBRE;
                    nombre = n;
                } else {
                    nombre = nombre * 10 + n;
                }
            } else if mot == ' ' {
                etat = ETAT_AUTRE;
            } else if mot == '+'
                || mot == '-'
                || mot == '*'
                || mot == '/'
                || mot == '='
                || mot == '('
                || mot == ')'
            {
                if etat != ETAT_AUTRE {
                    let token = creationToken(s.clone(), nombre, etat);
                    liste_tokens_ligne.push(token);
                    s = "".to_string();
                    nombre = 0;
                }
                etat = ETAT_SEPARATEUR;
                s = "".to_string();
                s.push(mot);
            } else if mot.is_alphabetic() || mot == '_' {
                if etat == ETAT_MOT {
                    s.push(mot);
                } else {
                    if etat != ETAT_AUTRE {
                        let token = creationToken(s.clone(), nombre, etat);
                        liste_tokens_ligne.push(token);
                        s = "".to_string();
                        nombre = 0;
                    }

                    etat = ETAT_MOT;
                    s = "".to_string();
                    s.push(mot);
                }
            } else {
                // ignoré
                println!("caractere ignore: {}", mot)
            }
        }

        if etat != ETAT_AUTRE {
            let token = creationToken(s.clone(), nombre, etat);
            liste_tokens_ligne.push(token);
        }

        if liste_tokens_ligne.len() > 0 {
            liste_tokens.push(liste_tokens_ligne);
        }
    }

    println!("liste token : {:?}", liste_tokens);

    Ok(())
}

fn creationToken(s: String, nombre: u32, etat: i32) -> Token {
    match etat {
        ETAT_NOMBRE => Token {
            texte: "".to_string(),
            nombre: nombre,
            type_token: TypeToken::Nombre,
        },
        ETAT_MOT => Token {
            texte: s,
            nombre: 0,
            type_token: TypeToken::Identifiant,
        },
        ETAT_SEPARATEUR => Token {
            texte: s,
            nombre: 0,
            type_token: TypeToken::Separateur,
        },
        _ => panic!("Etat invalide"),
    }
}

fn ajouteToken() {}
