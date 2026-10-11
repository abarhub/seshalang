use crate::basic::lexer_basic::TypeNombre::{Entier, Long, Single};
use crate::basic::lexer_basic::TypeSeparateur::{DIVE, DIVR, FOIS, MOINS, PARENTF, PARENTO, PLUS, POINTVIRGULE, VIRGULE};
use crate::basic::lexer_basic::{Token, TypeMotCle, TypeNombre, TypeSeparateur, lexer};
use crate::basic::main_basic::AstInstruction::AstAffectation;
use crate::basic::main_basic::AstOperateurBinaireType::{
    DivisionEntiere, DivisionReel, Moins, Multiplication,
};
use crate::basic::main_basic::{
    AstExpression, AstInstruction, AstOperateurBinaireType, AstProgramme,
};
use std::cmp::max;
use std::rc::Rc;

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
        if let Some(Token::TokenIdentifiant(_, _)) = self.get() {
            true
        } else {
            false
        }
    }

    fn est_identifiant_pos(&self, position: i32) -> bool {
        if let Some(Token::TokenIdentifiant(_, _)) = self.get_pos(position) {
            true
        } else {
            false
        }
    }

    fn est_mot_cle(&self, mot_cle: TypeMotCle) -> bool {
        if let Some(Token::TokenMotReserve(mot_reserve, _)) = self.get() {
            mot_cle == mot_reserve
        } else {
            false
        }
    }

    fn est_separateur_pos(&self, separateur: TypeSeparateur, position: i32) -> bool {
        if let Some(Token::TokenSeparateur(x, _)) = self.get_pos(position) {
            x == separateur
        } else {
            false
        }
    }

    fn est_separateur(&self, separateur: TypeSeparateur) -> bool {
        if let Some(Token::TokenSeparateur(x, _)) = self.get() {
            x == separateur
        } else {
            false
        }
    }

    fn est_nombre(&self) -> bool {
        if let Some(Token::TokenNombre(_, _, _, _, _, _)) = self.get() {
            true
        } else {
            false
        }
    }

    fn est_nombre_pos(&self, position: i32) -> bool {
        if let Some(Token::TokenNombre(_, _, _, _, _, _)) = self.get_pos(position) {
            true
        } else {
            false
        }
    }

    fn est_chaine(&self) -> bool {
        if let Some(Token::TokenChaine(_, _)) = self.get() {
            true
        } else {
            false
        }
    }

    fn est_chaine_pos(&self, position: i32) -> bool {
        if let Some(Token::TokenChaine(_, _)) = self.get_pos(position) {
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

fn parsing_programme(liste_tokens: Vec<Vec<Token>>) -> AstProgramme {
    let mut programme = AstProgramme {
        liste_instructions: Vec::new(),
    };
    for ligne in liste_tokens {
        if ligne.len() == 0 {
            continue;
        }

        let mut token_iterator = TokenIterator::new(ligne);

        if token_iterator.est_identifiant()
            && token_iterator.est_separateur_pos(TypeSeparateur::EQU, 1)
        {
            // affectation
            if let Token::TokenIdentifiant(ident, _) = token_iterator.next().unwrap() {
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
            if let Token::TokenIdentifiant(ident, _) = token_iterator.next().unwrap() {
                println!("appel {}", ident);
                let mut liste_expressions: Vec<AstExpression> = vec![];
                let mut separateur_virgule_liste: Vec<bool> = vec![];
                loop {
                    let exp = parsing_expression(&mut token_iterator);
                    liste_expressions.push(exp.0);
                    if token_iterator.len() > 0
                        && separateur_parametre(&ident, token_iterator.get().unwrap())
                    {
                        let separateur_virgule: bool;
                        if let Some(Token::TokenSeparateur(s, _)) = token_iterator.next() {
                            separateur_virgule = s == VIRGULE;
                        } else {
                            panic!(
                                "token {:?} n'est pas un séparateur",
                                token_iterator.get().unwrap()
                            );
                        };
                        separateur_virgule_liste.push(separateur_virgule);
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
        } else if token_iterator.est_mot_cle(TypeMotCle::Print) {
            if let Token::TokenMotReserve(_, _) = token_iterator.next().unwrap() {
                let mut liste_expressions: Vec<AstExpression> = vec![];
                let mut separateur_virgule_liste: Vec<bool> = vec![];
                loop {
                    let exp = parsing_expression(&mut token_iterator);
                    liste_expressions.push(exp.0);
                    if token_iterator.len() > 0
                        && separateur_parametre_print(token_iterator.get().unwrap())
                    {
                        let mut separateur_virgule = true;
                        if let Some(Token::TokenSeparateur(s, _)) = token_iterator.next() {
                            separateur_virgule = s == VIRGULE;
                        } else {
                            panic!(
                                "token {:?} n'est pas un séparateur",
                                token_iterator.get().unwrap()
                            );
                        };
                        separateur_virgule_liste.push(separateur_virgule);
                    } else {
                        break;
                    }
                }

                let instruction = AstInstruction::AstAppelMethode(
                    "print".to_string(),
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
        est_separateur(&mot, VIRGULE) || est_separateur(&mot, POINTVIRGULE)
    } else {
        est_separateur(&mot, VIRGULE)
    }
}

fn separateur_parametre_print(mot: Token) -> bool {
    est_separateur(&mot, VIRGULE) || est_separateur(&mot, POINTVIRGULE)
}

fn est_identifiant(token: &Token) -> bool {
    match token {
        Token::TokenIdentifiant(_, _) => true,
        _ => false,
    }
}

fn est_nombre(token: &Token) -> bool {
    match token {
        Token::TokenNombre(_, _, _, _, _, _) => true,
        _ => false,
    }
}

fn est_separateur(token: &Token, separateur_cherche: TypeSeparateur) -> bool {
    match token {
        Token::TokenSeparateur(separateur, _) => separateur == &separateur_cherche,
        _ => false,
    }
}

fn est_chaine(token: &Token) -> bool {
    match token {
        Token::TokenChaine(_, _) => true,
        _ => false,
    }
}

fn parsing_expression(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    let op1 = expr_mult(liste_tokens);

    if liste_tokens.est_separateur_pos(PLUS, 0) || liste_tokens.est_separateur_pos(MOINS, 0) {
        let token1 = liste_tokens.next().unwrap();
        let op2 = expr_mult(liste_tokens);
        if let Token::TokenSeparateur(separateur, _) = token1 {
            (
                AstExpression::AstOperateurBinaire(
                    calcul_type_operation_binaire(separateur),
                    Rc::new(op1.0),
                    Rc::new(op2.0),
                ),
                op1.1 + op2.1 + 1,
            )
        } else {
            panic!("Expected separator after expression")
        }
    } else {
        op1
    }
}

fn expr_mult(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    let op1 = expr_literaux(liste_tokens);

    if liste_tokens.est_separateur_pos(FOIS, 0)
        || liste_tokens.est_separateur_pos(DIVE, 0)
        || liste_tokens.est_separateur_pos(DIVR, 0)
    {
        let token1 = liste_tokens.next().unwrap();
        let op2 = expr_literaux(liste_tokens);
        if let Token::TokenSeparateur(separateur, _) = token1 {
            (
                AstExpression::AstOperateurBinaire(
                    calcul_type_operation_binaire(separateur),
                    Rc::new(op1.0),
                    Rc::new(op2.0),
                ),
                op1.1 + op2.1 + 1,
            )
        } else {
            panic!("Expected separator after expression")
        }
    } else {
        op1
    }
}

fn expr_literaux(mut liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    if liste_tokens.est_separateur(PARENTO) {
        let mut nb_token: u32 = 0;
        nb_token += 1;
        liste_tokens.next();
        let res = parsing_expression(liste_tokens);
        nb_token += res.1;
        if liste_tokens.est_separateur(PARENTF) {
            nb_token += 1;
            (res.0, nb_token)
        } else {
            panic!("parenthese manquante: {:?}", liste_tokens);
        }
    } else if liste_tokens.est_identifiant() {
        if let Token::TokenIdentifiant(ident, _) = liste_tokens.next().unwrap() {
            return (AstExpression::AstIdentifiant(ident.clone()), 1);
        } else {
            panic!("identifiant inconnu");
        }
    } else if liste_tokens.est_nombre() {
        if let Token::TokenNombre(nombrei16, nombrei32, nombref32, nombref64, type_nombre, _) =
            liste_tokens.next().unwrap()
        {
            match type_nombre {
                Entier => (AstExpression::AstNombreInteger(nombrei16), 1),
                Long => (AstExpression::AstNombreLong(nombrei32), 1),
                Single => (AstExpression::AstNombreSingle(nombref32), 1),
                TypeNombre::Double => (AstExpression::AstNombreDouble(nombref64), 1),
            }
        } else {
            panic!("nombre inconnu: {:?}", liste_tokens.get());
        }
    } else if liste_tokens.est_chaine() {
        if let Token::TokenChaine(chaine, _) = liste_tokens.next().unwrap() {
            return (AstExpression::AstChaine(chaine.clone()), 1);
        } else {
            panic!("chaine inconnue: {:?}", liste_tokens.get());
        }
    } else {
        panic!("expression inconnue: {:?}", liste_tokens);
    }
}

fn calcul_type_operation_binaire(type_operation: TypeSeparateur) -> AstOperateurBinaireType {
    match type_operation {
        PLUS => AstOperateurBinaireType::Plus,
        MOINS => Moins,
        FOIS => Multiplication,
        TypeSeparateur::DIVR => DivisionReel,
        DIVE => DivisionEntiere,
        _ => {
            panic!("unsupported type separateur: {:?}", type_operation);
        }
    }
}
