use std::collections::HashMap;
use crate::main_basic::{AstExpression, AstProgramme, TypeExpression, TypeInstruction};

pub fn execute(programme: AstProgramme) -> Vec<String> {
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