use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

const ETAT_AUTRE: i32 = 0;
const ETAT_NOMBRE: i32 = 1;
const ETAT_MOT: i32 = 2;
const ETAT_SEPARATEUR: i32 = 3;

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

#[derive(Debug)]
struct AstProgramme {
    liste_instructions: Vec<AstInstruction>,
}

#[derive(Debug, Eq, PartialEq)]
enum TypeInstruction {
    Affectation,
    AppelMethode,
}

#[derive(Debug)]
struct AstInstruction {
    type_instruction: TypeInstruction,
    nom: String,
    liste_expression: Vec<AstExpression>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum TypeExpression {
    Nombre,
    Identifiant,
    OperateurBinaire,
}

#[derive(Debug, Clone)]
struct AstExpression {
    type_expression: TypeExpression,
    identifiant: String,
    nombre: u32,
    operateur: String,
    exp: Rc<Option<AstExpression>>,
    exp2: Rc<Option<AstExpression>>,
}

pub fn main_basic(fichier: String) {
    let mut x = 5;
    x += 1;
    println!("x2 is {}", x);
    println!("fichier: {}", fichier);
    parse_basic_file(fichier).expect("Erreur pour lire le fichier");
}

fn parse_basic_file(fichier: String) -> std::io::Result<()> {
    let contenu = fs::read_to_string(fichier)?;

    let programme = parse_basic(contenu)?;

    execute(programme);
    Ok(())
}

fn parse_basic(contenu_fichier: String) -> std::io::Result<AstProgramme> {
    let mut liste_tokens: Vec<Vec<Token>> = Vec::new();

    for ligne in contenu_fichier.lines() {
        let mut s = String::new();
        let mut nombre = 0;
        let mut etat = ETAT_AUTRE;

        let mut liste_tokens_ligne: Vec<Token> = Vec::new();

        for mot in ligne.chars() {
            if mot.is_numeric() {
                let n = mot.to_digit(10).unwrap();
                if etat != ETAT_NOMBRE {
                    if etat != ETAT_AUTRE {
                        let token = creation_token(s.clone(), nombre, etat);
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
                if etat != ETAT_AUTRE {
                    let token = creation_token(s.clone(), nombre, etat);
                    liste_tokens_ligne.push(token);
                    s = "".to_string();
                    nombre = 0;
                }
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
                    let token = creation_token(s.clone(), nombre, etat);
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
                        let token = creation_token(s.clone(), nombre, etat);
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

fn creation_token(s: String, nombre: u32, etat: i32) -> Token {
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

fn execute(programme: AstProgramme) -> Vec<String> {
    let mut sortie: Vec<String> = Vec::new();
    let mut contexte: HashMap<String, i32> = HashMap::new();

    for instruction in programme.liste_instructions.iter() {
        if instruction.type_instruction == TypeInstruction::Affectation {
            let resultat = execute_expression(&instruction.liste_expression[0], &mut contexte);
            contexte.insert(instruction.nom.clone(), resultat);
        } else if instruction.type_instruction == TypeInstruction::AppelMethode {
            if instruction.nom == "print" {
                for expression in instruction.liste_expression.iter() {
                    let resultat = execute_expression(&expression, &mut contexte);
                    println!("{}", resultat);
                    sortie.push(resultat.to_string());
                }
            } else {
                eprintln!(
                    "appel de méthode {} inconnue: {:?}",
                    instruction.nom, instruction
                );
                panic!("appel de méthode inconnue");
            }
        } else {
            eprintln!("instruction inconnue: {:?}", instruction);
            panic!("instruction inconnue");
        }
    }

    sortie
}

fn execute_expression(expression: &AstExpression, contexte: &mut HashMap<String, i32>) -> i32 {
    if expression.type_expression == TypeExpression::Nombre {
        return expression.nombre as i32;
    } else if expression.type_expression == TypeExpression::Identifiant {
        return contexte.get(&expression.identifiant).unwrap().clone();
    } else if expression.type_expression == TypeExpression::OperateurBinaire {
        let e1 = expression.exp.as_ref();
        let e2 = expression.exp2.as_ref();
        let expr1 = execute_expression(&(e1.clone().unwrap()), contexte);
        let expr2 = execute_expression(&(e2.clone().unwrap()), contexte);
        return match expression.operateur.as_str() {
            "+" => expr1 + expr2,
            "-" => expr1 - expr2,
            "*" => expr1 * expr2,
            "/" => expr1 / expr2,
            _ => panic!("Opérateur inconnu: {}", expression.operateur),
        };
    } else {
        eprintln!("Expression inconnue: {:?}", expression);
        panic!("Expression inconnue");
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_execute() {
        let fichier = "x=5
                                y=x+7
                                print(x)
                                print(y)"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 2);
        assert_eq!(sortie[0], "5");
        assert_eq!(sortie[1], "12");
    }

    #[test]
    fn test_parse_execute2() {
        let fichier = "print 10".to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 1);
        assert_eq!(sortie[0], "10");
    }

    #[test]
    fn test_parse_execute3() {
        let fichier = "print ((15))".to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 1);
        assert_eq!(sortie[0], "15");
    }

    #[test]
    fn test_parse_execute4() {
        let fichier = "print 8 20".to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 2);
        assert_eq!(sortie[0], "8");
        assert_eq!(sortie[1], "20");
    }
}
