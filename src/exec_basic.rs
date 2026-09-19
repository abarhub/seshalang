use crate::main_basic::{
    AstExpression, AstInstruction, AstProgramme, TypeExpression, TypeInstruction,
};
use std::collections::HashMap;

pub fn execute(programme: AstProgramme) -> Vec<String> {
    let mut sortie: Vec<String> = Vec::new();
    let mut contexte: HashMap<String, i32> = HashMap::new();

    for instruction in programme.liste_instructions.iter() {
        if let AstInstruction::AstAffectation(ident, expression) = instruction {
            let resultat = execute_expression(&expression, &mut contexte);
            contexte.insert(ident.clone(), resultat);
        } else if let AstInstruction::AstAppelMethode(nom, parametres) = instruction {
            if nom.to_lowercase() == "print" {
                for expression in parametres.iter() {
                    let resultat = execute_expression(&expression, &mut contexte);
                    println!("{}", resultat);
                    sortie.push(resultat.to_string());
                }
            } else {
                eprintln!("appel de méthode {} inconnue: {:?}", nom, instruction);
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
    if let AstExpression::AstNombre(nombre) = expression {
        *nombre as i32
    } else if let AstExpression::AstIdentifiant(ident) = expression {
        return contexte.get(ident).unwrap().clone();
    } else if let AstExpression::AstOperateurBinaire(operateur, exp1, exp2) = expression {
        let e1 = exp1.as_ref();
        let e2 = exp2.as_ref();
        let expr1 = execute_expression(&(e1.clone()), contexte);
        let expr2 = execute_expression(&(e2.clone()), contexte);
        return match operateur.as_str() {
            "+" => expr1 + expr2,
            "-" => expr1 - expr2,
            "*" => expr1 * expr2,
            "/" => expr1 / expr2,
            _ => panic!("Opérateur inconnu: {}", operateur),
        };
    } else {
        panic!("Expression inconnue: {:?}", expression);
    }
}
