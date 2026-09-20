use crate::main_basic::AstInstruction::AstAffectation;
use crate::main_basic::{AstExpression, AstInstruction, AstProgramme};
use std::cmp::PartialEq;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Clone)]
enum EtatLexer {
    Autre,
    Nombre,
    Mot,
    Separateur,
    ChaineDeCaracteres,
}

#[derive(Debug, Clone)]
enum Token {
    TokenNombre(u32),
    TokenIdentifiant(String),
    TokenSeparateur(String),
    TokenMotReserve(String),
    TokenChaine(String),
}

#[derive(Debug, Clone, PartialEq)]
enum TypeCharactere {
    Nombre,
    Mot,
    Separateur,
    Ignorable,
    Guillemet,
}

struct CharIterator {
    texte: String,
    index: i32,
}

impl CharIterator {
    fn new(s: String) -> CharIterator {
        CharIterator {
            texte: s,
            index: -1,
        }
    }

    fn get(&self) -> Option<char> {
        if self.index < 0 {
            None
        } else {
            self.texte.chars().nth(self.index as usize)
        }
    }

    fn get_next(&self, pos: usize) -> Option<char> {
        if self.index < 0 || self.index + pos as i32 >= self.texte.len() as i32 {
            None
        } else {
            self.texte.chars().nth(self.index as usize + pos)
        }
    }

    fn is_type_charactere(&self, type_charactere: TypeCharactere) -> bool {
        if let Some(c) = self.get() {
            get_type_charactere(c) == type_charactere
        } else {
            false
        }
    }

    fn next_is_type_charactere(&self, pos: usize, type_charactere: TypeCharactere) -> bool {
        if let Some(c) = self.get_next(pos) {
            get_type_charactere(c) == type_charactere
        } else {
            false
        }
    }
}

impl Iterator for CharIterator {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.index + 1 >= 0 && self.index + 1 < self.texte.len() as i32 {
            self.index += 1;
            let c = self.texte.chars().nth(self.index as usize).unwrap();
            Some(c)
        } else {
            self.index = self.texte.len() as i32;
            None
        }
    }
}

pub fn parse_basic(contenu_fichier: String) -> std::io::Result<AstProgramme> {
    let mut liste_tokens: Vec<Vec<Token>> = Vec::new();

    for ligne in contenu_fichier.lines() {
        let mut liste_tokens_ligne: Vec<Token> = Vec::new();

        let mut iter = CharIterator::new(ligne.to_string());

        while let Some(mot) = iter.next() {
            if iter.is_type_charactere(TypeCharactere::Nombre) {
                let mut nombre = mot.to_digit(10).unwrap();

                while iter.next_is_type_charactere(1, TypeCharactere::Nombre) {
                    if let Some(mot2) = iter.next() {
                        nombre = nombre * 10 + mot2.to_digit(10).unwrap();
                    } else {
                        break;
                    }
                }

                let token = creation_token("".to_string(), nombre, EtatLexer::Nombre);
                liste_tokens_ligne.push(token);
            } else if iter.is_type_charactere(TypeCharactere::Separateur) {
                let mut s = "".to_string();
                s.push(mot);
                let token = creation_token(s, 0, EtatLexer::Separateur);
                liste_tokens_ligne.push(token);
            } else if iter.is_type_charactere(TypeCharactere::Mot) {
                let mut s = mot.to_string();

                while iter.next_is_type_charactere(1, TypeCharactere::Mot) {
                    if let Some(mot2) = iter.next() {
                        s = s + mot2.to_string().as_str();
                    } else {
                        break;
                    }
                }

                let token = creation_token(s, 0, EtatLexer::Mot);
                liste_tokens_ligne.push(token);
            } else if iter.is_type_charactere(TypeCharactere::Guillemet) {
                let mut s = "".to_string();
                while !iter.next_is_type_charactere(1, TypeCharactere::Guillemet) {
                    if let Some(mot2) = iter.next() {
                        s = s + mot2.to_string().as_str();
                    } else {
                        break;
                    }
                }
                if iter.next_is_type_charactere(1, TypeCharactere::Guillemet) {
                    iter.next();
                }
                let token = creation_token(s, 0, EtatLexer::ChaineDeCaracteres);
                liste_tokens_ligne.push(token);
            } else {
                // ignoré
                if mot == ' ' || mot == '\n' {
                    // caractères espaces
                } else {
                    println!("caractere ignore: {}", mot)
                }
            }
        }

        if liste_tokens_ligne.len() > 0 {
            liste_tokens.push(liste_tokens_ligne);
        }
    }

    println!("liste token : {:?}", liste_tokens);

    let programme = parsing_programme(liste_tokens);

    Ok(programme)
}

fn get_type_charactere(mot: char) -> TypeCharactere {
    if mot.is_ascii_digit() {
        TypeCharactere::Nombre
    } else if mot == ' ' {
        TypeCharactere::Ignorable
    } else if mot == '+'
        || mot == '-'
        || mot == '*'
        || mot == '/'
        || mot == '='
        || mot == '('
        || mot == ')'
        || mot == ','
        || mot == ';'
    {
        TypeCharactere::Separateur
    } else if mot.is_ascii_alphabetic()
        || mot == '_'
        || mot == '$'
        || mot == '!'
        || mot == '%'
        || mot == '#'
        || mot == '&'
    {
        TypeCharactere::Mot
    } else if mot == '"' {
        TypeCharactere::Guillemet
    } else {
        TypeCharactere::Ignorable
    }
}

fn creation_token(s: String, nombre: u32, etat: EtatLexer) -> Token {
    match etat {
        EtatLexer::Nombre => Token::TokenNombre(nombre),
        EtatLexer::Mot => Token::TokenIdentifiant(s),
        EtatLexer::Separateur => Token::TokenSeparateur(s),
        EtatLexer::ChaineDeCaracteres => Token::TokenChaine(s),
        _ => panic!("Etat invalide"),
    }
}

fn parsing_programme(liste_tokens: Vec<Vec<Token>>) -> AstProgramme {
    let mut programme = AstProgramme {
        liste_instructions: Vec::new(),
    };
    for ligne in liste_tokens {
        if ligne.len() == 0 {
            continue;
        }

        if ligne.len() > 1
            && est_identifiant(&ligne[0])
            && est_separateur(&ligne[1], "=".to_string())
        {
            // affectation
            if let Token::TokenIdentifiant(ident) = ligne[0].clone() {
                println!("affectation {}", ident);
                let exp = parsing_expression(ligne[2..].to_vec());
                let instruction = AstAffectation(ident.clone(), exp.0);
                programme.liste_instructions.push(instruction);
            } else {
                eprintln!("token {:?} n'est pas un identifiant", ligne[0]);
                panic!("token {:?} n'est pas un identifiant", ligne[0]);
            }
        } else if ligne.len() >= 1 && est_identifiant(&ligne[0]) {
            // appel de méthode
            if let Token::TokenIdentifiant(ident) = ligne[0].clone() {
                println!("appel {}", ident);
                let mut ligne_restant = ligne[1..].to_vec();
                let mut liste_expressions: Vec<AstExpression> = vec![];
                let mut separateur_virgule: Vec<bool> = vec![];
                loop {
                    let len = ligne_restant.len();
                    let exp = parsing_expression(ligne_restant.clone());
                    liste_expressions.push(exp.0);
                    if exp.1 + 1 < len as u32
                        && separateur_parametre(&ident, &ligne_restant[exp.1 as usize])
                    {
                        let n = exp.1 as usize;
                        separateur_virgule.push(est_separateur(
                            &ligne_restant[exp.1 as usize],
                            ",".to_string(),
                        ));
                        ligne_restant = ligne_restant[n + 1..].to_vec();
                    } else {
                        break;
                    }
                }

                let instruction =
                    AstInstruction::AstAppelMethode(ident, liste_expressions, separateur_virgule);

                programme.liste_instructions.push(instruction);
            } else {
                eprintln!("token {:?} n'est pas un identifiant", ligne[0]);
                panic!("token {:?} n'est pas un identifiant", ligne[0]);
            }
        } else {
            eprintln!("instruction inconnue: {:?}", ligne);
            panic!("instruction inconnue");
        }
    }

    programme
}

fn separateur_parametre(nom_methode: &String, mot: &Token) -> bool {
    if nom_methode.to_lowercase() == "print" {
        est_separateur(mot, ",".to_string()) || est_separateur(mot, ";".to_string())
    } else {
        est_separateur(mot, ",".to_string())
    }
}

fn est_identifiant(token: &Token) -> bool {
    match token {
        Token::TokenIdentifiant(_) => true,
        _ => false,
    }
}

fn est_nombre(token: &Token) -> bool {
    match token {
        Token::TokenNombre(_) => true,
        _ => false,
    }
}

fn est_separateur(token: &Token, separateur_cherche: String) -> bool {
    match token {
        Token::TokenSeparateur(separateur) => separateur == &separateur_cherche,
        _ => false,
    }
}

fn est_chaine(token: &Token) -> bool {
    match token {
        Token::TokenChaine(_) => true,
        _ => false,
    }
}

fn parsing_expression(liste_tokens: Vec<Token>) -> (AstExpression, u32) {
    if liste_tokens.len() == 1 && est_identifiant(&liste_tokens[0]) {
        if let Token::TokenIdentifiant(ident) = &liste_tokens[0] {
            (AstExpression::AstIdentifiant(ident.clone()), 1)
        } else {
            panic!(
                "Token {:?} n'est pas un identifiant",
                liste_tokens[0].clone()
            );
        }
    } else if liste_tokens.len() == 1 && est_nombre(&liste_tokens[0]) {
        if let Token::TokenNombre(nombre) = liste_tokens[0] {
            return (AstExpression::AstNombre(nombre), 1);
        } else {
            panic!("Token {:?} n'est pas un nombre", liste_tokens[0].clone());
        }
    } else if liste_tokens.len() == 3
        && (est_identifiant(&liste_tokens[0])
            || est_nombre(&liste_tokens[0])
            || est_chaine(&liste_tokens[0]))
        && (est_separateur(&liste_tokens[1], "+".to_string())
            || est_separateur(&liste_tokens[1], "-".to_string())
            || est_separateur(&liste_tokens[1], "*".to_string())
            || est_separateur(&liste_tokens[1], "/".to_string()))
        && (est_identifiant(&liste_tokens[2])
            || est_nombre(&liste_tokens[2])
            || est_chaine(&liste_tokens[2]))
    {
        if let Token::TokenSeparateur(separateur) = &liste_tokens[1] {
            let mut nb_token: u32 = 0;
            let expr1: AstExpression;
            let expr2: AstExpression;
            let mut expr_list: Vec<Token> = Vec::new();
            expr_list.push(liste_tokens[0].clone());
            let res = parsing_expression(expr_list);
            expr1 = res.0;
            nb_token += res.1;

            nb_token += 1;

            let mut expr_list2: Vec<Token> = Vec::new();
            expr_list2.push(liste_tokens[2].clone());
            let res = parsing_expression(expr_list2);
            expr2 = res.0;
            nb_token += res.1;

            return (
                AstExpression::AstOperateurBinaire(
                    separateur.clone(),
                    Rc::new(expr1),
                    Rc::new(expr2),
                ),
                nb_token,
            );
        } else {
            eprintln!("Invalid separator token: {:?}", liste_tokens[1]);
            panic!("Invalid separator token");
        }
    } else if liste_tokens.len() > 0 && est_separateur(&liste_tokens[0], "(".to_string()) {
        let mut nb_token: u32 = 0;
        nb_token += 1;
        let res = parsing_expression(liste_tokens[1..].to_vec());
        nb_token += res.1;
        if est_separateur(&liste_tokens[nb_token as usize], ")".to_string()) {
            nb_token += 1;
            return (res.0, nb_token);
        } else {
            eprintln!("parenthese manquante: {:?}", liste_tokens);
            panic!("parenthese manquante");
        }
    } else if liste_tokens.len() > 0 && est_identifiant(&liste_tokens[0]) {
        if let Token::TokenIdentifiant(ident) = &liste_tokens[0] {
            return (AstExpression::AstIdentifiant(ident.clone()), 1);
        } else {
            panic!("identifiant inconnu");
        }
    } else if liste_tokens.len() > 0 && est_nombre(&liste_tokens[0]) {
        if let Token::TokenNombre(nombre) = &liste_tokens[0] {
            return (AstExpression::AstNombre(*nombre), 1);
        } else {
            panic!("nombre inconnu: {:?}", liste_tokens[0]);
        }
    } else if liste_tokens.len() > 0 && est_chaine(&liste_tokens[0]) {
        if let Token::TokenChaine(chaine) = &liste_tokens[0] {
            return (AstExpression::AstChaine(chaine.clone()), 1);
        } else {
            panic!("chaine inconnue: {:?}", liste_tokens[0]);
        }
    } else {
        panic!("expression inconnue: {:?}", liste_tokens);
    }
}
