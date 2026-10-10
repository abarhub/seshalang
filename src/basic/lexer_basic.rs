#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeMotCle {
    If,
    Else,
    For,
    Next,
    Goto,
    Print,
    Mod,
    Not,
    And,
    Or,
    Xor,
    Eqv,
    Imp,
    Const,
    Let,
    End,
    Then,
    Select,
    Case,
    ElseIf,
    Dim,
    As,
    Integer,
    String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum EtatLexer {
    Autre,
    Nombre,
    Mot,
    Separateur,
    ChaineDeCaracteres,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeSeparateur {
    EQU,
    DIFF,
    SUP,
    SUPE,
    INF,
    INFE,
    PLUS,
    MOINS,
    FOIS,
    DIVR,
    DIVE,
    EXP,
    PARENTO,
    PARENTF,
    VIRGULE,
    POINTVIRGULE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    TokenNombre(u32),
    TokenIdentifiant(String),
    TokenSeparateur(TypeSeparateur),
    TokenMotReserve(TypeMotCle),
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

    fn next_is_character(&self, pos: usize, car: char) -> bool {
        if let Some(c) = self.get_next(1) {
            if get_type_charactere(c) == TypeCharactere::Separateur {
                c == car
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

pub fn lexer(contenu_fichier: String) -> Vec<Vec<Token>> {
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
                if mot == '<' && iter.next_is_character(1, '=') // <=
                    || mot == '>' && iter.next_is_character(1, '=')// >=
                    || mot == '<' && iter.next_is_character(1, '>')
                // <>
                {
                    if let Some(mot2) = iter.next() {
                        s.push(mot2);
                    } else {
                        panic!("erreur de caractere: {}", mot)
                    }
                }
                let token = creation_token(s, 0, EtatLexer::Separateur);
                liste_tokens_ligne.push(token);
            } else if iter.is_type_charactere(TypeCharactere::Mot) {
                // identifiant
                let mut s = mot.to_string();

                while iter.next_is_type_charactere(1, TypeCharactere::Mot)
                    || iter.next_is_fin_variable()
                    || iter.next_is_type_charactere(1, TypeCharactere::Nombre)
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
        || mot == '<'
        || mot == '>'
        || mot == '\\'
        || mot == '^'
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
        EtatLexer::Mot => {
            if let Some(mot_cle) = mots_cles(s.clone()) {
                Token::TokenMotReserve(mot_cle)
            } else {
                Token::TokenIdentifiant(s)
            }
        }
        EtatLexer::Separateur => Token::TokenSeparateur(calcul_operateur(s)),
        EtatLexer::ChaineDeCaracteres => Token::TokenChaine(s),
        _ => panic!("Etat invalide"),
    }
}

fn calcul_operateur(s: String) -> TypeSeparateur {
    if s == "+" {
        TypeSeparateur::PLUS
    } else if s == "-" {
        TypeSeparateur::MOINS
    } else if s == "(" {
        TypeSeparateur::PARENTO
    } else if s == ")" {
        TypeSeparateur::PARENTF
    } else if s == "=" {
        TypeSeparateur::EQU
    } else if s == "<>" {
        TypeSeparateur::DIFF
    } else if s == ">" {
        TypeSeparateur::SUP
    } else if s == ">=" {
        TypeSeparateur::SUPE
    } else if s == "<" {
        TypeSeparateur::INF
    } else if s == "<=" {
        TypeSeparateur::INFE
    } else if s == "*" {
        TypeSeparateur::FOIS
    } else if s == "/" {
        TypeSeparateur::DIVR
    } else if s == "\\" {
        TypeSeparateur::DIVE
    } else if s == "^" {
        TypeSeparateur::EXP
    } else if s == "," {
        TypeSeparateur::VIRGULE
    } else if s == ";" {
        TypeSeparateur::POINTVIRGULE
    } else {
        panic!("separateur invalide : {}", s)
    }
}

fn mots_cles(s: String) -> Option<TypeMotCle> {
    match s.to_uppercase().as_str() {
        "IF" => Some(TypeMotCle::If),
        "ELSE" => Some(TypeMotCle::Else),
        "FOR" => Some(TypeMotCle::For),
        "GOTO" => Some(TypeMotCle::Goto),
        "NEXT" => Some(TypeMotCle::Next),
        "PRINT" => Some(TypeMotCle::Print),
        "MOD" => Some(TypeMotCle::Mod),
        "NOT" => Some(TypeMotCle::Not),
        "AND" => Some(TypeMotCle::And),
        "OR" => Some(TypeMotCle::Or),
        "XOR" => Some(TypeMotCle::Xor),
        "EQV" => Some(TypeMotCle::Eqv),
        "IMP" => Some(TypeMotCle::Imp),
        "CONST" => Some(TypeMotCle::Const),
        "LET" => Some(TypeMotCle::Let),
        "END" => Some(TypeMotCle::End),
        "THEN" => Some(TypeMotCle::Then),
        "SELECT" => Some(TypeMotCle::Select),
        "CASE" => Some(TypeMotCle::Case),
        "ELSEIF" => Some(TypeMotCle::ElseIf),
        "DIM" => Some(TypeMotCle::Dim),
        "AS" => Some(TypeMotCle::As),
        "INTEGER" => Some(TypeMotCle::Integer),
        "STRING" => Some(TypeMotCle::String),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::basic::lexer_basic::TypeSeparateur::{
        DIFF, DIVE, DIVR, EQU, EXP, FOIS, INF, INFE, MOINS, PARENTF, PARENTO, PLUS, SUP, SUPE,
    };

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
    fn test_lexer_identifiant5() {
        let fichier = "aaa
                            bbbb
                            cccc"
            .to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![
                vec![Token::TokenIdentifiant("aaa".to_string())],
                vec![Token::TokenIdentifiant("bbbb".to_string())],
                vec![Token::TokenIdentifiant("cccc".to_string())]
            ]
        );
    }

    #[test]
    fn test_lexer_identifiant6() {
        let fichier = "aaa$ bbb! ccc% ddd# eeee&".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![
                Token::TokenIdentifiant("aaa$".to_string()),
                Token::TokenIdentifiant("bbb!".to_string()),
                Token::TokenIdentifiant("ccc%".to_string()),
                Token::TokenIdentifiant("ddd#".to_string()),
                Token::TokenIdentifiant("eeee&".to_string())
            ]]
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
                Token::TokenSeparateur(PLUS),
                Token::TokenNombre(3)
            ]]
        );
    }

    #[test]
    fn test_lexer_separateur2() {
        let fichier = "+ - * / \\ ^ ( ) < > = >= <= <>".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![
                Token::TokenSeparateur(PLUS),
                Token::TokenSeparateur(MOINS),
                Token::TokenSeparateur(FOIS),
                Token::TokenSeparateur(DIVR),
                Token::TokenSeparateur(DIVE),
                Token::TokenSeparateur(EXP),
                Token::TokenSeparateur(PARENTO),
                Token::TokenSeparateur(PARENTF),
                Token::TokenSeparateur(INF),
                Token::TokenSeparateur(SUP),
                Token::TokenSeparateur(EQU),
                Token::TokenSeparateur(SUPE),
                Token::TokenSeparateur(INFE),
                Token::TokenSeparateur(DIFF)
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
