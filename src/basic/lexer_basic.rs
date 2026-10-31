use std::fmt;

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
    StringMC,
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
    Input,
    While,
    Wend,
    Until,
    Do,
    Loop,
    To,
    Step,
    Cls,
    Defint,
    Deflng,
    Defsng,
    Defdbl,
    Defstr,
    Shared,
    Str,
    Val,
    Space,
    Mid,
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
            | TypeMotCle::Str
            | TypeMotCle::Val
            | TypeMotCle::Space
            | TypeMotCle::Mid => true,
            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum EtatLexer {
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
    DEUXPOINT,
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

impl fmt::Display for PositionToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.ligne, self.colonne)
    }
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

pub fn lexer(contenu_fichier: String) -> Result<Vec<Vec<Token>>, String> {
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

                let token = creation_token_nombre(s, pos)?;
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
                        return Err(format!(
                            "erreur de caractere à la position {} : {}",
                            pos, mot
                        ));
                    }
                }
                let token = creation_token(s, EtatLexer::Separateur, pos)?;
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

                let token = creation_token(s, EtatLexer::Mot, pos)?;
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
                let token = creation_token(s, EtatLexer::ChaineDeCaracteres, pos)?;
                liste_tokens_ligne.push(token);
            } else {
                // ignoré
                if mot == ' ' || mot == '\n' {
                    // caractères espaces
                } else {
                    return Err(format!(
                        "caractere invalide à la position {} : {}",
                        pos, mot
                    ));
                }
            }
        }

        if liste_tokens_ligne.len() > 0 {
            liste_tokens.push(liste_tokens_ligne);
        }

        ligne_num += 1;
    }
    Ok(liste_tokens)
}

fn creation_token_nombre(s: String, pos: PositionToken) -> Result<Token, String> {
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
    } else if iter.next_is_ident(1, '&') {
        est_long = true;
        iter.next().unwrap();
    } else if iter.next_is_ident(1, '%') {
        est_integer = true;
        iter.next().unwrap();
    }
    if iter.no_next() {
        return Err(format!("Nombre invalide à la position {} : {}", pos, s));
    }
    // vérification
    if est_integer && est_long {
        return Err(format!("Nombre invalide à la position {} : {}", pos, s));
    } else if (est_integer || est_long) && (decimal || exposant) {
        return Err(format!("Nombre invalide à la position {} : {}", pos, s));
    } else if est_integer && nombre_entier > 32767 {
        return Err(format!("Nombre invalide à la position {} : {}", pos, s));
    } else if est_long && nombre_entier > 2147483647 {
        return Err(format!("Nombre invalide à la position {} : {}", pos, s));
    }
    // construction du token
    if !decimal && !exposant {
        if nombre_entier <= 32768 && !est_long {
            Ok(Token::TokenNombre(
                i16::try_from(nombre_entier).unwrap(),
                0,
                0.,
                0.,
                TypeNombre::Entier,
                pos,
            ))
        } else {
            Ok(Token::TokenNombre(
                0,
                i32::try_from(nombre_entier).unwrap(),
                0.,
                0.0,
                TypeNombre::Long,
                pos,
            ))
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
            Ok(Token::TokenNombre(
                0,
                0,
                0.,
                nombre,
                TypeNombre::Double,
                pos,
            ))
        } else {
            let mut nombre: f32 = nombre_entier as f32;
            if decimal {
                nombre = nombre + f64_to_f32(nombre_decimal, pos.clone())?;
            }
            if exposant {
                nombre = nombre * 10.0_f32.powi(nombre_exposant);
            }
            Ok(Token::TokenNombre(
                0,
                0,
                f32::try_from(nombre).unwrap(),
                0.,
                TypeNombre::Single,
                pos,
            ))
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

fn creation_token(s: String, etat: EtatLexer, pos: PositionToken) -> Result<Token, String> {
    match etat {
        EtatLexer::Mot => {
            if let Some(mot_cle) = mots_cles(s.clone()) {
                Ok(Token::TokenMotReserve(mot_cle, pos))
            } else {
                Ok(Token::TokenIdentifiant(s, pos))
            }
        }
        EtatLexer::Separateur => Ok(Token::TokenSeparateur(calcul_operateur(s)?, pos)),
        EtatLexer::ChaineDeCaracteres => Ok(Token::TokenChaine(s, pos)),
    }
}

fn calcul_operateur(s: String) -> Result<TypeSeparateur, String> {
    if s == "+" {
        Ok(TypeSeparateur::PLUS)
    } else if s == "-" {
        Ok(TypeSeparateur::MOINS)
    } else if s == "(" {
        Ok(TypeSeparateur::PARENTO)
    } else if s == ")" {
        Ok(TypeSeparateur::PARENTF)
    } else if s == "=" {
        Ok(TypeSeparateur::EQU)
    } else if s == "<>" {
        Ok(TypeSeparateur::DIFF)
    } else if s == ">" {
        Ok(TypeSeparateur::SUP)
    } else if s == ">=" {
        Ok(TypeSeparateur::SUPE)
    } else if s == "<" {
        Ok(TypeSeparateur::INF)
    } else if s == "<=" {
        Ok(TypeSeparateur::INFE)
    } else if s == "*" {
        Ok(TypeSeparateur::FOIS)
    } else if s == "/" {
        Ok(TypeSeparateur::DIVR)
    } else if s == "\\" {
        Ok(TypeSeparateur::DIVE)
    } else if s == "^" {
        Ok(TypeSeparateur::EXP)
    } else if s == "," {
        Ok(TypeSeparateur::VIRGULE)
    } else if s == ";" {
        Ok(TypeSeparateur::POINTVIRGULE)
    } else if s == ":" {
        Ok(TypeSeparateur::DEUXPOINT)
    } else {
        return Err(format!("separateur invalide : {}", s));
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
        "STRING" => Some(TypeMotCle::StringMC),
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
        "INPUT" => Some(TypeMotCle::Input),
        "WHILE" => Some(TypeMotCle::While),
        "WEND" => Some(TypeMotCle::Wend),
        "UNTIL" => Some(TypeMotCle::Until),
        "DO" => Some(TypeMotCle::Do),
        "LOOP" => Some(TypeMotCle::Loop),
        "TO" => Some(TypeMotCle::To),
        "STEP" => Some(TypeMotCle::Step),
        "CLS" => Some(TypeMotCle::Cls),
        "DEFINT" => Some(TypeMotCle::Defint),
        "DEFLNG" => Some(TypeMotCle::Deflng),
        "DEFSNG" => Some(TypeMotCle::Defsng),
        "DEFDBL" => Some(TypeMotCle::Defdbl),
        "DEFSTR" => Some(TypeMotCle::Defstr),
        "SHARED" => Some(TypeMotCle::Shared),
        "STR$" => Some(TypeMotCle::Str),
        "VAL" => Some(TypeMotCle::Val),
        "SPACE$" => Some(TypeMotCle::Space),
        "MID$" => Some(TypeMotCle::Mid),
        _ => None,
    }
}

fn f64_to_f32(x: f64, pos: PositionToken) -> Result<f32, String> {
    let y = x as f32;
    if y.is_finite() || !x.is_finite() {
        Ok(y) // pas de débordement (ou x était déjà inf/NaN)
    } else {
        Err(format!(
            "Nombre decimal trop grand à la position {} : {}",
            pos, x
        ))
    }
}

#[cfg(test)]
mod tests;
