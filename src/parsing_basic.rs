use crate::main_basic::{
    AstExpression, AstInstruction, AstProgramme, TypeExpression, TypeInstruction,
};
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Clone)]
enum EtatLexer {
    Autre,
    Nombre,
    Mot,
    Separateur,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum TypeToken {
    Nombre,
    MotReserve,
    Identifiant,
    Separateur,
}

#[derive(Debug, Clone)]
struct Token {
    texte: String,
    nombre: u32,
    type_token: TypeToken,
}

pub fn parse_basic(contenu_fichier: String) -> std::io::Result<AstProgramme> {
    let mut liste_tokens: Vec<Vec<Token>> = Vec::new();

    for ligne in contenu_fichier.lines() {
        let mut s = String::new();
        let mut nombre = 0;
        let mut etat = EtatLexer::Autre;

        let mut liste_tokens_ligne: Vec<Token> = Vec::new();

        for mot in ligne.chars() {
            let nouvel_etat = calcul_etat(mot);
            let etat_precedant = etat.clone();
            if nouvel_etat != etat {
                if etat != EtatLexer::Autre {
                    let token = creation_token(s.clone(), nombre, etat);
                    liste_tokens_ligne.push(token);
                    s = "".to_string();
                    nombre = 0;
                }
            }
            etat = nouvel_etat;
            if etat == EtatLexer::Nombre {
                let n = mot.to_digit(10).unwrap();
                if etat_precedant != EtatLexer::Nombre {
                    etat = EtatLexer::Nombre;
                    nombre = n;
                } else {
                    nombre = nombre * 10 + n;
                }
            } else if etat == EtatLexer::Autre {
                etat = EtatLexer::Autre;
                s = "".to_string();
                nombre = 0;
            } else if etat == EtatLexer::Separateur {
                etat = EtatLexer::Separateur;
                s = "".to_string();
                s.push(mot);
            } else if etat == EtatLexer::Mot {
                if etat_precedant == EtatLexer::Mot {
                    s.push(mot);
                } else {
                    etat = EtatLexer::Mot;
                    s = "".to_string();
                    s.push(mot);
                }
            } else {
                // ignoré
                println!("caractere ignore: {}", mot)
            }
        }

        if etat != EtatLexer::Autre {
            let token = creation_token(s.clone(), nombre, etat);
            liste_tokens_ligne.push(token);
        }

        if liste_tokens_ligne.len() > 0 {
            liste_tokens.push(liste_tokens_ligne);
        }
    }

    println!("liste token : {:?}", liste_tokens);

    let programme = parsing_programme(liste_tokens);

    Ok(programme)
}

fn calcul_etat(mot: char) -> EtatLexer {
    if mot.is_numeric() {
        EtatLexer::Nombre
    } else if mot == ' ' {
        EtatLexer::Autre
    } else if mot == '+'
        || mot == '-'
        || mot == '*'
        || mot == '/'
        || mot == '='
        || mot == '('
        || mot == ')'
    {
        EtatLexer::Separateur
    } else if mot.is_alphabetic()
        || mot == '_'
        || mot == '$'
        || mot == '!'
        || mot == '%'
        || mot == '#'
        || mot == '&'
    {
        EtatLexer::Mot
    } else {
        EtatLexer::Autre
    }
}

fn creation_token(s: String, nombre: u32, etat: EtatLexer) -> Token {
    match etat {
        EtatLexer::Nombre => Token {
            texte: "".to_string(),
            nombre: nombre,
            type_token: TypeToken::Nombre,
        },
        EtatLexer::Mot => Token {
            texte: s,
            nombre: 0,
            type_token: TypeToken::Identifiant,
        },
        EtatLexer::Separateur => Token {
            texte: s,
            nombre: 0,
            type_token: TypeToken::Separateur,
        },
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
            && ligne[0].type_token == TypeToken::Identifiant
            && ligne[1].type_token == TypeToken::Separateur
            && ligne[1].texte == "="
        {
            // affectation
            println!("affectation {}", ligne[0].texte);
            let exp = parsing_expression(ligne[2..].to_vec());
            let instruction = AstInstruction {
                type_instruction: TypeInstruction::Affectation,
                nom: ligne[0].texte.clone(),
                liste_expression: Vec::from([exp.0]),
            };
            programme.liste_instructions.push(instruction);
        } else if ligne.len() >= 1 && ligne[0].type_token == TypeToken::Identifiant {
            // appel de méthode
            println!("appel {}", ligne[0].texte);
            let mut ligne_restant = ligne[1..].to_vec();
            let mut liste_expressions: Vec<AstExpression> = vec![];
            loop {
                let len = ligne_restant.len();
                let exp = parsing_expression(ligne_restant.clone());
                liste_expressions.push(exp.0);
                if exp.1 < len as u32 {
                    let n = exp.1 as usize;
                    ligne_restant = ligne_restant[n..].to_vec();
                } else {
                    break;
                }
            }

            let instruction = AstInstruction {
                type_instruction: TypeInstruction::AppelMethode,
                nom: ligne[0].texte.clone(),
                liste_expression: liste_expressions,
            };

            programme.liste_instructions.push(instruction);
        } else {
            eprintln!("instrction inconnue: {:?}", ligne);
            panic!("instruction inconnue");
        }
    }

    programme
}

fn parsing_expression(liste_tokens: Vec<Token>) -> (AstExpression, u32) {
    if liste_tokens.len() == 1 && liste_tokens[0].type_token == TypeToken::Identifiant {
        return (
            AstExpression {
                type_expression: TypeExpression::Identifiant,
                identifiant: liste_tokens[0].texte.clone(),
                nombre: 0,
                operateur: "".to_string(),
                exp: Rc::new(None),
                exp2: Rc::new(None),
            },
            1,
        );
    } else if liste_tokens.len() == 1 && liste_tokens[0].type_token == TypeToken::Nombre {
        return (
            AstExpression {
                type_expression: TypeExpression::Nombre,
                identifiant: "".to_string(),
                nombre: liste_tokens[0].nombre,
                operateur: "".to_string(),
                exp: Rc::new(None),
                exp2: Rc::new(None),
            },
            1,
        );
    } else if liste_tokens.len() == 3
        && (liste_tokens[0].type_token == TypeToken::Identifiant
            || liste_tokens[0].type_token == TypeToken::Nombre)
        && liste_tokens[1].type_token == TypeToken::Separateur
        && (liste_tokens[1].texte == "+"
            || liste_tokens[1].texte == "-"
            || liste_tokens[1].texte == "*"
            || liste_tokens[1].texte == "/")
        && (liste_tokens[2].type_token == TypeToken::Identifiant
            || liste_tokens[2].type_token == TypeToken::Nombre)
    {
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
            AstExpression {
                type_expression: TypeExpression::OperateurBinaire,
                identifiant: "".to_string(),
                nombre: 0,
                operateur: liste_tokens[1].texte.to_string(),
                exp: Rc::new(Some(expr1)),
                exp2: Rc::new(Some(expr2)),
            },
            nb_token,
        );
    } else if liste_tokens.len() > 0
        && liste_tokens[0].type_token == TypeToken::Separateur
        && liste_tokens[0].texte == "("
    {
        let mut nb_token: u32 = 0;
        nb_token += 1;
        let res = parsing_expression(liste_tokens[1..].to_vec());
        nb_token += res.1;
        if liste_tokens[nb_token as usize].type_token == TypeToken::Separateur
            && liste_tokens[nb_token as usize].texte == ")"
        {
            nb_token += 1;
            return (res.0, nb_token);
        } else {
            eprintln!("expression inconnue: {:?}", liste_tokens);
            panic!("expression inconnue");
        }
    } else if liste_tokens.len() > 0 && liste_tokens[0].type_token == TypeToken::Identifiant {
        return (
            AstExpression {
                type_expression: TypeExpression::Identifiant,
                identifiant: liste_tokens[0].texte.clone(),
                nombre: 0,
                operateur: "".to_string(),
                exp: Rc::new(None),
                exp2: Rc::new(None),
            },
            1,
        );
    } else if liste_tokens.len() > 0 && liste_tokens[0].type_token == TypeToken::Nombre {
        return (
            AstExpression {
                type_expression: TypeExpression::Nombre,
                identifiant: "".to_string(),
                nombre: liste_tokens[0].nombre,
                operateur: "".to_string(),
                exp: Rc::new(None),
                exp2: Rc::new(None),
            },
            1,
        );
    } else {
        eprintln!("expression inconnue: {:?}", liste_tokens);
        panic!("expression inconnue");
    }
}
