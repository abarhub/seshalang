use crate::basic::exec_basic::execute;
use crate::basic::parsing_basic::parse_basic;
use std::fs;
use std::rc::Rc;

#[derive(Debug)]
pub struct AstProgramme {
    pub liste_instructions: Vec<AstInstruction>,
}

#[derive(Debug, PartialEq)]
pub enum AstInstruction {
    AstAffectation(String, AstExpression),
    AstAppelMethode(String, Vec<AstExpression>, Vec<bool>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstOperateurBinaireType {
    Plus,
    Moins,
    Multiplication,
    DivisionReel,
    DivisionEntiere,
    Mod,
    Egal,
    Different,
    Superieur,
    SuperieurEgal,
    Inferieur,
    InferieurEgal,
    And,
    Or,
    Xor,
    Imp,
    Eqv,
    Exponentiel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstOperateurUnaireType {
    Plus,
    Moins,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstExpression {
    AstNombreInteger(i16),
    AstNombreLong(i32),
    AstNombreSingle(f32),
    AstNombreDouble(f64),
    AstIdentifiant(String),
    AstOperateurBinaire(
        AstOperateurBinaireType,
        Rc<AstExpression>,
        Rc<AstExpression>,
    ),
    AstOperateurUnnaire(AstOperateurUnaireType, Rc<AstExpression>),
    AstChaine(String),
}

pub fn parse_basic_file(fichier: String) -> std::io::Result<()> {
    let contenu = fs::read_to_string(fichier)?;

    let programme = parse_basic(contenu)?;

    execute(programme);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::basic::parsing_basic::parse_basic;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::basic::exec_basic::execute;

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
        let fichier = "print 8, 20".to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 2);
        assert_eq!(sortie[0], "8");
        assert_eq!(sortie[1], "20");
    }

    #[test]
    fn test_parse_execute5() {
        let fichier = "print 10; 30".to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 2);
        assert_eq!(sortie[0], "10");
        assert_eq!(sortie[1], "30");
    }

    #[test]
    fn test_parse_execute6() {
        let fichier = "print \"abc\"".to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 1);
        assert_eq!(sortie[0], "abc");
    }

    #[test]
    fn test_parse_execute7() {
        let fichier = "x=\"XYZ t1\"
                                print x"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 1);
        assert_eq!(sortie[0], "XYZ t1");
    }

    #[test]
    fn test_parse_execute8() {
        let fichier = "x=\"ABC\" + \"DEF\"
                                print x"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 1);
        assert_eq!(sortie[0], "ABCDEF");
    }

    #[test]
    fn test_parse_execute9() {
        let fichier = "x=2*3+6*7
                                print x"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 1);
        assert_eq!(sortie[0], "48");
    }

    #[test]
    fn test_parse_execute10() {
        let fichier = "print 6\\3
                              print 6*3
                              print 6+3
                              print 6-3"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 4);
        assert_eq!(sortie[0], "2");
        assert_eq!(sortie[1], "18");
        assert_eq!(sortie[2], "9");
        assert_eq!(sortie[3], "3");
    }

    #[test]
    fn test_parse_execute11() {
        let fichier = "print 6.3\\3
                              print 6.1*3
                              print 6.1+3
                              print 6.1-3"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 4);
        assert_eq!(sortie[0], "2");
        assert_eq!(sortie[1], "18.3");
        assert_eq!(sortie[2], "9.1");
        assert_eq!(sortie[3], "3.1");
    }

    #[test]
    fn test_parse_execute12() {
        let fichier = "print 5>3
                              print 5<5
                              print 5<=5
                              print 5<6
                              print 5<3
                              print 7=7
                              print 5=7
                              print 7<>7
                              print 7<>5
                              print 7 MOD 3
                              print 2^3"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 11);
        assert_eq!(sortie[0], "-1");
        assert_eq!(sortie[1], "0");
        assert_eq!(sortie[2], "-1");
        assert_eq!(sortie[3], "-1");
        assert_eq!(sortie[4], "0");
        assert_eq!(sortie[5], "-1");
        assert_eq!(sortie[6], "0");
        assert_eq!(sortie[7], "0");
        assert_eq!(sortie[8], "-1");
        assert_eq!(sortie[9], "1");
        assert_eq!(sortie[10], "8");
    }

    #[test]
    fn test_parse_execute13() {
        let fichier = "print 7  MOD 3".to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 1);
        assert_eq!(sortie[0], "1");
    }

    #[test]
    fn test_parse_execute14() {
        let fichier = "print 5>3 AND 5<3
                              print 5>3 AND 5>3
                              print 5>3 OR 5<3
                              print 5>3 OR 5>3
                              print 5<3 OR 5<3
                              print 5>3 XOR 5<3
                              print 5>3 XOR 5>3
                              print 5>3 EQV 5<3
                              print 5>3 EQV 5>3
                              print 5<3 EQV 5>3
                              print 5<3 EQV 5<3
                              print 5>3 IMP 5<3
                              print 5>3 IMP 5>3
                              print 5<3 IMP 5>3
                              print 5<3 IMP 5<3
                              print -7
                              print +7
                              print NOT (5>3)"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 18);
        assert_eq!(sortie[0], "0"); // AND
        assert_eq!(sortie[1], "-1");
        assert_eq!(sortie[2], "-1"); // OR
        assert_eq!(sortie[3], "-1");
        assert_eq!(sortie[4], "0");
        assert_eq!(sortie[5], "-1"); //XOR
        assert_eq!(sortie[6], "0");
        assert_eq!(sortie[7], "0"); //EQ
        assert_eq!(sortie[8], "-1");
        assert_eq!(sortie[9], "0");
        assert_eq!(sortie[10], "-1");
        assert_eq!(sortie[11], "0"); //IMP
        assert_eq!(sortie[12], "-1");
        assert_eq!(sortie[13], "-1");
        assert_eq!(sortie[14], "-1");
        assert_eq!(sortie[15], "-7"); // -
        assert_eq!(sortie[16], "7"); // +
        assert_eq!(sortie[17], "0"); // NOT
    }

    #[test]
    fn test_parse_execute15() {
        let fichier = "t=5>3
                              f=5<3
                              print t EQV f
                              print t EQV t
                              print f EQV t
                              print f EQV f"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 4);
        let t=-1;
        let f=0;
        let x=!(t ^ f);
        println!("{:?}", x);
        assert_eq!(sortie[0], "0"); //EQ
        assert_eq!(sortie[1], "-1");
        assert_eq!(sortie[2], "0");
        assert_eq!(sortie[3], "-1");
    }


    #[test]
    fn test_parse_execute16() {
        let fichier = "print 3>2 EQV 0"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 1);
        let t=-1;
        let f=0;
        let x=!(t ^ f);
        println!("{:?}", x);
        assert_eq!(sortie[0], "0"); //EQ
    }
}
