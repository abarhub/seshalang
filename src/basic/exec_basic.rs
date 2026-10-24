use crate::basic::exec_basic::Valeur::{
    Chaine, NombreDouble, NombreInteger, NombreLong, NombreSingle,
};
use crate::basic::fonction_basic;
use std::cmp::PartialEq;

use crate::basic::struct_basic::AstExpression::{AstFonction, AstOperateurUnnaire};
use crate::basic::struct_basic::AstInstruction::{
    AstAffectation, AstAppelMethode, AstGoto, AstIf, AstInput, AstInstrLabel,
};
use crate::basic::struct_basic::AstLabelEnum::AstLabel;
use crate::basic::struct_basic::{
    AstExpression, AstInstruction, AstLabelEnum, AstOperateurBinaireType, AstOperateurUnaireType,
    AstProgramme,
};
use crate::basic::utils_basic::Sortie;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

const TRUE: i16 = -1;
const FALSE: i16 = 0;

const DEBUG_PRINT: bool = false;

#[derive(Debug, Clone)]
pub(crate) enum Valeur {
    NombreInteger(i16),
    NombreLong(i32),
    NombreSingle(f32),
    NombreDouble(f64),
    Chaine(String),
}

impl fmt::Display for Valeur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NombreInteger(n) => write!(f, "{}", n),
            NombreLong(n) => write!(f, "{}", n),
            NombreSingle(n) => write!(f, "{}", n),
            NombreDouble(n) => write!(f, "{}", n),
            Chaine(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug)]
pub struct TableSymbole {
    variables: HashMap<String, Valeur>,
}

impl TableSymbole {
    pub(crate) fn new() -> TableSymbole {
        TableSymbole {
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, nom: String, valeur: Valeur) {
        self.variables.insert(nom, valeur);
    }

    pub fn get_variable(&self, nom: &str) -> Option<&Valeur> {
        self.variables.get(nom)
    }
}

pub fn execute(programme: AstProgramme, mut sortie: &mut Sortie) {
    let mut contexte = TableSymbole::new();

    if DEBUG_PRINT {
        println!("programme a executer: {:?}", programme);
    }

    let mut pos_instruction = 0;

    while pos_instruction < programme.liste_instructions.len() {
        let instruction = &programme.liste_instructions[pos_instruction];

        let mut pos_change = false;

        if let AstAffectation(ident, expression) = instruction {
            let resultat = execute_expression(&expression, &mut contexte);
            let tmp = resultat.clone();
            contexte.set_variable(ident.clone(), resultat);
            if DEBUG_PRINT {
                println!("affect {:?} := {:?}", ident.clone(), tmp);
            }
        } else if let AstAppelMethode(nom, parametres, separateurs) = instruction {
            if nom.to_lowercase() == "print" {
                print(&mut sortie, &mut contexte, parametres, separateurs);
            } else {
                panic!("appel de méthode {} inconnue: {:?}", nom, instruction);
            }
        } else if let AstGoto(label2) = instruction {
            let pos = trouve_label(label2, &programme.liste_instructions);
            pos_instruction = pos;
            pos_change = true;
        } else if let AstIf(expr, label) = instruction {
            let resultat = execute_expression(&expr, &mut contexte);
            let valeur_true = match resultat {
                NombreInteger(n) => n != 0,
                NombreLong(n) => n != 0,
                _ => false,
            };
            if valeur_true {
                let pos = trouve_label(label, &programme.liste_instructions);
                pos_instruction = pos;
                pos_change = true;
            }
        } else if let AstInstrLabel(_) = instruction {
            // on ne fait rien
        } else if let AstInput(message, liste_variable, retour_ligne, question) = instruction {
            input(
                message,
                liste_variable,
                retour_ligne,
                question,
                &mut contexte,
            );
        } else {
            panic!("instruction inconnue: {:?}", instruction);
        }
        if !pos_change {
            pos_instruction += 1;
        }
    }
}

fn print(
    sortie: &mut Sortie,
    mut contexte: &mut TableSymbole,
    parametres: &Vec<AstExpression>,
    separateurs: &Vec<bool>,
) {
    let mut i = 0;
    for expression in parametres.iter() {
        let resultat = execute_expression(&expression, &mut contexte);
        let resultat = calcul_affichage(resultat);
        if i < separateurs.len() && !separateurs[i] {
            if DEBUG_PRINT {
                println!("{:?}", resultat);
            }
            print!("{:?} ", resultat);
            sortie.ajoute(resultat.to_string(), false);
        } else {
            if DEBUG_PRINT {
                println!("{:?}", resultat);
            }
            sortie.ajoute(resultat.to_string(), true);
        }

        i += 1;
    }
}

fn calcul_affichage(expression: Valeur) -> String {
    match expression {
        NombreInteger(n) => {
            if n >= 0 {
                " ".to_string() + n.to_string().as_str() + " "
            } else {
                n.to_string() + " "
            }
        }
        NombreLong(n) => {
            if n >= 0 {
                " ".to_string() + n.to_string().as_str() + " "
            } else {
                n.to_string() + " "
            }
        }
        NombreSingle(n) => {
            if n >= 0. {
                " ".to_string() + n.to_string().as_str() + " "
            } else {
                n.to_string() + " "
            }
        }
        NombreDouble(n) => {
            if n >= 0. {
                " ".to_string() + n.to_string().as_str() + " "
            } else {
                n.to_string() + " "
            }
        }
        Chaine(s) => s.to_string(),
    }
}

fn input(
    message: &Option<String>,
    liste_variables: &Vec<String>,
    retour_ligne: &bool,
    question: &bool,
    contexte: &mut TableSymbole,
) {
    if let Some(message) = message {
        if *question {
            print!("{} ? ", message);
        } else {
            print!("{}", message);
        }
    } else if *question {
        print!("? ");
    }
    if !liste_variables.is_empty() {
        for nom_var in liste_variables {
            loop {
                let mut valeur = String::new();
                std::io::stdin().read_line(&mut valeur).unwrap();
                valeur.pop();
                valeur = valeur.trim().to_string();
                if valeur.is_empty() {
                    let valeur = NombreInteger(0);
                    contexte.set_variable(nom_var.clone(), valeur);
                    break;
                }
                let nombre_opt = valeur.parse::<i32>();
                if nombre_opt.is_ok() {
                    let valeur = NombreLong(nombre_opt.unwrap());
                    contexte.set_variable(nom_var.clone(), valeur.clone());
                    break;
                } else {
                    println!("Redo from start");
                }
            }
        }
    }
    if *retour_ligne {
        println!();
    }
}

fn trouve_label(label: &AstLabelEnum, liste_instructions: &Vec<AstInstruction>) -> usize {
    if let AstLabel(nom_label) = label {
        let mut pos = 0;
        for instr in liste_instructions {
            if let AstInstrLabel(AstLabel(label2)) = instr {
                if *nom_label == *label2 {
                    return pos;
                }
            }
            pos += 1;
        }
        pos
    } else {
        panic!("label {:?} non trouvé", label);
    }
}

pub fn execute_expression(expression: &AstExpression, table_symbole: &mut TableSymbole) -> Valeur {
    let tmp = execute_expression0(expression, table_symbole);
    if DEBUG_PRINT {
        println!("res => {:?}", tmp);
    }
    tmp
}

fn execute_expression0(expression: &AstExpression, table_symbole: &mut TableSymbole) -> Valeur {
    if DEBUG_PRINT {
        println!("exp={:?}", expression);
    }
    if let AstExpression::AstNombreInteger(nombre) = expression {
        NombreInteger(*nombre)
    } else if let AstExpression::AstNombreLong(nombre) = expression {
        NombreLong(*nombre)
    } else if let AstExpression::AstNombreSingle(nombre) = expression {
        NombreSingle(*nombre)
    } else if let AstExpression::AstNombreDouble(nombre) = expression {
        NombreDouble(*nombre)
    } else if let AstExpression::AstIdentifiant(ident) = expression {
        if let Some(v) = table_symbole.get_variable(ident) {
            return v.clone();
        } else {
            panic!("Identifiant {} non déclaré", ident);
        }
    } else if let AstExpression::AstOperateurBinaire(operateur, exp1, exp2) = expression {
        if let Some(value) = operateur_binaire(table_symbole, operateur, exp1, exp2) {
            return value;
        } else {
            panic!("Opérateur binaire inconnu: {:?}", operateur);
        }
    } else if let AstExpression::AstChaine(chaine) = expression {
        return Chaine(chaine.clone());
    } else if let AstOperateurUnnaire(operateur, exp1) = expression {
        if let Some(value) = operateur_unaire(table_symbole, operateur, exp1) {
            return value;
        } else {
            panic!("Opérateur unaire inconnu: {:?}", operateur);
        }
    } else if let AstFonction(nom_methode, liste_expressions) = expression {
        return fonction_basic::execute_fonction(
            nom_methode.clone(),
            liste_expressions.clone(),
            table_symbole,
        );
    } else {
        panic!("Expression inconnue: {:?}", expression);
    }
}

fn operateur_binaire(
    table_symbole: &mut TableSymbole,
    operateur: &AstOperateurBinaireType,
    exp1: &Rc<AstExpression>,
    exp2: &Rc<AstExpression>,
) -> Option<Valeur> {
    let e1 = exp1.as_ref();
    let e2 = exp2.as_ref();
    let expr1 = execute_expression(&(e1.clone()), table_symbole);
    let expr2 = execute_expression(&(e2.clone()), table_symbole);
    if DEBUG_PRINT {
        println!("op={:?}", operateur);
    }
    if est_nombre(expr1.clone()) {
        let (expr1, expr2) = normalise(expr1, expr2);
        Some(match (expr1, expr2) {
            (NombreInteger(n1), NombreInteger(n2)) => match operateur {
                AstOperateurBinaireType::Plus => NombreInteger(n1 + n2),
                AstOperateurBinaireType::Moins => NombreInteger(n1 - n2),
                AstOperateurBinaireType::Multiplication => NombreInteger(n1 * n2),
                AstOperateurBinaireType::DivisionEntiere => NombreInteger(n1 / n2),
                AstOperateurBinaireType::DivisionReel => NombreSingle(n1 as f32 / n2 as f32),
                AstOperateurBinaireType::Mod => NombreInteger(n1.checked_rem(n2).unwrap_or(0)),
                AstOperateurBinaireType::Exponentiel => {
                    NombreInteger(n1.checked_pow(n2 as u32).unwrap_or(0))
                }
                AstOperateurBinaireType::Egal => test(n1 == n2),
                AstOperateurBinaireType::Different => test(n1 != n2),
                AstOperateurBinaireType::Superieur => test(n1 > n2),
                AstOperateurBinaireType::SuperieurEgal => test(n1 >= n2),
                AstOperateurBinaireType::Inferieur => test(n1 < n2),
                AstOperateurBinaireType::InferieurEgal => test(n1 <= n2),
                AstOperateurBinaireType::And => NombreInteger(n1 & n2),
                AstOperateurBinaireType::Or => NombreInteger(n1 | n2),
                AstOperateurBinaireType::Xor => NombreInteger(n1 ^ n2),
                AstOperateurBinaireType::Imp => NombreInteger(!n1 | n2),
                AstOperateurBinaireType::Eqv => NombreInteger(!(n1 ^ n2)),
                _ => panic!("Opérateur inconnu: {:?}", operateur),
            },
            (NombreLong(n1), NombreLong(n2)) => match operateur {
                AstOperateurBinaireType::Plus => NombreLong(n1 + n2),
                AstOperateurBinaireType::Moins => NombreLong(n1 - n2),
                AstOperateurBinaireType::Multiplication => NombreLong(n1 * n2),
                AstOperateurBinaireType::DivisionEntiere => NombreLong(n1 / n2),
                AstOperateurBinaireType::DivisionReel => NombreSingle(n1 as f32 / n2 as f32),
                AstOperateurBinaireType::Mod => NombreLong(n1.checked_rem(n2).unwrap_or(0)),
                AstOperateurBinaireType::Exponentiel => {
                    NombreLong(n1.checked_pow(n2 as u32).unwrap_or(0))
                }
                AstOperateurBinaireType::Egal => test(n1 == n2),
                AstOperateurBinaireType::Different => test(n1 != n2),
                AstOperateurBinaireType::Superieur => test(n1 > n2),
                AstOperateurBinaireType::SuperieurEgal => test(n1 >= n2),
                AstOperateurBinaireType::Inferieur => test(n1 < n2),
                AstOperateurBinaireType::InferieurEgal => test(n1 <= n2),
                AstOperateurBinaireType::And => NombreLong(n1 & n2),
                AstOperateurBinaireType::Or => NombreLong(n1 | n2),
                AstOperateurBinaireType::Xor => NombreLong(n1 ^ n2),
                AstOperateurBinaireType::Imp => NombreLong(!n1 | n2),
                AstOperateurBinaireType::Eqv => NombreLong(!(n1 ^ n2)),
                _ => panic!("Opérateur inconnu: {:?}", operateur),
            },
            (NombreSingle(n1), NombreSingle(n2)) => match operateur {
                AstOperateurBinaireType::Plus => NombreSingle(n1 + n2),
                AstOperateurBinaireType::Moins => NombreSingle(n1 - n2),
                AstOperateurBinaireType::Multiplication => NombreSingle(n1 * n2),
                AstOperateurBinaireType::DivisionEntiere => NombreLong((n1 / n2) as i32),
                AstOperateurBinaireType::DivisionReel => NombreSingle(n1 / n2),
                AstOperateurBinaireType::Exponentiel => NombreSingle(n1.powf(n2)),
                AstOperateurBinaireType::Egal => test(n1 == n2),
                AstOperateurBinaireType::Different => test(n1 != n2),
                AstOperateurBinaireType::Superieur => test(n1 > n2),
                AstOperateurBinaireType::SuperieurEgal => test(n1 >= n2),
                AstOperateurBinaireType::Inferieur => test(n1 < n2),
                AstOperateurBinaireType::InferieurEgal => test(n1 <= n2),
                _ => panic!("Opérateur inconnu: {:?}", operateur),
            },
            (NombreDouble(n1), NombreDouble(n2)) => match operateur {
                AstOperateurBinaireType::Plus => NombreDouble(n1 + n2),
                AstOperateurBinaireType::Moins => NombreDouble(n1 - n2),
                AstOperateurBinaireType::Multiplication => NombreDouble(n1 * n2),
                AstOperateurBinaireType::DivisionEntiere => NombreDouble(n1 / n2),
                AstOperateurBinaireType::DivisionReel => NombreDouble(n1 / n2),
                AstOperateurBinaireType::Exponentiel => NombreDouble(n1.powf(n2)),
                AstOperateurBinaireType::Egal => test(n1 == n2),
                AstOperateurBinaireType::Different => test(n1 != n2),
                AstOperateurBinaireType::Superieur => test(n1 > n2),
                AstOperateurBinaireType::SuperieurEgal => test(n1 >= n2),
                AstOperateurBinaireType::Inferieur => test(n1 < n2),
                AstOperateurBinaireType::InferieurEgal => test(n1 <= n2),
                _ => panic!("Opérateur inconnu: {:?}", operateur),
            },
            _ => panic!("Opérateur inconnu: {:?}", operateur),
        })
    } else if let Chaine(s1) = expr1 {
        let s2: String;
        if let Chaine(s02) = expr2 {
            s2 = s02;
        } else {
            panic!("Expression non numérique: {:?}", expr2);
        }
        return Some(match operateur {
            AstOperateurBinaireType::Plus => Chaine(s1 + s2.as_str()),
            _ => panic!("Opérateur inconnu: {:?}", operateur),
        });
    } else {
        panic!("Expression invalide: {:?}", expr1);
    }
}

fn operateur_unaire(
    table_symbole: &mut TableSymbole,
    operateur: &AstOperateurUnaireType,
    exp1: &Rc<AstExpression>,
) -> Option<Valeur> {
    let e1 = exp1.as_ref();
    let expr1 = execute_expression(&(e1.clone()), table_symbole);
    if est_nombre(expr1.clone()) {
        Some(match expr1 {
            NombreInteger(n) => match operateur {
                AstOperateurUnaireType::Plus => expr1,
                AstOperateurUnaireType::Moins => NombreInteger(-n),
                AstOperateurUnaireType::Not => NombreInteger(!n),
                _ => panic!("Opérateur inconnu: {:?}", operateur),
            },
            NombreLong(n) => match operateur {
                AstOperateurUnaireType::Plus => expr1,
                AstOperateurUnaireType::Moins => NombreLong(-n),
                AstOperateurUnaireType::Not => NombreLong(!n),
                _ => panic!("Opérateur inconnu: {:?}", operateur),
            },
            NombreSingle(n) => match operateur {
                AstOperateurUnaireType::Plus => expr1,
                AstOperateurUnaireType::Moins => NombreSingle(-n),
                _ => panic!("Opérateur inconnu: {:?}", operateur),
            },
            NombreDouble(n) => match operateur {
                AstOperateurUnaireType::Plus => expr1,
                AstOperateurUnaireType::Moins => NombreDouble(-n),
                _ => panic!("Opérateur inconnu: {:?}", operateur),
            },
            _ => panic!("Opérateur inconnu: {:?}", operateur),
        })
    } else {
        panic!("Expression invalide: {:?}", expr1);
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

fn test(exp: bool) -> Valeur {
    NombreInteger(if exp { TRUE } else { FALSE })
}
