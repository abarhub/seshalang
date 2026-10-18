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
    Long,
    Single,
    Double,
    Abs,
    Asc,
    Chr,
    Atn,
    Cos,
    Sin,
    Tan,
    Cdbl,
    Csng,
    Cint,
    Clng,
    Mki,
    Mkl,
    Mks,
    Mkd,
    Cvi,
    Cvl,
    Cvs,
    Cvd,
    Date,
    Fix,
    Int,
    Time,
    Instr,
    Hex,
    Oct,
}

impl TypeMotCle {
    pub const fn est_fonction(self) -> bool {
        match self {
            TypeMotCle::Abs
            | TypeMotCle::Asc
            | TypeMotCle::Chr
            | TypeMotCle::Atn
            | TypeMotCle::Cos
            | TypeMotCle::Sin
            | TypeMotCle::Tan
            | TypeMotCle::Cdbl
            | TypeMotCle::Csng
            | TypeMotCle::Cint
            | TypeMotCle::Clng
            | TypeMotCle::Mki
            | TypeMotCle::Mkl
            | TypeMotCle::Mks
            | TypeMotCle::Mkd
            | TypeMotCle::Cvi
            | TypeMotCle::Cvl
            | TypeMotCle::Cvs
            | TypeMotCle::Cvd
            | TypeMotCle::Date
            | TypeMotCle::Fix
            | TypeMotCle::Int
            | TypeMotCle::Time
            | TypeMotCle::Instr
            | TypeMotCle::Hex
            | TypeMotCle::Oct
            => true,
            _ => false,
        }
    }
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
pub enum TypeNombre {
    Entier,
    Long,
    Single,
    Double,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PositionToken {
    ligne: u32,
    colonne: u32,
}

impl PositionToken {
    fn new(no_ligne: u32, no_colonne: u32) -> PositionToken {
        PositionToken {
            ligne: no_ligne,
            colonne: no_colonne,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    TokenNombre(i16, i32, f32, f64, TypeNombre, PositionToken),
    TokenIdentifiant(String, PositionToken),
    TokenSeparateur(TypeSeparateur, PositionToken),
    TokenMotReserve(TypeMotCle, PositionToken),
    TokenChaine(String, PositionToken),
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
        let position = self.index + pos as i32;
        if position < 0 || position >= self.texte.len() as i32 {
            None
        } else {
            self.texte.chars().nth(position as usize)
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
        if let Some(c) = self.get_next(pos) {
            if get_type_charactere(c) == TypeCharactere::Separateur {
                c == car
            } else {
                false
            }
        } else {
            false
        }
    }

    fn next_is_ident(&self, pos: usize, car: char) -> bool {
        if let Some(c) = self.get_next(pos) {
            c.to_lowercase().eq(car.to_lowercase())
        } else {
            false
        }
    }

    fn next_is_commentaire(&self) -> bool {
        if let Some(c) = self.get() {
            if c == '\'' {
                true
            } else if c.to_lowercase().eq('r'.to_lowercase()) {
                if (self.next_is_ident(1, 'e')) && (self.next_is_ident(2, 'm')) {
                    if let Some(c) = self.get_next(3) {
                        if c.is_ascii_alphanumeric()
                            || (c == '$' || c == '!' || c == '%' || c == '#' || c == '&')
                        {
                            return false;
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn next_is_number(&self, pos: usize) -> bool {
        if let Some(c) = self.get_next(pos) {
            get_type_charactere(c) == TypeCharactere::Nombre
        } else {
            false
        }
    }

    fn no_next(&self) -> bool {
        self.index >= self.texte.len() as i32
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

    let mut ligne_num = 1;

    for ligne in contenu_fichier.lines() {
        let mut liste_tokens_ligne: Vec<Token> = Vec::new();

        let mut iter = CharIterator::new(ligne.to_string());

        while let Some(mot) = iter.next() {
            let pos = PositionToken::new(ligne_num, u32::try_from(iter.index).unwrap());
            if iter.next_is_commentaire() {
                // commentaire
                break;
            } else if iter.is_type_charactere(TypeCharactere::Nombre) {
                // nombre
                let mut s = "".to_string();
                s.push(mot);

                while iter.next_is_type_charactere(1, TypeCharactere::Nombre)
                    || iter.next_is_character(1, '.')
                    || iter.next_is_ident(1, 'e')
                    || iter.next_is_ident(1, 'd')
                    || iter.next_is_ident(1, '#')
                    || iter.next_is_ident(1, '!')
                    || iter.next_is_ident(1, '&')
                    || iter.next_is_ident(1, '%')
                {
                    if let Some(mot2) = iter.next() {
                        s.push(mot2);
                    } else {
                        break;
                    }
                }

                let token = creation_token_nombre(s, pos);
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
                let token = creation_token(s, EtatLexer::Separateur, pos);
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

                let token = creation_token(s, EtatLexer::Mot, pos);
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
                let token = creation_token(s, EtatLexer::ChaineDeCaracteres, pos);
                liste_tokens_ligne.push(token);
            } else {
                // ignoré
                if mot == ' ' || mot == '\n' {
                    // caractères espaces
                } else {
                    panic!("caractere invalide: {}", mot)
                }
            }
        }

        if liste_tokens_ligne.len() > 0 {
            liste_tokens.push(liste_tokens_ligne);
        }

        ligne_num += 1;
    }
    liste_tokens
}

fn creation_token_nombre(s: String, pos: PositionToken) -> Token {
    let mut nombre_entier = 0u32;
    let mut decimal = false;
    let mut decimal_double = false;
    let mut nombre_decimal = 0f64;
    let mut exposant = false;
    let mut nombre_exposant = 0i32;
    let mut iter = CharIterator::new(s.clone());
    let mut est_long = false;
    let mut est_integer = false;

    // partie entiere du nombre
    while iter.next_is_number(1) {
        let c = iter.next().unwrap();
        nombre_entier = nombre_entier * 10 + (c as u8 - '0' as u8) as u32;
    }

    // partie décimale du nombre : .XXXX
    if iter.next_is_character(1, '.') {
        iter.next().unwrap();
        let mut facteur = 0.1f64;
        decimal = true;
        while iter.next_is_number(1) {
            let c = iter.next().unwrap();
            nombre_decimal = nombre_decimal + (c as u8 - '0' as u8) as f64 * facteur;
            facteur = facteur / 10.0;
        }
    }

    // partie exposant du nombre : eXXX
    if iter.next_is_ident(1, 'e') || iter.next_is_ident(1, 'd') {
        let mut exposant_negatif = false;
        decimal_double = iter.next_is_ident(1, 'e');
        exposant = true;
        iter.next().unwrap();
        if iter.next_is_character(1, '-') || iter.next_is_character(1, '+') {
            exposant_negatif = iter.next_is_character(1, '-');
            iter.next().unwrap();
        }
        while iter.next_is_number(1) {
            let c = iter.next().unwrap();
            nombre_exposant = nombre_exposant * 10 + (c as u8 - '0' as u8) as i32;
        }
        if exposant_negatif {
            nombre_exposant = -nombre_exposant;
        }
    }
    if iter.next_is_ident(1, '#') || iter.next_is_ident(1, '!') {
        decimal = true;
        decimal_double = iter.next_is_ident(1, '#');
        iter.next().unwrap();
    } else if iter.next_is_ident(1, '&'){
        est_long = true;
        iter.next().unwrap();
    } else if iter.next_is_ident(1, '%'){
        est_integer = true;
        iter.next().unwrap();
    }
    if iter.no_next() {
        panic!("Nombre invalide {}", s);
    }
    // vérification
    if est_integer && est_long {
        panic!("Nombre invalide {}", s);
    } else if (est_integer||est_long)&&(decimal||exposant) {
        panic!("Nombre invalide {}", s);
    } else if est_integer &&nombre_entier>32767 {
        panic!("Nombre invalide {}", s);
    } else if est_long &&nombre_entier>2147483647 {
        panic!("Nombre invalide {}", s);
    }
    // construction du token
    if !decimal && !exposant {
        if nombre_entier <= 32768&&!est_long {
            Token::TokenNombre(
                i16::try_from(nombre_entier).unwrap(),
                0,
                0.,
                0.,
                TypeNombre::Entier,
                pos,
            )
        } else {
            Token::TokenNombre(
                0,
                i32::try_from(nombre_entier).unwrap(),
                0.,
                0.0,
                TypeNombre::Long,
                pos,
            )
        }
    } else {
        if decimal_double {
            let mut nombre: f64 = nombre_entier as f64;
            if decimal {
                nombre = nombre + nombre_decimal;
            }
            if exposant {
                nombre = nombre * 10.0_f64.powi(nombre_exposant);
            }
            Token::TokenNombre(0, 0, 0., nombre, TypeNombre::Double, pos)
        } else {
            let mut nombre: f32 = nombre_entier as f32;
            if decimal {
                nombre = nombre + f64_to_f32(nombre_decimal);
            }
            if exposant {
                nombre = nombre * 10.0_f32.powi(nombre_exposant);
            }
            Token::TokenNombre(
                0,
                0,
                f32::try_from(nombre).unwrap(),
                0.,
                TypeNombre::Single,
                pos,
            )
        }
    }
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
        || mot == '.'
        || mot == ':'
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

fn creation_token(s: String, etat: EtatLexer, pos: PositionToken) -> Token {
    match etat {
        EtatLexer::Mot => {
            if let Some(mot_cle) = mots_cles(s.clone()) {
                Token::TokenMotReserve(mot_cle, pos)
            } else {
                Token::TokenIdentifiant(s, pos)
            }
        }
        EtatLexer::Separateur => Token::TokenSeparateur(calcul_operateur(s), pos),
        EtatLexer::ChaineDeCaracteres => Token::TokenChaine(s, pos),
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
        "LONG" => Some(TypeMotCle::Long),
        "SINGLE" => Some(TypeMotCle::Single),
        "DOUBLE" => Some(TypeMotCle::Double),
        "ABS" => Some(TypeMotCle::Abs),
        "ASC" => Some(TypeMotCle::Asc),
        "CHR$" => Some(TypeMotCle::Chr),
        "ATN" => Some(TypeMotCle::Atn),
        "COS" => Some(TypeMotCle::Cos),
        "SIN" => Some(TypeMotCle::Sin),
        "TAN" => Some(TypeMotCle::Tan),
        "CDBL" => Some(TypeMotCle::Cdbl),
        "CSNG" => Some(TypeMotCle::Csng),
        "CINT" => Some(TypeMotCle::Cint),
        "CLNG" => Some(TypeMotCle::Clng),
        "MKI$" => Some(TypeMotCle::Mki),
        "MKL$" => Some(TypeMotCle::Mkl),
        "MKS$" => Some(TypeMotCle::Mks),
        "MKD$" => Some(TypeMotCle::Mkd),
        "CVI" => Some(TypeMotCle::Cvi),
        "CVL" => Some(TypeMotCle::Cvl),
        "CVS" => Some(TypeMotCle::Cvs),
        "CVD" => Some(TypeMotCle::Cvd),
        "DATE$" => Some(TypeMotCle::Date),
        "FIX" => Some(TypeMotCle::Fix),
        "INT" => Some(TypeMotCle::Int),
        "TIME$" => Some(TypeMotCle::Time),
        "INSTR" => Some(TypeMotCle::Instr),
        "HEX$" => Some(TypeMotCle::Hex),
        "OCT$" => Some(TypeMotCle::Oct),
        _ => None,
    }
}

fn f64_to_f32(x: f64) -> f32 {
    let y = x as f32;
    assert_eq!(
        x.is_finite(),
        y.is_finite(),
        "f32 overflow during conversion"
    );
    y
}

#[cfg(test)]
mod tests {
    use crate::basic::lexer_basic::TypeMotCle::{Cdbl, Chr, Integer, Mod};
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::basic::lexer_basic::TypeNombre::{Double, Entier, Long, Single};
    use crate::basic::lexer_basic::TypeSeparateur::{
        DIFF, DIVE, DIVR, EQU, EXP, FOIS, INF, INFE, MOINS, PARENTF, PARENTO, PLUS, SUP, SUPE,
    };

    #[test]
    fn test_lexer_nombre() {
        let fichier = "123".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenNombre(
                123,
                0,
                0.,
                0.,
                Entier,
                PositionToken::new(1, 0)
            )]]
        );
    }

    #[test]
    fn test_lexer_nombre2() {
        let fichier = "123.456".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenNombre(
                0,
                0,
                123.456,
                0.,
                Single,
                PositionToken::new(1, 0)
            )]]
        );
    }

    #[test]
    fn test_lexer_nombre3() {
        let fichier = "123.456e2".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenNombre(
                0,
                0,
                0.,
                12345.6,
                Double,
                PositionToken::new(1, 0)
            )]]
        );
    }

    #[test]
    fn test_lexer_nombre4() {
        let fichier = "10%".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenNombre(
                10,
                0,
                0.,
                0.,
                Entier,
                PositionToken::new(1, 0)
            )]]
        );
    }

    #[test]
    fn test_lexer_nombre5() {
        let fichier = "10&".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenNombre(
                0,
                10,
                0.,
                0.,
                Long,
                PositionToken::new(1, 0)
            )]]
        );
    }

    #[test]
    fn test_lexer_nombre6() {
        let fichier = "10!".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenNombre(
                0,
                0,
                10.,
                0.,
                Single,
                PositionToken::new(1, 0)
            )]]
        );
    }

    #[test]
    fn test_lexer_nombre7() {
        let fichier = "10#".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenNombre(
                0,
                0,
                0.,
                10.,
                Double,
                PositionToken::new(1, 0)
            )]]
        );
    }

    #[test]
    fn test_lexer_identifiant() {
        let fichier = "abc".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenIdentifiant(
                "abc".to_string(),
                PositionToken::new(1, 0)
            )]]
        );
    }

    #[test]
    fn test_lexer_identifiant2() {
        let fichier = "abc$".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenIdentifiant(
                "abc$".to_string(),
                PositionToken::new(1, 0)
            )]]
        );
    }

    #[test]
    fn test_lexer_identifiant3() {
        let fichier = "toto tata% 96325".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![
                Token::TokenIdentifiant("toto".to_string(), PositionToken::new(1, 0)),
                Token::TokenIdentifiant("tata%".to_string(), PositionToken::new(1, 5)),
                Token::TokenNombre(0, 96325, 0., 0., Long, PositionToken::new(1, 11))
            ]]
        );
    }

    #[test]
    fn test_lexer_identifiant4() {
        let fichier = "azertyu123_bbb".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenIdentifiant(
                "azertyu123_bbb".to_string(),
                PositionToken::new(1, 0)
            )]]
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
                vec![Token::TokenIdentifiant(
                    "aaa".to_string(),
                    PositionToken::new(1, 0)
                )],
                vec![Token::TokenIdentifiant(
                    "bbbb".to_string(),
                    PositionToken::new(2, 0)
                )],
                vec![Token::TokenIdentifiant(
                    "cccc".to_string(),
                    PositionToken::new(3, 0)
                )]
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
                Token::TokenIdentifiant("aaa$".to_string(), PositionToken::new(1, 0)),
                Token::TokenIdentifiant("bbb!".to_string(), PositionToken::new(1, 5)),
                Token::TokenIdentifiant("ccc%".to_string(), PositionToken::new(1, 10)),
                Token::TokenIdentifiant("ddd#".to_string(), PositionToken::new(1, 15)),
                Token::TokenIdentifiant("eeee&".to_string(), PositionToken::new(1, 20))
            ]]
        );
    }

    #[test]
    fn test_lexer_identifiant7() {
        let fichier = "aaa MOD bbb".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![
                Token::TokenIdentifiant("aaa".to_string(), PositionToken::new(1, 0)),
                Token::TokenMotReserve(Mod, PositionToken::new(1, 4)),
                Token::TokenIdentifiant("bbb".to_string(), PositionToken::new(1, 8))
            ]]
        );
    }

    #[test]
    fn test_lexer_identifiant8() {
        let fichier = "chr$".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenMotReserve(Chr, PositionToken::new(1, 0))]]
        );
    }

    #[test]
    fn test_lexer_identifiant9() {
        let fichier = "CDBL".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenMotReserve(Cdbl, PositionToken::new(1, 0))]]
        );
    }

    #[test]
    fn test_lexer_separateur() {
        let fichier = "2+3".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![
                Token::TokenNombre(2, 0, 0., 0., Entier, PositionToken::new(1, 0)),
                Token::TokenSeparateur(PLUS, PositionToken::new(1, 1)),
                Token::TokenNombre(3, 0, 0., 0., Entier, PositionToken::new(1, 2))
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
                Token::TokenSeparateur(PLUS, PositionToken::new(1, 0)),
                Token::TokenSeparateur(MOINS, PositionToken::new(1, 2)),
                Token::TokenSeparateur(FOIS, PositionToken::new(1, 4)),
                Token::TokenSeparateur(DIVR, PositionToken::new(1, 6)),
                Token::TokenSeparateur(DIVE, PositionToken::new(1, 8)),
                Token::TokenSeparateur(EXP, PositionToken::new(1, 10)),
                Token::TokenSeparateur(PARENTO, PositionToken::new(1, 12)),
                Token::TokenSeparateur(PARENTF, PositionToken::new(1, 14)),
                Token::TokenSeparateur(INF, PositionToken::new(1, 16)),
                Token::TokenSeparateur(SUP, PositionToken::new(1, 18)),
                Token::TokenSeparateur(EQU, PositionToken::new(1, 20)),
                Token::TokenSeparateur(SUPE, PositionToken::new(1, 22)),
                Token::TokenSeparateur(INFE, PositionToken::new(1, 25)),
                Token::TokenSeparateur(DIFF, PositionToken::new(1, 28))
            ]]
        );
    }

    #[test]
    fn test_lexer_guillemet() {
        let fichier = "\"bbb\"".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenChaine(
                "bbb".to_string(),
                PositionToken::new(1, 0)
            ),]]
        );
    }

    #[test]
    fn test_lexer_guillemet2() {
        let fichier = "\"\"".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenChaine(
                "".to_string(),
                PositionToken::new(1, 0)
            ),]]
        );
    }

    #[test]
    fn test_lexer_commentaire() {
        let fichier = "REM aaa bbb ccc".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(liste_tokens.len(), 0);
    }

    #[test]
    fn test_lexer_commentaire2() {
        let fichier = "' aaa bbb ccc".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(liste_tokens.len(), 0);
    }

    #[test]
    fn test_lexer_commentaire3() {
        let fichier = "i REM xxxx".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenIdentifiant(
                "i".to_string(),
                PositionToken::new(1, 0)
            )]]
        );
    }

    #[test]
    fn test_lexer_commentaire4() {
        let fichier = "XXX ' zzz".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenIdentifiant(
                "XXX".to_string(),
                PositionToken::new(1, 0)
            )]]
        );
    }

    #[test]
    fn test_lexer_commentaire5() {
        let fichier = "a=3 rem www".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![
                Token::TokenIdentifiant("a".to_string(), PositionToken::new(1, 0)),
                Token::TokenSeparateur(EQU, PositionToken::new(1, 1)),
                Token::TokenNombre(3, 0, 0., 0., Entier, PositionToken::new(1, 2))
            ]]
        );
    }

    #[test]
    fn test_lexer_commentaire6() {
        let fichier = "remember".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenIdentifiant(
                "remember".to_string(),
                PositionToken::new(1, 0)
            )]]
        );
    }

    #[test]
    fn test_lexer_commentaire7() {
        let fichier = "REM$".to_string();
        let liste_tokens = lexer(fichier);

        assert_eq!(
            liste_tokens,
            vec![vec![Token::TokenIdentifiant(
                "REM$".to_string(),
                PositionToken::new(1, 0)
            )]]
        );
    }
}
