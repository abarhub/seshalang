use crate::exec_basic::execute;
use crate::parsing_basic::parse_basic;
use std::cmp::PartialEq;
use std::fs;
use std::rc::Rc;

#[derive(Debug)]
pub struct AstProgramme {
    pub liste_instructions: Vec<AstInstruction>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TypeInstruction {
    Affectation,
    AppelMethode,
}

#[derive(Debug)]
pub enum AstInstruction {
    AstAffectation(String, AstExpression),
    AstAppelMethode(String, Vec<AstExpression>),
    // pub type_instruction: TypeInstruction,
    // pub nom: String,
    // pub liste_expression: Vec<AstExpression>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TypeExpression {
    Nombre,
    Identifiant,
    OperateurBinaire,
}

#[derive(Debug, Clone)]
pub enum AstExpression {
    AstNombre(u32),
    AstIdentifiant(String),
    AstOperateurBinaire(String,Rc<AstExpression>,Rc<AstExpression>),


    // pub type_expression: TypeExpression,
    // pub identifiant: String,
    // pub nombre: u32,
    // pub operateur: String,
    // pub exp: Rc<Option<AstExpression>>,
    // pub exp2: Rc<Option<AstExpression>>,
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::exec_basic::execute;

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
