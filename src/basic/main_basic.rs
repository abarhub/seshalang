use crate::basic::exec_basic::execute;
use crate::basic::lexer_basic::TypeMotCle;
use crate::basic::main_basic::AstInstruction::{AstAffectation, AstAppelMethode, AstGoto, AstIf};
use crate::basic::parsing_basic::parse_basic;
use std::fs;
use std::rc::Rc;

#[derive(Debug)]
pub struct AstProgramme {
    pub liste_instructions: Vec<AstInstruction>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstLabelEnum {
    AstLabel(String),
}

#[derive(Debug, PartialEq)]
pub enum AstInstruction {
    AstAffectation(String, AstExpression, Option<AstLabelEnum>),
    AstAppelMethode(String, Vec<AstExpression>, Vec<bool>, Option<AstLabelEnum>),
    AstInstrLabel(Option<AstLabelEnum>),
    AstIf(AstExpression, Option<AstLabelEnum>, AstLabelEnum),
    AstGoto(Option<AstLabelEnum>, AstLabelEnum),
}

impl AstInstruction {
    pub(crate) fn getLabel(&self) -> Option<AstLabelEnum> {
        match self {
            AstInstruction::AstInstrLabel(label) => label.clone(),
            AstAffectation(_, _, label) => label.clone(),
            AstAppelMethode(_, _, _, label) => label.clone(),
            AstIf(_, label, _) => label.clone(),
            AstGoto(label, _) => label.clone(),
        }
    }
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
pub enum AstNomMethode {
    AstNomFonction(String),
    AstNomReserve(TypeMotCle),
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
    AstFonction(AstNomMethode, Vec<AstExpression>),
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
        assert_eq!(sortie[0], "0"); //EQ
        assert_eq!(sortie[1], "-1");
        assert_eq!(sortie[2], "0");
        assert_eq!(sortie[3], "-1");
    }

    #[test]
    fn test_parse_execute16() {
        let fichier = "print 3>2 EQV 0".to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 1);
        assert_eq!(sortie[0], "0"); //EQ
    }

    #[test]
    fn test_parse_execute17() {
        let fichier = "print abs(-10)
                              print abs(-15.10)"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 2);
        assert_eq!(sortie[0], "10");
        assert_eq!(sortie[1], "15.1");
    }

    #[test]
    fn test_parse_execute18() {
        let fichier = "print asc(\"A\")
                              print chr$(66)"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 2);
        assert_eq!(sortie[0], "65");
        assert_eq!(sortie[1], "B");
    }

    #[test]
    fn test_parse_execute19() {
        let fichier = "print atn(0)
                              print cos(0)
                              print sin(0)
                              print tan(0)
                              print cdbl(3)
                              print csng(5)
                              print cint(4.5)
                              print clng(8.5)"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 8);
        assert_eq!(sortie[0], "0");
        assert_eq!(sortie[1], "1");
        assert_eq!(sortie[2], "0");
        assert_eq!(sortie[3], "0");
        assert_eq!(sortie[4], "3");
        assert_eq!(sortie[5], "5");
        assert_eq!(sortie[6], "4");
        assert_eq!(sortie[7], "8");
    }

    #[test]
    fn test_parse_execute20() {
        let fichier = "print mki$(5%)
                              print mkl$(10&)
                              print mks$(15.632!)
                              print mkd$(20.951#)
                              print cvi(\"8\")
                              print cvl(\"10\")
                              print cvs(\"4.5\")
                              print cvd(\"8.5\")"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 8);
        assert_eq!(sortie[0], "5");
        assert_eq!(sortie[1], "10");
        assert_eq!(sortie[2], "15.632");
        assert_eq!(sortie[3], "20.951");
        assert_eq!(sortie[4], "8");
        assert_eq!(sortie[5], "10");
        assert_eq!(sortie[6], "4.5");
        assert_eq!(sortie[7], "8.5");
    }

    #[test]
    fn test_parse_execute21() {
        let fichier = "print fix(5.2)
                              print fix(5.5)
                              print int(6.4)
                              print int(6.7)
                              print instr(1,\"abcdefcd\",\"cd\")
                              print instr(4,\"abcdefcd\",\"cd\")
                              print instr(\"abcabcaaa\",\"bc\")
                              print instr(\"abcabcaaa\",\"xy\")
                              print hex$(20)
                              print oct$(20)"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 10);
        assert_eq!(sortie[0], "5"); // fix
        assert_eq!(sortie[1], "5");
        assert_eq!(sortie[2], "6"); // int
        assert_eq!(sortie[3], "7");
        assert_eq!(sortie[4], "3"); // instr
        assert_eq!(sortie[5], "7");
        assert_eq!(sortie[6], "2");
        assert_eq!(sortie[7], "0");
        assert_eq!(sortie[8], "14"); // hex
        assert_eq!(sortie[9], "24"); // oct
    }

    #[test]
    fn test_parse_execute22() {
        let fichier = "print 1
                              goto label1
                              print 2
                              label1: print 3"
            .to_string();
        let programme = parse_basic(fichier).unwrap();
        let sortie = execute(programme);
        assert_eq!(sortie.len(), 2);
        assert_eq!(sortie[0], "1");
        assert_eq!(sortie[1], "3");
    }
}
