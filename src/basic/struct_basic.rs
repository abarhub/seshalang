use crate::basic::lexer_basic::TypeMotCle;
use crate::basic::struct_basic::AstInstruction::{
    AstAffectation, AstAppelMethode, AstGoto, AstIf, AstInput,
};
use std::rc::Rc;

#[derive(Debug)]
pub struct AstProgramme {
    pub liste_instructions: Vec<AstInstruction>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstLabelEnum {
    AstLabel(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstInstruction {
    AstAffectation(String, AstExpression),
    AstAppelMethode(String, Vec<AstExpression>, Vec<bool>),
    AstInstrLabel(AstLabelEnum),
    AstIf(AstExpression, AstLabelEnum),
    AstGoto(AstLabelEnum),
    AstInput(Option<String>, Vec<String>, bool, bool),
    AstEnd()
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
