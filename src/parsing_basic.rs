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



#[derive(Debug, Clone)]
enum Token {
    TokenNombre(u32),
    TokenIdentifiant(String),
    TokenSeparateur(String),
    TokenMotReserve(String),
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
        EtatLexer::Nombre => Token::TokenNombre(nombre),
        EtatLexer::Mot => Token::TokenIdentifiant(s),
        EtatLexer::Separateur => Token::TokenSeparateur(s),
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
                let instruction = AstInstruction {
                    type_instruction: TypeInstruction::Affectation,
                    nom: ident,
                    liste_expression: Vec::from([exp.0]),
                };
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
                    nom: ident,
                    liste_expression: liste_expressions,
                };

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

fn parsing_expression(liste_tokens: Vec<Token>) -> (AstExpression, u32) {
    if liste_tokens.len() == 1 && est_identifiant(&liste_tokens[0]) {
        if let Token::TokenIdentifiant(ident) = &liste_tokens[0] {
            return (
                AstExpression {
                    type_expression: TypeExpression::Identifiant,
                    identifiant: ident.clone(),
                    nombre: 0,
                    operateur: "".to_string(),
                    exp: Rc::new(None),
                    exp2: Rc::new(None),
                },
                1,
            );
        } else {
            panic!(
                "Token {:?} n'est pas un identifiant",
                liste_tokens[0].clone()
            );
        }
    } else if liste_tokens.len() == 1 && est_nombre(&liste_tokens[0]) {
        if let Token::TokenNombre(nombre) = liste_tokens[0] {
            return (
                AstExpression {
                    type_expression: TypeExpression::Nombre,
                    identifiant: "".to_string(),
                    nombre: nombre,
                    operateur: "".to_string(),
                    exp: Rc::new(None),
                    exp2: Rc::new(None),
                },
                1,
            );
        } else {
            panic!("Token {:?} n'est pas un nombre", liste_tokens[0].clone());
        }
    } else if liste_tokens.len() == 3
        && (est_identifiant(&liste_tokens[0]) || est_nombre(&liste_tokens[0]))
        && (est_separateur(&liste_tokens[1], "+".to_string())
            || est_separateur(&liste_tokens[1], "-".to_string())
            || est_separateur(&liste_tokens[1], "*".to_string())
            || est_separateur(&liste_tokens[1], "/".to_string()))
        && (est_identifiant(&liste_tokens[2]) || est_nombre(&liste_tokens[2]))
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
                AstExpression {
                    type_expression: TypeExpression::OperateurBinaire,
                    identifiant: "".to_string(),
                    nombre: 0,
                    operateur: separateur.clone(),
                    exp: Rc::new(Some(expr1)),
                    exp2: Rc::new(Some(expr2)),
                },
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
            return (
                AstExpression {
                    type_expression: TypeExpression::Identifiant,
                    identifiant: ident.clone(),
                    nombre: 0,
                    operateur: "".to_string(),
                    exp: Rc::new(None),
                    exp2: Rc::new(None),
                },
                1,
            );
        } else {
            panic!("identifiant inconnu");
        }
    } else if liste_tokens.len() > 0 && est_nombre(&liste_tokens[0]) {
        if let Token::TokenNombre(nombre) = &liste_tokens[0] {
            return (
                AstExpression {
                    type_expression: TypeExpression::Nombre,
                    identifiant: "".to_string(),
                    nombre: *nombre,
                    operateur: "".to_string(),
                    exp: Rc::new(None),
                    exp2: Rc::new(None),
                },
                1,
            );
        } else {
            panic!("nombre inconnu");
        }
    } else {
        eprintln!("expression inconnue: {:?}", liste_tokens);
        panic!("expression inconnue");
    }
}
