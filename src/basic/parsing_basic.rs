use crate::basic::main_basic::AstInstruction::AstAffectation;
use crate::basic::main_basic::{AstExpression, AstInstruction, AstProgramme};
use std::cmp::{PartialEq, max};
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Clone)]
enum EtatLexer {
    Autre,
    Nombre,
    Mot,
    Separateur,
    ChaineDeCaracteres,
}

#[derive(Debug, Clone, PartialEq)]
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

    fn next_is_fin_variable(&self) -> bool {
        if let Some(c) = self.get_next(1) {
            if get_type_charactere(c) == TypeCharactere::Separateur {
                est_fin_variable(c)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn next_is_underscore(&self) -> bool {
        if let Some(c) = self.get_next(1) {
            if get_type_charactere(c) == TypeCharactere::Separateur {
                c == '_'
            } else {
                false
            }
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

#[derive(Debug)]
struct TokenIterator {
    liste_tokens: Vec<Token>,
    index: i32,
}

impl TokenIterator {
    fn new(liste: Vec<Token>) -> TokenIterator {
        TokenIterator {
            liste_tokens: liste.clone(),
            index: -1,
        }
    }

    fn get(&self) -> Option<Token> {
        if self.index + 1 >= 0 && self.index + 1 < self.liste_tokens.len() as i32 {
            Some(self.liste_tokens[(self.index + 1) as usize].clone())
        } else {
            None
        }
    }

    fn get_pos(&self, position: i32) -> Option<Token> {
        if self.index + 1 + position >= 0
            && self.index + 1 + position < self.liste_tokens.len() as i32
        {
            Some(self.liste_tokens[(self.index + 1 + position) as usize].clone())
        } else {
            None
        }
    }

    fn est_type_token(&self, type_token: Token) -> bool {
        if let Some(t) = self.get() {
            t == type_token
        } else {
            false
        }
    }

    fn est_identifiant(&self) -> bool {
        if let Some(Token::TokenIdentifiant(x)) = self.get() {
            true
        } else {
            false
        }
    }

    fn est_identifiant_pos(&self, position: i32) -> bool {
        if let Some(Token::TokenIdentifiant(x)) = self.get_pos(position) {
            true
        } else {
            false
        }
    }

    fn est_separateur_pos(&self, separateur: String, position: i32) -> bool {
        if let Some(Token::TokenSeparateur(x)) = self.get_pos(position) {
            x == separateur
        } else {
            false
        }
    }

    fn est_separateur(&self, separateur: String) -> bool {
        if let Some(Token::TokenSeparateur(x)) = self.get() {
            x == separateur
        } else {
            false
        }
    }

    fn est_nombre(&self) -> bool {
        if let Some(Token::TokenNombre(x)) = self.get() {
            true
        } else {
            false
        }
    }

    fn est_nombre_pos(&self, position: i32) -> bool {
        if let Some(Token::TokenNombre(x)) = self.get_pos(position) {
            true
        } else {
            false
        }
    }

    fn est_chaine(&self) -> bool {
        if let Some(Token::TokenChaine(x)) = self.get() {
            true
        } else {
            false
        }
    }

    fn est_chaine_pos(&self, position: i32) -> bool {
        if let Some(Token::TokenChaine(x)) = self.get_pos(position) {
            true
        } else {
            false
        }
    }

    fn len(&self) -> usize {
        if self.index < 0 {
            self.liste_tokens.len()
        } else {
            max(self.liste_tokens.len() - (self.index + 1) as usize, 0)
        }
    }
}

impl Iterator for TokenIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.index + 1 >= 0 && self.index + 1 < self.liste_tokens.len() as i32 {
            self.index += 1;
            let c = self.liste_tokens.get(self.index as usize).unwrap();
            Some(c.clone())
        } else {
            self.index = self.liste_tokens.len() as i32;
            None
        }
    }
}

pub fn parse_basic(contenu_fichier: String) -> std::io::Result<AstProgramme> {
    let liste_tokens: Vec<Vec<Token>> = lexer(contenu_fichier);

    println!("liste token : {:?}", liste_tokens);

    let programme = parsing_programme(liste_tokens);

    Ok(programme)
}

fn lexer(contenu_fichier: String) -> Vec<Vec<Token>> {
    let mut liste_tokens: Vec<Vec<Token>> = Vec::new();

    for ligne in contenu_fichier.lines() {
        let mut liste_tokens_ligne: Vec<Token> = Vec::new();

        let mut iter = CharIterator::new(ligne.to_string());

        while let Some(mot) = iter.next() {
            if iter.is_type_charactere(TypeCharactere::Nombre) {
                // nombre
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
                // séparateur
                let mut s = "".to_string();
                s.push(mot);
                let token = creation_token(s, 0, EtatLexer::Separateur);
                liste_tokens_ligne.push(token);
            } else if iter.is_type_charactere(TypeCharactere::Mot) {
                // identifiant
                let mut s = mot.to_string();

                while iter.next_is_type_charactere(1, TypeCharactere::Mot)
                    || iter.next_is_fin_variable()
                    || iter.next_is_type_charactere(1, TypeCharactere::Nombre)
                    || iter.next_is_underscore()
                {
                    if let Some(mot2) = iter.next() {
                        s = s + mot2.to_string().as_str();
                        if est_fin_variable(mot2) {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                let token = creation_token(s, 0, EtatLexer::Mot);
                liste_tokens_ligne.push(token);
            } else if iter.is_type_charactere(TypeCharactere::Guillemet) {
                // chaine de caractère
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
    liste_tokens
}

fn est_fin_variable(c: char) -> bool {
    c == '$' || c == '%' || c == '&' || c == '!' || c == '#'
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

        let mut token_iterator = TokenIterator::new(ligne);

        if token_iterator.est_identifiant() && token_iterator.est_separateur_pos("=".to_string(), 1)
        {
            // affectation
            if let Token::TokenIdentifiant(ident) = token_iterator.next().unwrap() {
                println!("affectation {}", ident);
                token_iterator.next();
                let exp = parsing_expression(&mut token_iterator);
                let instruction = AstAffectation(ident.clone(), exp.0);
                programme.liste_instructions.push(instruction);
            } else {
                panic!("token {:?} n'est pas un identifiant", token_iterator.get());
            }
        } else if token_iterator.est_identifiant() {
            // appel de méthode
            if let Token::TokenIdentifiant(ident) = token_iterator.next().unwrap() {
                println!("appel {}", ident);
                // let mut ligne_restant = ligne[1..].to_vec();
                let mut liste_expressions: Vec<AstExpression> = vec![];
                let mut separateur_virgule_liste: Vec<bool> = vec![];
                loop {
                    // let len = ligne_restant.len();
                    let exp = parsing_expression(&mut token_iterator);
                    liste_expressions.push(exp.0);
                    if token_iterator.len() > 0
                        && separateur_parametre(&ident, token_iterator.get().unwrap())
                    {
                        let mut separateur_virgule = true;
                        if let Some(Token::TokenSeparateur(s)) = token_iterator.next() {
                            separateur_virgule = s == ",";
                        } else {
                            panic!(
                                "token {:?} n'est pas un séparateur",
                                token_iterator.get().unwrap()
                            );
                        };
                        // let n = exp.1 as usize;
                        separateur_virgule_liste.push(separateur_virgule);
                        // ligne_restant = ligne_restant[n + 1..].to_vec();
                    } else {
                        break;
                    }
                }

                let instruction = AstInstruction::AstAppelMethode(
                    ident,
                    liste_expressions,
                    separateur_virgule_liste,
                );

                programme.liste_instructions.push(instruction);
            } else {
                panic!("token {:?} n'est pas un identifiant", token_iterator.get());
            }
        } else {
            panic!("instruction inconnue");
        }
    }

    programme
}

fn separateur_parametre(nom_methode: &String, mot: Token) -> bool {
    if nom_methode.to_lowercase() == "print" {
        est_separateur(&mot, ",".to_string()) || est_separateur(&mot, ";".to_string())
    } else {
        est_separateur(&mot, ",".to_string())
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

fn parsing_expression(mut liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    if liste_tokens.len() == 3
        && (liste_tokens.est_identifiant()
            || liste_tokens.est_nombre()
            || liste_tokens.est_chaine())
        && (liste_tokens.est_separateur_pos("+".to_string(), 1)
            || liste_tokens.est_separateur_pos("-".to_string(), 1)
            || liste_tokens.est_separateur_pos("*".to_string(), 1)
            || liste_tokens.est_separateur_pos("/".to_string(), 1))
        && (liste_tokens.est_identifiant_pos(2)
            || liste_tokens.est_nombre_pos(2)
            || liste_tokens.est_chaine_pos(2))
    {
        let token1 = liste_tokens.next().unwrap();
        if let Token::TokenSeparateur(separateur) = liste_tokens.next().unwrap() {
            let mut nb_token: u32 = 0;
            let expr1: AstExpression;
            let expr2: AstExpression;
            let mut expr_list: Vec<Token> = Vec::new();
            expr_list.push(token1);
            let mut token_iter = TokenIterator::new(expr_list);
            let res = parsing_expression(&mut token_iter);
            expr1 = res.0;
            nb_token += res.1;

            nb_token += 1;

            let mut expr_list2: Vec<Token> = Vec::new();
            expr_list2.push(liste_tokens.next().unwrap());
            let mut token_iter2 = TokenIterator::new(expr_list2);
            let res = parsing_expression(&mut token_iter2);
            expr2 = res.0;
            nb_token += res.1;

            (
                AstExpression::AstOperateurBinaire(
                    separateur.clone(),
                    Rc::new(expr1),
                    Rc::new(expr2),
                ),
                nb_token,
            )
        } else {
            panic!("Invalid separator token: {:?}", liste_tokens.get());
        }
    } else if liste_tokens.est_separateur("(".to_string()) {
        let mut nb_token: u32 = 0;
        nb_token += 1;
        liste_tokens.next();
        let res = parsing_expression(liste_tokens);
        nb_token += res.1;
        if liste_tokens.est_separateur(")".to_string()) {
            nb_token += 1;
            return (res.0, nb_token);
        } else {
            panic!("parenthese manquante: {:?}", liste_tokens);
        }
    } else if liste_tokens.est_identifiant() {
        if let Token::TokenIdentifiant(ident) = liste_tokens.next().unwrap() {
            return (AstExpression::AstIdentifiant(ident.clone()), 1);
        } else {
            panic!("identifiant inconnu");
        }
    } else if liste_tokens.est_nombre() {
        if let Token::TokenNombre(nombre) = liste_tokens.next().unwrap() {
            return (AstExpression::AstNombre(nombre), 1);
        } else {
            panic!("nombre inconnu: {:?}", liste_tokens.get());
        }
    } else if liste_tokens.est_chaine() {
        if let Token::TokenChaine(chaine) = liste_tokens.next().unwrap() {
            return (AstExpression::AstChaine(chaine.clone()), 1);
        } else {
            panic!("chaine inconnue: {:?}", liste_tokens.get());
        }
    } else {
        panic!("expression inconnue: {:?}", liste_tokens);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_lexer_nombre() {
        let fichier = "123".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(liste_tokens, vec![vec![Token::TokenNombre(123)]]);
    }

    #[test]
    fn test_lexer_identifiant() {
        let fichier = "abc".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenIdentifiant("abc".to_string())]]
        );
    }

    #[test]
    fn test_lexer_identifiant2() {
        let fichier = "abc$".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenIdentifiant("abc$".to_string())]]
        );
    }

    #[test]
    fn test_lexer_identifiant3() {
        let fichier = "toto tata% 96325".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![
                Token::TokenIdentifiant("toto".to_string()),
                Token::TokenIdentifiant("tata%".to_string()),
                Token::TokenNombre(96325)
            ]]
        );
    }

    #[test]
    fn test_lexer_identifiant4() {
        let fichier = "azertyu123_bbb".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenIdentifiant("azertyu123_bbb".to_string())]]
        );
    }

    #[test]
    fn test_lexer_separateur() {
        let fichier = "2+3".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![
                Token::TokenNombre(2),
                Token::TokenSeparateur("+".to_string()),
                Token::TokenNombre(3)
            ]]
        );
    }

    #[test]
    fn test_lexer_guillemet() {
        let fichier = "\"bbb\"".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenChaine("bbb".to_string()),]]
        );
    }
}
