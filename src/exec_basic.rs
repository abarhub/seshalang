use crate::main_basic::{AstExpression, AstInstruction, AstProgramme};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
enum Valeur {
    Nombre(i32),
    Chaine(String),
}

impl fmt::Display for Valeur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Valeur::Nombre(n) => write!(f, "{}", n),
            Valeur::Chaine(s) => write!(f, "{}", s),
        }
    }
}

pub fn execute(programme: AstProgramme) -> Vec<String> {
    let mut sortie: Vec<String> = Vec::new();
    let mut contexte: HashMap<String, Valeur> = HashMap::new();

    for instruction in programme.liste_instructions.iter() {
        if let AstInstruction::AstAffectation(ident, expression) = instruction {
            let resultat = execute_expression(&expression, &mut contexte);
            contexte.insert(ident.clone(), resultat);
        } else if let AstInstruction::AstAppelMethode(nom, parametres, separateurs) = instruction {
            if nom.to_lowercase() == "print" {
                let mut i = 0;
                for expression in parametres.iter() {
                    let resultat = execute_expression(&expression, &mut contexte);
                    if i < separateurs.len() && !separateurs[i] {
                        print!("{:?} ", resultat);
                        if sortie.is_empty() {
                            sortie.push(resultat.to_string());
                        } else {
                            let len = sortie.len() - 1;
                            let mut s = sortie[len].clone();
                            let s2 = resultat.to_string();
                            s += s2.as_str();
                            sortie[len] = s;
                        }
                    } else {
                        println!("{:?}", resultat);
                        sortie.push(resultat.to_string());
                    }

                    i += 1;
                }
            } else {
                panic!("appel de méthode {} inconnue: {:?}", nom, instruction);
            }
        } else {
            panic!("instruction inconnue: {:?}", instruction);
        }
    }

    sortie
}

fn execute_expression(
    expression: &AstExpression,
    table_symbole: &mut HashMap<String, Valeur>,
) -> Valeur {
    if let AstExpression::AstNombre(nombre) = expression {
        Valeur::Nombre(*nombre as i32)
    } else if let AstExpression::AstIdentifiant(ident) = expression {
        return table_symbole.get(ident).unwrap().clone();
    } else if let AstExpression::AstOperateurBinaire(operateur, exp1, exp2) = expression {
        let e1 = exp1.as_ref();
        let e2 = exp2.as_ref();
        let expr1 = execute_expression(&(e1.clone()), table_symbole);
        let expr2 = execute_expression(&(e2.clone()), table_symbole);
        let n1: i32;
        let n2: i32;
        if let Valeur::Nombre(n) = expr1 {
            n1 = n;
        } else {
            panic!("Expression non numérique: {:?}", expr1);
        }
        if let Valeur::Nombre(n) = expr2 {
            n2 = n;
        } else {
            panic!("Expression non numérique: {:?}", expr2);
        }
        return match operateur.as_str() {
            "+" => Valeur::Nombre(n1 + n2),
            "-" => Valeur::Nombre(n1 - n2),
            "*" => Valeur::Nombre(n1 * n2),
            "/" => Valeur::Nombre(n1 / n2),
            _ => panic!("Opérateur inconnu: {}", operateur),
        };
    } else if let AstExpression::AstChaine(chaine) = expression {
        return Valeur::Chaine(chaine.clone());
    } else {
        panic!("Expression inconnue: {:?}", expression);
    }
}
