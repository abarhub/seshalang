use crate::basic::exec_basic::Valeur::{NombreDouble, NombreInteger, NombreLong, NombreSingle};
use crate::basic::main_basic::{
    AstExpression, AstInstruction, AstOperateurBinaireType, AstProgramme,
};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
enum Valeur {
    NombreInteger(i16),
    NombreLong(i32),
    NombreSingle(f32),
    NombreDouble(f64),
    Chaine(String),
}

impl fmt::Display for Valeur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Valeur::NombreInteger(n) => write!(f, "{}", n),
            Valeur::NombreLong(n) => write!(f, "{}", n),
            Valeur::NombreSingle(n) => write!(f, "{}", n),
            Valeur::NombreDouble(n) => write!(f, "{}", n),
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
    if let AstExpression::AstNombreInteger(nombre) = expression {
        NombreInteger(*nombre)
    } else if let AstExpression::AstNombreLong(nombre) = expression {
        NombreLong(*nombre)
    } else if let AstExpression::AstNombreSingle(nombre) = expression {
        NombreSingle(*nombre)
    } else if let AstExpression::AstNombreDouble(nombre) = expression {
        NombreDouble(*nombre)
    } else if let AstExpression::AstIdentifiant(ident) = expression {
        return table_symbole.get(ident).unwrap().clone();
    } else if let AstExpression::AstOperateurBinaire(operateur, exp1, exp2) = expression {
        let e1 = exp1.as_ref();
        let e2 = exp2.as_ref();
        let expr1 = execute_expression(&(e1.clone()), table_symbole);
        let expr2 = execute_expression(&(e2.clone()), table_symbole);
        if est_nombre(expr1.clone()) {
            let (expr1, expr2) = normalise(expr1, expr2);
            return match (operateur, expr1, expr2) {
                (AstOperateurBinaireType::Plus, NombreInteger(n1), NombreInteger(n2)) => {
                    NombreInteger(n1 + n2)
                }
                (AstOperateurBinaireType::Moins, NombreInteger(n1), NombreInteger(n2)) => {
                    NombreInteger(n1 - n2)
                }
                (AstOperateurBinaireType::Multiplication, NombreInteger(n1), NombreInteger(n2)) => {
                    NombreInteger(n1 * n2)
                }
                (
                    AstOperateurBinaireType::DivisionEntiere,
                    NombreInteger(n1),
                    NombreInteger(n2),
                ) => NombreInteger(n1 / n2),
                (AstOperateurBinaireType::DivisionReel, NombreInteger(n1), NombreInteger(n2)) => {
                    NombreSingle(n1 as f32 / n2 as f32)
                }
                (AstOperateurBinaireType::Plus, NombreLong(n1), NombreLong(n2)) => {
                    NombreLong(n1 + n2)
                }
                (AstOperateurBinaireType::Moins, NombreLong(n1), NombreLong(n2)) => {
                    NombreLong(n1 - n2)
                }
                (AstOperateurBinaireType::Multiplication, NombreLong(n1), NombreLong(n2)) => {
                    NombreLong(n1 * n2)
                }
                (AstOperateurBinaireType::DivisionEntiere, NombreLong(n1), NombreLong(n2)) => {
                    NombreLong(n1 / n2)
                }
                (AstOperateurBinaireType::DivisionReel, NombreLong(n1), NombreLong(n2)) => {
                    NombreSingle(n1 as f32 / n2 as f32)
                }
                (AstOperateurBinaireType::Plus, NombreSingle(n1), NombreSingle(n2)) => {
                    NombreSingle(n1 + n2)
                }
                (AstOperateurBinaireType::Moins, NombreSingle(n1), NombreSingle(n2)) => {
                    NombreSingle(n1 - n2)
                }
                (AstOperateurBinaireType::Multiplication, NombreSingle(n1), NombreSingle(n2)) => {
                    NombreSingle(n1 * n2)
                }
                (AstOperateurBinaireType::DivisionEntiere, NombreSingle(n1), NombreSingle(n2)) => {
                    NombreSingle(n1 / n2)
                }
                (AstOperateurBinaireType::DivisionReel, NombreSingle(n1), NombreSingle(n2)) => {
                    NombreSingle(n1 / n2)
                }
                (AstOperateurBinaireType::Plus, NombreDouble(n1), NombreDouble(n2)) => {
                    NombreDouble(n1 + n2)
                }
                (AstOperateurBinaireType::Moins, NombreDouble(n1), NombreDouble(n2)) => {
                    NombreDouble(n1 - n2)
                }
                (AstOperateurBinaireType::Multiplication, NombreDouble(n1), NombreDouble(n2)) => {
                    NombreDouble(n1 * n2)
                }
                (AstOperateurBinaireType::DivisionReel, NombreDouble(n1), NombreDouble(n2)) => {
                    NombreDouble(n1 / n2)
                }
                _ => panic!("Opérateur inconnu: {:?}", operateur),
            };
        } else if let Valeur::Chaine(s1) = expr1 {
            let s2: String;
            if let Valeur::Chaine(s02) = expr2 {
                s2 = s02;
            } else {
                panic!("Expression non numérique: {:?}", expr2);
            }
            return match operateur {
                AstOperateurBinaireType::Plus => Valeur::Chaine(s1 + s2.as_str()),
                _ => panic!("Opérateur inconnu: {:?}", operateur),
            };
        } else {
            panic!("Expression invalide: {:?}", expr1);
        }
    } else if let AstExpression::AstChaine(chaine) = expression {
        return Valeur::Chaine(chaine.clone());
    } else {
        panic!("Expression inconnue: {:?}", expression);
    }
}

fn normalise(expr1: Valeur, expr2: Valeur) -> (Valeur, Valeur) {
    match (expr1.clone(), expr2.clone()) {
        (NombreInteger(n1), NombreInteger(n2)) => (NombreInteger(n1), NombreInteger(n2)),
        (NombreLong(n1), NombreLong(n2)) => (NombreLong(n1), NombreLong(n2)),
        (NombreSingle(n1), NombreSingle(n2)) => (NombreSingle(n1), NombreSingle(n2)),
        (NombreDouble(n1), NombreDouble(n2)) => (NombreDouble(n1), NombreDouble(n2)),
        (NombreDouble(n1), NombreInteger(n2)) => (NombreDouble(n1), NombreDouble(n2 as f64)),
        (NombreInteger(n1), NombreDouble(n2)) => (NombreDouble(n1 as f64), NombreDouble(n2)),
        (NombreDouble(n1), NombreLong(n2)) => (NombreDouble(n1), NombreDouble(n2 as f64)),
        (NombreLong(n1), NombreDouble(n2)) => (NombreDouble(n1 as f64), NombreDouble(n2)),
        (NombreDouble(n1), NombreSingle(n2)) => (NombreDouble(n1), NombreDouble(n2 as f64)),
        (NombreSingle(n1), NombreDouble(n2)) => (NombreDouble(n1 as f64), NombreDouble(n2)),
        (NombreLong(n1), NombreInteger(n2)) => (NombreLong(n1), NombreLong(n2 as i32)),
        (NombreInteger(n1), NombreLong(n2)) => (NombreLong(n1 as i32), NombreLong(n2)),
        (NombreLong(n1), NombreSingle(n2)) => (NombreSingle(n1 as f32), NombreSingle(n2)),
        (NombreSingle(n1), NombreLong(n2)) => (NombreSingle(n1), NombreSingle(n2 as f32)),
        (NombreInteger(n1), NombreSingle(n2)) => (NombreSingle(n1 as f32), NombreSingle(n2)),
        (NombreSingle(n1), NombreInteger(n2)) => (NombreSingle(n1), NombreSingle(n2 as f32)),
        _ => (expr1, expr2),
    }
}

fn est_nombre(valeur: Valeur) -> bool {
    match valeur {
        NombreInteger(_) => true,
        NombreLong(_) => true,
        NombreSingle(_) => true,
        NombreDouble(_) => true,
        _ => false,
    }
}
