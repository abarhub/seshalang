use crate::basic::lexer_basic::Token::TokenNombre;
use crate::basic::lexer_basic::TypeMotCle::{Cls, Defdbl, Defint, Deflng, Defsng, Defstr, Do, Else, ElseIf, End, For, Goto, If, Input, Loop, Next, Step, Then, To, Until, Wend, While};
use crate::basic::lexer_basic::TypeNombre::{Entier, Long, Single};
use crate::basic::lexer_basic::TypeSeparateur::{
    DEUXPOINT, DIFF, DIVE, DIVR, EQU, EXP, FOIS, INF, INFE, MOINS, PARENTF, PARENTO, PLUS,
    POINTVIRGULE, SUP, SUPE, VIRGULE,
};
use crate::basic::lexer_basic::{Token, TypeMotCle, TypeNombre, TypeSeparateur, lexer};

use crate::basic::struct_basic::AstExpression::{
    AstIdentifiant, AstNombreInteger, AstOperateurBinaire,
};
use crate::basic::struct_basic::AstInstruction::{AstAffectation, AstDecl, AstEnd, AstGoto, AstIf, AstInput, AstInstrLabel};
use crate::basic::struct_basic::AstLabelEnum::AstLabel;
use crate::basic::struct_basic::AstOperateurBinaireType::{
    Different, DivisionEntiere, DivisionReel, Egal, Exponentiel, Inferieur, InferieurEgal, Moins,
    Multiplication, Plus, Superieur, SuperieurEgal,
};
use crate::basic::struct_basic::{AstExpression, AstInstruction, AstLabelEnum, AstNomMethode, AstOperateurBinaireType, AstOperateurUnaireType, AstProgramme, TypeVariable};
use std::cmp::max;
use std::rc::Rc;
use std::slice::Iter;

const DEBUG_PRINT: bool = false;

#[derive(Debug)]
struct TokenIterator {
    liste_tokens: Vec<Token>,
    index: i32,
}

impl TokenIterator {
    fn new(liste: Vec<Token>) -> TokenIterator {
        TokenIterator {
            liste_tokens: liste.clone(),
            index: -1,
        }
    }

    fn get(&self) -> Option<Token> {
        if self.index + 1 >= 0 && self.index + 1 < self.liste_tokens.len() as i32 {
            Some(self.liste_tokens[(self.index + 1) as usize].clone())
        } else {
            None
        }
    }

    fn get_pos(&self, position: i32) -> Option<Token> {
        if self.index + 1 + position >= 0
            && self.index + 1 + position < self.liste_tokens.len() as i32
        {
            Some(self.liste_tokens[(self.index + 1 + position) as usize].clone())
        } else {
            None
        }
    }

    fn est_type_token(&self, type_token: Token) -> bool {
        if let Some(t) = self.get() {
            t == type_token
        } else {
            false
        }
    }

    fn est_identifiant(&self) -> bool {
        if let Some(Token::TokenIdentifiant(_, _)) = self.get() {
            true
        } else {
            false
        }
    }

    fn est_identifiant_pos(&self, position: i32) -> bool {
        if let Some(Token::TokenIdentifiant(_, _)) = self.get_pos(position) {
            true
        } else {
            false
        }
    }

    fn est_mot_cle(&self, mot_cle: TypeMotCle) -> bool {
        if let Some(Token::TokenMotReserve(mot_reserve, _)) = self.get() {
            mot_cle == mot_reserve
        } else {
            false
        }
    }

    fn est_mot_cle_pos(&self, mot_cle: TypeMotCle, position: i32) -> bool {
        if let Some(Token::TokenMotReserve(mot_reserve, _)) = self.get_pos(position) {
            mot_cle == mot_reserve
        } else {
            false
        }
    }

    fn est_mot_cle_fonction(&self) -> bool {
        if let Some(Token::TokenMotReserve(mot_reserve, _)) = self.get() {
            mot_reserve.est_fonction()
        } else {
            false
        }
    }

    fn est_separateur_pos(&self, separateur: TypeSeparateur, position: i32) -> bool {
        if let Some(Token::TokenSeparateur(x, _)) = self.get_pos(position) {
            x == separateur
        } else {
            false
        }
    }

    fn est_separateur(&self, separateur: TypeSeparateur) -> bool {
        if let Some(Token::TokenSeparateur(x, _)) = self.get() {
            x == separateur
        } else {
            false
        }
    }

    fn est_nombre(&self) -> bool {
        if let Some(Token::TokenNombre(_, _, _, _, _, _)) = self.get() {
            true
        } else {
            false
        }
    }

    fn est_nombre_pos(&self, position: i32) -> bool {
        if let Some(Token::TokenNombre(_, _, _, _, _, _)) = self.get_pos(position) {
            true
        } else {
            false
        }
    }

    fn est_chaine(&self) -> bool {
        if let Some(Token::TokenChaine(_, _)) = self.get() {
            true
        } else {
            false
        }
    }

    fn est_chaine_pos(&self, position: i32) -> bool {
        if let Some(Token::TokenChaine(_, _)) = self.get_pos(position) {
            true
        } else {
            false
        }
    }

    fn est_label(&self) -> bool {
        if let Some(Token::TokenIdentifiant(_, _)) = self.get() {
            let mut i = 1;
            while self.est_identifiant_pos(i) {
                i += 1;
            }
            self.est_separateur_pos(DEUXPOINT, i)
        } else {
            false
        }
    }

    fn len(&self) -> usize {
        if self.index < 0 {
            self.liste_tokens.len()
        } else {
            max(self.liste_tokens.len() - (self.index + 1) as usize, 0)
        }
    }
}

impl Iterator for TokenIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.index + 1 >= 0 && self.index + 1 < self.liste_tokens.len() as i32 {
            self.index += 1;
            let c = self.liste_tokens.get(self.index as usize).unwrap();
            Some(c.clone())
        } else {
            self.index = self.liste_tokens.len() as i32;
            None
        }
    }
}

#[derive(Debug)]
struct Compteur {
    compteur_if: i32,
}

impl Compteur {
    fn new() -> Compteur {
        Compteur { compteur_if: 1 }
    }

    fn next_if(&mut self) -> i32 {
        let no = self.compteur_if;
        self.compteur_if += 1;
        no
    }
}

pub fn parse_basic(contenu_fichier: String) -> std::io::Result<AstProgramme> {
    let liste_tokens: Vec<Vec<Token>> = lexer(contenu_fichier);

    if DEBUG_PRINT {
        println!("liste token : {:?}", liste_tokens);
    }

    let programme = parsing_programme(liste_tokens);

    Ok(programme)
}

fn parsing_programme(liste_tokens: Vec<Vec<Token>>) -> AstProgramme {
    let mut programme = AstProgramme {
        liste_instructions: Vec::new(),
    };

    let mut compteur = Compteur::new();

    let mut iter = liste_tokens.iter();

    while let Some(ligne) = iter.next() {
        if ligne.len() == 0 {
            continue;
        }

        let mut token_iterator = TokenIterator::new(ligne.clone());

        let mut label: Option<AstLabelEnum> = None;

        // le label
        if token_iterator.est_nombre() {
            let nombre = token_iterator.next().unwrap();
            match nombre.clone() {
                TokenNombre(n16, n32, _, _, typeNombre, _) => {
                    label = {
                        match typeNombre {
                            Entier => Some(AstLabel(n16.to_string())),
                            Long => Some(AstLabel(n32.to_string())),
                            _ => {
                                panic!("token {:?} n'est pas un nombre entier", nombre);
                            }
                        }
                    }
                }
                _ => {
                    panic!("token {:?} n'est pas un nombre", nombre);
                }
            }
        } else if token_iterator.est_label() {
            let label_ident = token_iterator.next().unwrap();
            if let Token::TokenIdentifiant(ident, _) = label_ident {
                label = Some(AstLabel(ident));
                let deux_point = token_iterator.next().unwrap();
                if let Token::TokenSeparateur(DEUXPOINT, _) = deux_point {} else {
                    panic!("token {:?} n'est pas un deux point", deux_point);
                }
            } else {
                panic!("token {:?} n'est pas un label", label_ident);
            }
        }

        // les instructions
        let mut liste_instructions =
            parse_instruction(&mut token_iterator, label, &mut iter, &mut compteur);
        if !liste_instructions.is_empty() {
            programme.liste_instructions.append(&mut liste_instructions);
        }
    }

    programme
}

fn parse_instruction(
    mut token_iterator: &mut TokenIterator,
    label: Option<AstLabelEnum>,
    liste_instructions_iter: &mut Iter<Vec<Token>>,
    compteur: &mut Compteur,
) -> Vec<AstInstruction> {
    let mut debut_instr = vec![];

    if label.is_some() {
        let instruction = AstInstrLabel(label.unwrap());
        debut_instr.push(instruction);
    }
    if token_iterator.len() == 0 {
        return debut_instr;
    }
    if token_iterator.est_identifiant() && token_iterator.est_separateur_pos(EQU, 1) {
        // affectation
        affectation(&mut token_iterator, &mut debut_instr);
        debut_instr
    } else if token_iterator.est_identifiant() {
        // appel de méthode
        parse_appel_methode(&mut token_iterator, &mut debut_instr);
        debut_instr
    } else if token_iterator.est_mot_cle(TypeMotCle::Print) {
        parse_print(&mut token_iterator, &mut debut_instr);
        debut_instr
    } else if token_iterator.est_mot_cle(Goto) {
        parse_goto(&mut token_iterator, &mut debut_instr);
        debut_instr
    } else if token_iterator.est_mot_cle(If) {
        parse_if(
            &mut token_iterator,
            liste_instructions_iter,
            compteur,
            &mut debut_instr,
        );
        debut_instr
    } else if token_iterator.est_mot_cle(Input) {
        parse_input(&mut token_iterator, &mut debut_instr);
        debut_instr
    } else if token_iterator.est_mot_cle(End) {
        parse_end(&mut token_iterator, &mut debut_instr);
        debut_instr
    } else if token_iterator.est_mot_cle(While) {
        parse_while(
            &mut token_iterator,
            &mut debut_instr,
            compteur,
            liste_instructions_iter,
        );
        debut_instr
    } else if token_iterator.est_mot_cle(Do) {
        parse_do_while(
            &mut token_iterator,
            &mut debut_instr,
            compteur,
            liste_instructions_iter,
        );
        debut_instr
    } else if token_iterator.est_mot_cle(For) {
        parse_for(
            &mut token_iterator,
            &mut debut_instr,
            compteur,
            liste_instructions_iter,
        );
        debut_instr
    } else if token_iterator.est_mot_cle(Cls) {
        // ne fait rien
        debut_instr
    } else if token_iterator.est_mot_cle(Defint) || token_iterator.est_mot_cle(Deflng) || token_iterator.est_mot_cle(Defsng) || token_iterator.est_mot_cle(Defdbl) || token_iterator.est_mot_cle(Defstr) {
        parse_declaration_defaut(
            &mut token_iterator,
            &mut debut_instr,
        );
        debut_instr
    } else {
        panic!("instruction inconnue {:?}", token_iterator.get());
    }
}

fn parse_declaration_defaut(token_iterator: &mut &mut TokenIterator, debut_instr: &mut Vec<AstInstruction>) {
    let type_variables;

    if let Some(Token::TokenMotReserve(mot_reserve, _)) = token_iterator.next() {
        if mot_reserve == Defint {
            type_variables = TypeVariable::Integer;
        } else if mot_reserve == Deflng {
            type_variables = TypeVariable::Long;
        } else if mot_reserve == Defsng {
            type_variables = TypeVariable::Single;
        } else if mot_reserve == Defdbl {
            type_variables = TypeVariable::Double;
        } else if mot_reserve == Defstr {
            type_variables = TypeVariable::String;
        } else {
            panic!("token {:?} n'est pas un mot clé de déclaration", token_iterator.get());
        }
    } else {
        panic!("token {:?} n'est pas un mot clé de déclaration", token_iterator.get());
    }

    let mut liste_variables = Vec::new();

    loop {
        let variable;

        if token_iterator.est_identifiant() {
            if let Token::TokenIdentifiant(ident, _) = token_iterator.next().unwrap() {
                variable = ident;
            } else {
                panic!("token {:?} n'est pas un identifiant", token_iterator.get());
            }
        } else {
            panic!("token {:?} n'est pas un identifiant", token_iterator.get());
        }

        if token_iterator.est_separateur(MOINS) {
            token_iterator.next();
            let variable2;
            if let Token::TokenIdentifiant(ident, _) = token_iterator.next().unwrap() {
                variable2 = ident;
                let tmp = vec![variable, variable2];
                liste_variables.push(tmp);
            } else {
                panic!("token {:?} n'est pas un identifiant", token_iterator.get());
            }
        } else {
            let tmp = vec![variable];
            liste_variables.push(tmp);
        }
        if token_iterator.est_separateur(VIRGULE) {
            token_iterator.next();
        } else {
            break;
        }
    }

    debut_instr.push(AstDecl(type_variables, liste_variables));
}

fn parse_for(
    mut token_iterator: &mut TokenIterator,
    debut_instr: &mut Vec<AstInstruction>,
    compteur: &mut Compteur,
    liste_instructions_iter: &mut Iter<Vec<Token>>,
) {
    token_iterator.next();

    let variable;

    if token_iterator.est_identifiant() {
        if let Token::TokenIdentifiant(ident, _) = token_iterator.next().unwrap() {
            variable = ident;
        } else {
            panic!("token {:?} n'est pas un identifiant", token_iterator.get());
        }
    } else {
        panic!("token {:?} n'est pas un identifiant", token_iterator.get());
    }

    if token_iterator.est_separateur(EQU) {
        token_iterator.next();
    } else {
        panic!("token {:?} n'est pas un séparateur", token_iterator.get());
    }

    let exp1 = parsing_expression(&mut token_iterator);

    if token_iterator.est_mot_cle(To) {
        token_iterator.next();
    } else {
        panic!("token {:?} n'est pas to", token_iterator.get());
    }

    let exp2 = parsing_expression(&mut token_iterator);

    let mut exp3 = AstNombreInteger(1);
    if token_iterator.est_mot_cle(Step) {
        token_iterator.next();

        let exp03 = parsing_expression(&mut token_iterator);
        exp3 = exp03.0;
    }

    let mut liste_instruction_actif: Vec<AstInstruction> = vec![];

    while let Some(ligne) = liste_instructions_iter.next() {
        let mut token_iterator2 = TokenIterator::new(ligne.clone());

        if token_iterator2.est_mot_cle(Next) {
            token_iterator2.next();

            break;
        } else {
            // les intructions
            let v: Vec<Vec<Token>> = vec![];
            let mut iter = v.iter();

            let mut liste_instructions2 =
                parse_instruction(&mut token_iterator2, None, &mut iter, compteur);
            liste_instruction_actif.append(&mut liste_instructions2);
        }
    }

    let no = compteur.next_if();
    let label_debut = AstLabel(format!("_for_debut{}", no).to_string());
    let label_suite = AstLabel(format!("_for_suite{}", no).to_string());
    let label_fin = AstLabel(format!("_for_fin{}", no).to_string());
    let label_inf = AstLabel(format!("_for_inf{}", no).to_string());
    let step = format!("_step{}", no);

    let exp_fin_sup = AstOperateurBinaire(
        Superieur,
        Rc::new(AstIdentifiant(variable.clone())),
        Rc::new(exp2.0.clone()),
    );

    let exp_fin_inf = AstOperateurBinaire(
        Inferieur,
        Rc::new(AstIdentifiant(variable.clone())),
        Rc::new(exp2.0),
    );

    let incr = AstOperateurBinaire(
        Plus,
        Rc::new(AstIdentifiant(variable.clone())),
        Rc::new(AstIdentifiant(step.clone())),
    );

    let exp_test_step = AstOperateurBinaire(
        Inferieur,
        Rc::new(AstIdentifiant(step.clone())),
        Rc::new(AstNombreInteger(0)),
    );

    debut_instr.push(AstAffectation(variable.clone(), exp1.0));
    debut_instr.push(AstAffectation(step.clone(), exp3.clone()));

    debut_instr.push(AstInstrLabel(label_debut.clone()));

    debut_instr.push(AstIf(exp_test_step, label_inf.clone()));

    // test si variable > max
    debut_instr.push(AstIf(exp_fin_sup, label_fin.clone()));

    debut_instr.push(AstGoto(label_suite.clone()));

    // test si variable < min
    debut_instr.push(AstInstrLabel(label_inf));

    debut_instr.push(AstIf(exp_fin_inf, label_fin.clone()));

    // la boucle d'instruction
    debut_instr.push(AstInstrLabel(label_suite));

    debut_instr.append(&mut liste_instruction_actif);

    debut_instr.push(AstAffectation(variable.clone(), incr));

    debut_instr.push(AstGoto(label_debut));

    debut_instr.push(AstInstrLabel(label_fin));
}

fn parse_do_while(
    mut token_iterator: &mut TokenIterator,
    debut_instr: &mut Vec<AstInstruction>,
    compteur: &mut Compteur,
    liste_instructions_iter: &mut Iter<Vec<Token>>,
) {
    token_iterator.next();

    let mut debut_while = false;
    let mut debut_until = false;
    let mut fin_while = false;
    let mut fin_until = false;
    let mut exp_debut: Option<AstExpression> = None;
    let mut exp_fin: Option<AstExpression> = None;

    if token_iterator.est_mot_cle(While) {
        token_iterator.next();
        debut_while = true;
    } else if token_iterator.est_mot_cle(Until) {
        token_iterator.next();
        debut_until = true;
    }

    if (debut_while || debut_until) {
        let exp = parsing_expression(&mut token_iterator);
        exp_debut = Some(exp.0);
    }

    let mut liste_instruction_actif: Vec<AstInstruction> = vec![];

    while let Some(ligne) = liste_instructions_iter.next() {
        let mut token_iterator2 = TokenIterator::new(ligne.clone());

        if token_iterator2.est_mot_cle(Loop) {
            token_iterator2.next();

            if token_iterator2.est_mot_cle(While) {
                token_iterator2.next();
                fin_while = true;
            } else if token_iterator2.est_mot_cle(Until) {
                token_iterator2.next();
                fin_until = true;
            }

            if fin_while || fin_until {
                let exp = parsing_expression(&mut token_iterator2);
                exp_fin = Some(exp.0);
            }

            break;
        } else {
            // les intructions
            let v: Vec<Vec<Token>> = vec![];
            let mut iter = v.iter();

            let mut liste_instructions2 =
                parse_instruction(&mut token_iterator2, None, &mut iter, compteur);
            liste_instruction_actif.append(&mut liste_instructions2);
        }
    }

    if (debut_while || debut_until) && (fin_while || fin_until) {
        panic!("while ou until au début et à la fin");
    }

    let no = compteur.next_if();
    let label_debut = AstLabel(format!("_do_while_debut{}", no).to_string());
    let label_suite = AstLabel(format!("_do_while_suite{}", no).to_string());
    let label_fin = AstLabel(format!("_do_while_fin{}", no).to_string());

    if fin_while || fin_until {
        debut_instr.push(AstGoto(label_suite.clone()));
    }

    debut_instr.push(AstInstrLabel(label_debut.clone()));

    if debut_while || fin_while {
        if debut_while {
            debut_instr.push(AstIf(exp_debut.unwrap(), label_suite.clone()));
        } else {
            debut_instr.push(AstIf(exp_fin.unwrap(), label_suite.clone()));
        }
        debut_instr.push(AstGoto(label_fin.clone()));
    } else if debut_until || fin_until {
        if debut_until {
            debut_instr.push(AstIf(exp_debut.unwrap(), label_fin.clone()));
        } else {
            debut_instr.push(AstIf(exp_fin.unwrap(), label_fin.clone()));
        }
        debut_instr.push(AstGoto(label_suite.clone()));
    }

    debut_instr.push(AstInstrLabel(label_suite));

    debut_instr.append(&mut liste_instruction_actif);

    debut_instr.push(AstGoto(label_debut));

    debut_instr.push(AstInstrLabel(label_fin));
}

fn parse_while(
    mut token_iterator: &mut TokenIterator,
    debut_instr: &mut Vec<AstInstruction>,
    compteur: &mut Compteur,
    liste_instructions_iter: &mut Iter<Vec<Token>>,
) {
    token_iterator.next();
    let exp = parsing_expression(&mut token_iterator);

    let no = compteur.next_if();
    let label_debut = AstLabel(format!("_while_debut{}", no).to_string());
    let label_suite = AstLabel(format!("_while_suite{}", no).to_string());
    let label_fin = AstLabel(format!("_while_fin{}", no).to_string());

    let mut liste_instruction_actif: Vec<AstInstruction> = vec![];

    while let Some(ligne) = liste_instructions_iter.next() {
        let mut token_iterator2 = TokenIterator::new(ligne.clone());

        if token_iterator2.est_mot_cle(Wend) {
            break;
        } else {
            // les intructions
            let v: Vec<Vec<Token>> = vec![];
            let mut iter = v.iter();

            let mut liste_instructions2 =
                parse_instruction(&mut token_iterator2, None, &mut iter, compteur);
            liste_instruction_actif.append(&mut liste_instructions2);
        }
    }

    let instruction = AstInstrLabel(label_debut.clone());
    debut_instr.push(instruction);
    let instruction = AstIf(exp.0, label_suite.clone());
    debut_instr.push(instruction);
    let instruction = AstGoto(label_fin.clone());
    debut_instr.push(instruction);
    let instruction = AstInstrLabel(label_suite);
    debut_instr.push(instruction);
    debut_instr.append(&mut liste_instruction_actif);
    let instruction = AstGoto(label_debut);
    debut_instr.push(instruction);
    let instruction = AstInstrLabel(label_fin);
    debut_instr.push(instruction);
}

fn parse_end(token_iterator: &mut &mut TokenIterator, debut_instr: &mut Vec<AstInstruction>) {
    token_iterator.next();
    let instruction = AstEnd();
    debut_instr.push(instruction);
}
fn parse_goto(token_iterator: &mut &mut TokenIterator, debut_instr: &mut Vec<AstInstruction>) {
    token_iterator.next();
    let identifiant = token_iterator.next().unwrap();
    if let Token::TokenIdentifiant(s, _) = identifiant {
        let instruction = AstGoto(AstLabel(s.to_string()));
        debut_instr.push(instruction);
    } else {
        panic!("token {:?} n'est pas un identifiant", identifiant);
    }
}

fn parse_print(mut token_iterator: &mut &mut TokenIterator, debut_instr: &mut Vec<AstInstruction>) {
    if let Token::TokenMotReserve(_, _) = token_iterator.next().unwrap() {
        let mut liste_expressions: Vec<AstExpression> = vec![];
        let mut separateur_virgule_liste: Vec<bool> = vec![];
        loop {
            let exp = parsing_expression(&mut token_iterator);
            if DEBUG_PRINT {
                let tmp = exp.0.clone();
                println!("parsing appel print exp: {:?}", tmp);
            }
            liste_expressions.push(exp.0);
            if token_iterator.len() > 0 && separateur_parametre_print(token_iterator.get().unwrap())
            {
                let separateur_virgule: bool;
                if let Some(Token::TokenSeparateur(s, _)) = token_iterator.next() {
                    separateur_virgule = s == VIRGULE;
                } else {
                    panic!(
                        "token {:?} n'est pas un séparateur",
                        token_iterator.get().unwrap()
                    );
                };
                separateur_virgule_liste.push(separateur_virgule);
            } else {
                break;
            }
        }

        let instruction = AstInstruction::AstAppelMethode(
            "print".to_string(),
            liste_expressions,
            separateur_virgule_liste,
        );

        debut_instr.push(instruction);
    } else {
        panic!("token {:?} n'est pas un identifiant", token_iterator.get());
    }
}

fn parse_appel_methode(
    mut token_iterator: &mut &mut TokenIterator,
    debut_instr: &mut Vec<AstInstruction>,
) {
    if let Token::TokenIdentifiant(ident, _) = token_iterator.next().unwrap() {
        if DEBUG_PRINT {
            println!("appel {}", ident);
        }
        let mut liste_expressions: Vec<AstExpression> = vec![];
        let mut separateur_virgule_liste: Vec<bool> = vec![];
        loop {
            let exp = parsing_expression(&mut token_iterator);
            let tmp = exp.0.clone();
            if DEBUG_PRINT {
                println!("parsing appel exp: {:?}", tmp);
            }
            liste_expressions.push(exp.0);
            if token_iterator.len() > 0
                && separateur_parametre(&ident, token_iterator.get().unwrap())
            {
                let separateur_virgule: bool;
                if let Some(Token::TokenSeparateur(s, _)) = token_iterator.next() {
                    separateur_virgule = s == VIRGULE;
                } else {
                    panic!(
                        "token {:?} n'est pas un séparateur",
                        token_iterator.get().unwrap()
                    );
                };
                separateur_virgule_liste.push(separateur_virgule);
            } else {
                break;
            }
        }

        let instruction =
            AstInstruction::AstAppelMethode(ident, liste_expressions, separateur_virgule_liste);

        debut_instr.push(instruction);
    } else {
        panic!("token {:?} n'est pas un identifiant", token_iterator.get());
    }
}

fn affectation(mut token_iterator: &mut &mut TokenIterator, debut_instr: &mut Vec<AstInstruction>) {
    if let Token::TokenIdentifiant(ident, _) = token_iterator.next().unwrap() {
        if DEBUG_PRINT {
            println!("affectation {}", ident);
        }
        token_iterator.next();
        let exp = parsing_expression(&mut token_iterator);
        let instruction = AstAffectation(ident.clone(), exp.0);
        debut_instr.push(instruction);
    } else {
        panic!("token {:?} n'est pas un identifiant", token_iterator.get());
    }
}

fn parse_if(
    mut token_iterator: &mut &mut TokenIterator,
    liste_instructions_iter: &mut Iter<Vec<Token>>,
    compteur: &mut Compteur,
    debut_instr: &mut Vec<AstInstruction>,
) {
    token_iterator.next();
    let exp = parsing_expression(&mut token_iterator);
    if token_iterator.est_mot_cle(Then) {
        token_iterator.next();
    } else {
        panic!("token {:?} n'est pas un mot clé Then", token_iterator.get());
    }
    if token_iterator.len() == 0 {
        let no = compteur.next_if();
        let label_if = AstLabel(format!("_if_then{}", no).to_string());
        let label_end = AstLabel(format!("_if_end{}", no).to_string());
        let label_else = AstLabel(format!("_if_else{}", no).to_string());
        let label_else_if_debut = AstLabel(format!("_if_else_if_debut{}", no).to_string());
        let mut label_else_if_debut2 = label_else_if_debut.clone();
        let mut liste_label_else_if: Vec<AstLabelEnum> = vec![];
        let mut liste_instruction = vec![];
        let mut compteur_label_else_if = 1;

        let mut liste_instruction_if: Vec<AstInstruction> = vec![];
        let mut liste_instruction_else: Vec<AstInstruction> = vec![];
        let mut liste_instruction_else_if: Vec<Vec<AstInstruction>> = vec![];
        let mut liste_instruction_actif: &mut Vec<AstInstruction> = &mut liste_instruction_if;
        while let instr = liste_instructions_iter.next() {
            if let Some(ligne) = instr {
                let mut token_iterator2 = TokenIterator::new(ligne.clone());

                if token_iterator2.est_mot_cle(End) && token_iterator2.est_mot_cle_pos(If, 1) {
                    // end if
                    break;
                } else if token_iterator2.est_mot_cle(ElseIf) {
                    // else if
                    if !liste_instruction_else.is_empty() {
                        panic!("elseif apres un else");
                    }
                    token_iterator2.next();
                    liste_instruction_else_if.push(vec![]);
                    liste_instruction_actif = liste_instruction_else_if.last_mut().unwrap();
                    let n = compteur_label_else_if;
                    compteur_label_else_if += 1;
                    let exp = parsing_expression(&mut token_iterator2);

                    if token_iterator2.est_mot_cle(Then) {
                        token_iterator2.next();
                    } else {
                        panic!(
                            "token {:?} n'est pas un mot clé Then",
                            token_iterator2.get()
                        );
                    }

                    let label_else_if = AstLabel(format!("_if_else_if{}_{}", no, n).to_string());
                    let label_else_if_suite =
                        AstLabel(format!("_if_else_if_suite{}_{}", no, n).to_string());
                    liste_instruction_actif.push(AstInstrLabel(label_else_if_debut2.clone()));
                    liste_instruction_actif.push(AstIf(exp.0, label_else_if.clone()));
                    liste_instruction_actif.push(AstGoto(label_else_if_suite.clone()));
                    liste_instruction_actif.push(AstInstrLabel(label_else_if));
                    liste_label_else_if.push(label_else_if_suite.clone());
                    label_else_if_debut2 = label_else_if_suite;
                } else if token_iterator2.est_mot_cle(Else) {
                    // else
                    if !liste_instruction_else.is_empty() {
                        panic!("else apres un else");
                    }
                    liste_instruction_actif = &mut liste_instruction_else;
                } else {
                    // les intructions
                    let v: Vec<Vec<Token>> = vec![];
                    let mut iter = v.iter();

                    let mut liste_instructions2 =
                        parse_instruction(&mut token_iterator2, None, &mut iter, compteur);
                    liste_instruction_actif.append(&mut liste_instructions2);
                }
            } else {
                break;
            }
        }

        let mut label_suite_if = &label_else;
        if !liste_instruction_else_if.is_empty() {
            label_suite_if = &label_else_if_debut;
        }
        // if
        liste_instruction.push(AstIf(exp.0, label_if.clone()));
        liste_instruction.push(AstGoto(label_suite_if.clone()));
        liste_instruction.push(AstInstrLabel(label_if));
        liste_instruction.append(&mut liste_instruction_if);
        liste_instruction.push(AstGoto(label_end.clone()));

        // else if
        if !liste_instruction_else_if.is_empty() {
            for mut liste_instr in liste_instruction_else_if {
                liste_instruction.append(&mut liste_instr);
                liste_instruction.push(AstGoto(label_end.clone()));
            }

            let label_fin_else_if = liste_label_else_if
                .get(liste_label_else_if.len() - 1)
                .unwrap()
                .clone();
            liste_instruction.push(AstInstrLabel(label_fin_else_if));
        }

        // else
        liste_instruction.push(AstInstrLabel(label_else));
        liste_instruction.append(&mut liste_instruction_else);
        liste_instruction.push(AstInstrLabel(label_end));

        for tmp in liste_instruction.clone() {
            println!("{:?}", tmp);
        }

        debut_instr.append(&mut liste_instruction);
    } else {
        let no = compteur.next_if();
        let label_if = AstLabel(format!("_if_then{}", no).to_string());
        let label_end = AstLabel(format!("_if_end{}", no).to_string());
        let label_else = AstLabel(format!("_if_else{}", no).to_string());
        let v: Vec<Vec<Token>> = vec![];
        let mut iter = v.iter();
        let mut instruction = parse_instruction(
            &mut token_iterator,
            Some(label_if.clone()),
            &mut iter,
            compteur,
        );
        let mut instruction_else: Vec<AstInstruction> = vec![];
        if token_iterator.est_mot_cle(Else) {
            token_iterator.next();

            instruction_else = parse_instruction(&mut token_iterator, None, &mut iter, compteur);
        }
        if !instruction.is_empty() || !instruction_else.is_empty() {
            let mut liste_instruction = vec![];
            if instruction_else.is_empty() {
                liste_instruction.push(AstIf(exp.0, label_if));
                liste_instruction.push(AstGoto(label_end.clone()));
                liste_instruction.append(&mut instruction);
                liste_instruction.push(AstInstrLabel(label_end));
            } else {
                liste_instruction.push(AstIf(exp.0, label_if));
                liste_instruction.push(AstGoto(label_else.clone()));
                liste_instruction.append(&mut instruction);
                liste_instruction.push(AstGoto(label_end.clone()));
                liste_instruction.push(AstInstrLabel(label_else));
                liste_instruction.append(&mut instruction_else);
                liste_instruction.push(AstInstrLabel(label_end));
            }
            debut_instr.append(&mut liste_instruction);
        } else {
            panic!("Erreur: pas d'instruction pour le then du if")
        }
    }
}

fn parse_input(token_iterator: &mut &mut TokenIterator, debutInstr: &mut Vec<AstInstruction>) {
    token_iterator.next();
    let retour_ligne: bool;
    if token_iterator.est_separateur(POINTVIRGULE) {
        token_iterator.next();
        retour_ligne = false;
    } else {
        retour_ligne = true;
    }
    let mut prompt: Option<String> = None;
    if token_iterator.est_chaine() {
        if let Token::TokenChaine(chaine, _) = token_iterator.next().unwrap() {
            prompt = Some(chaine.clone());
        }
    }
    let point_interrogation: bool;
    if token_iterator.est_separateur(POINTVIRGULE) {
        token_iterator.next();
        point_interrogation = true;
    } else if token_iterator.est_separateur(VIRGULE) {
        token_iterator.next();
        point_interrogation = false;
    } else {
        point_interrogation = false;
    }
    let mut liste_variables: Vec<String> = Vec::new();
    while token_iterator.est_identifiant() {
        if let Token::TokenIdentifiant(identifiant, _) = token_iterator.next().unwrap() {
            liste_variables.push(identifiant.clone());
        }
        if token_iterator.est_separateur(VIRGULE) {
            token_iterator.next();
        }
    }
    let mut liste_instruction = vec![];
    liste_instruction.push(AstInput(
        prompt,
        liste_variables,
        retour_ligne,
        point_interrogation,
    ));
    debutInstr.append(&mut liste_instruction);
}

fn separateur_parametre(nom_methode: &String, mot: Token) -> bool {
    if nom_methode.to_lowercase() == "print" {
        est_separateur(&mot, VIRGULE) || est_separateur(&mot, POINTVIRGULE)
    } else {
        est_separateur(&mot, VIRGULE)
    }
}

fn separateur_parametre_print(mot: Token) -> bool {
    est_separateur(&mot, VIRGULE) || est_separateur(&mot, POINTVIRGULE)
}

fn est_identifiant(token: &Token) -> bool {
    match token {
        Token::TokenIdentifiant(_, _) => true,
        _ => false,
    }
}

fn est_nombre(token: &Token) -> bool {
    match token {
        Token::TokenNombre(_, _, _, _, _, _) => true,
        _ => false,
    }
}

fn est_separateur(token: &Token, separateur_cherche: TypeSeparateur) -> bool {
    match token {
        Token::TokenSeparateur(separateur, _) => separateur == &separateur_cherche,
        _ => false,
    }
}

fn est_chaine(token: &Token) -> bool {
    match token {
        Token::TokenChaine(_, _) => true,
        _ => false,
    }
}

fn parsing_expression(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    expr_imp(liste_tokens)
}

// expression X IMP Y
fn expr_imp(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    let mut op1 = expr_eqv(liste_tokens);

    while liste_tokens.est_mot_cle(TypeMotCle::Imp) {
        liste_tokens.next().unwrap();
        let op2 = expr_eqv(liste_tokens);
        op1 = (
            AstExpression::AstOperateurBinaire(
                AstOperateurBinaireType::Imp,
                Rc::new(op1.0),
                Rc::new(op2.0),
            ),
            op1.1 + op2.1 + 1,
        )
    }
    op1
}
// expression X EQV Y
fn expr_eqv(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    let mut op1 = expr_or(liste_tokens);

    while liste_tokens.est_mot_cle(TypeMotCle::Eqv) {
        liste_tokens.next().unwrap();
        let op2 = expr_or(liste_tokens);
        op1 = (
            AstExpression::AstOperateurBinaire(
                AstOperateurBinaireType::Eqv,
                Rc::new(op1.0),
                Rc::new(op2.0),
            ),
            op1.1 + op2.1 + 1,
        )
    }
    op1
}
// expression X OR Y ou X XOR Y
fn expr_or(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    let mut op1 = expr_and(liste_tokens);

    while liste_tokens.est_mot_cle(TypeMotCle::Or) || liste_tokens.est_mot_cle(TypeMotCle::Xor) {
        let operateur = if liste_tokens.est_mot_cle(TypeMotCle::Or) {
            AstOperateurBinaireType::Or
        } else {
            AstOperateurBinaireType::Xor
        };
        liste_tokens.next().unwrap();
        let op2 = expr_and(liste_tokens);
        op1 = (
            AstExpression::AstOperateurBinaire(operateur, Rc::new(op1.0), Rc::new(op2.0)),
            op1.1 + op2.1 + 1,
        )
    }
    op1
}

// expression X AND Y
fn expr_and(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    let mut op1 = expr_not(liste_tokens);

    while liste_tokens.est_mot_cle(TypeMotCle::And) {
        liste_tokens.next().unwrap();
        let op2 = expr_not(liste_tokens);
        op1 = (
            AstExpression::AstOperateurBinaire(
                AstOperateurBinaireType::And,
                Rc::new(op1.0),
                Rc::new(op2.0),
            ),
            op1.1 + op2.1 + 1,
        )
    }
    op1
}

// expression NOT X
fn expr_not(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    if liste_tokens.est_mot_cle(TypeMotCle::Not) {
        let mut op1: (AstExpression, u32) = (AstExpression::AstNombreInteger(0), 0);
        while liste_tokens.est_mot_cle(TypeMotCle::Not) {
            liste_tokens.next().unwrap();
            op1 = expr_comparaison(liste_tokens);
            op1 = (
                AstExpression::AstOperateurUnnaire(AstOperateurUnaireType::Not, Rc::new(op1.0)),
                op1.1 + 1,
            )
        }
        op1
    } else {
        expr_comparaison(liste_tokens)
    }
}

// expression X > Y ou X >= Y ou X < Y ou X <= Y ou X = Y ou X <> Y
fn expr_comparaison(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    let mut op1 = expr_plus(liste_tokens);

    while liste_tokens.est_separateur_pos(EQU, 0)
        || liste_tokens.est_separateur_pos(SUP, 0)
        || liste_tokens.est_separateur_pos(SUPE, 0)
        || liste_tokens.est_separateur_pos(INF, 0)
        || liste_tokens.est_separateur_pos(INFE, 0)
        || liste_tokens.est_separateur_pos(DIFF, 0)
    {
        let token1 = liste_tokens.next().unwrap();
        let op2 = expr_plus(liste_tokens);
        if let Token::TokenSeparateur(separateur, _) = token1 {
            op1 = (
                AstExpression::AstOperateurBinaire(
                    calcul_type_operation_binaire(separateur),
                    Rc::new(op1.0),
                    Rc::new(op2.0),
                ),
                op1.1 + op2.1 + 1,
            )
        } else {
            panic!("Expected separator after expression")
        }
    }
    op1
}

// expression X + Y ou X - Y
fn expr_plus(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    let mut op1 = expr_mod(liste_tokens);

    while liste_tokens.est_separateur_pos(PLUS, 0) || liste_tokens.est_separateur_pos(MOINS, 0) {
        let token1 = liste_tokens.next().unwrap();
        let op2 = expr_mod(liste_tokens);
        if let Token::TokenSeparateur(separateur, _) = token1 {
            op1 = (
                AstExpression::AstOperateurBinaire(
                    calcul_type_operation_binaire(separateur),
                    Rc::new(op1.0),
                    Rc::new(op2.0),
                ),
                op1.1 + op2.1 + 1,
            )
        } else {
            panic!("Expected separator after expression")
        }
    }
    op1
}

// expression X MOD Y
fn expr_mod(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    let mut op1 = expr_dive(liste_tokens);

    while liste_tokens.est_mot_cle(TypeMotCle::Mod) {
        liste_tokens.next().unwrap();
        let op2 = expr_dive(liste_tokens);
        op1 = (
            AstExpression::AstOperateurBinaire(
                AstOperateurBinaireType::Mod,
                Rc::new(op1.0),
                Rc::new(op2.0),
            ),
            op1.1 + op2.1 + 1,
        )
    }
    op1
}

// expression  X \ Y (division  entiere)
fn expr_dive(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    let mut op1 = expr_mult(liste_tokens);

    while liste_tokens.est_separateur_pos(DIVE, 0) {
        liste_tokens.next().unwrap();
        let op2 = expr_mult(liste_tokens);
        op1 = (
            AstExpression::AstOperateurBinaire(DivisionEntiere, Rc::new(op1.0), Rc::new(op2.0)),
            op1.1 + op2.1 + 1,
        )
    }
    op1
}

// expression X * Y ou X / Y
fn expr_mult(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    let mut op1 = expr_neg(liste_tokens);

    while liste_tokens.est_separateur_pos(FOIS, 0) || liste_tokens.est_separateur_pos(DIVR, 0) {
        let token1 = liste_tokens.next().unwrap();
        let op2 = expr_neg(liste_tokens);
        if let Token::TokenSeparateur(separateur, _) = token1 {
            op1 = (
                AstExpression::AstOperateurBinaire(
                    calcul_type_operation_binaire(separateur),
                    Rc::new(op1.0),
                    Rc::new(op2.0),
                ),
                op1.1 + op2.1 + 1,
            )
        } else {
            panic!("Expected separator after expression")
        }
    }
    op1
}

// expression -X
fn expr_neg(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    if liste_tokens.est_separateur_pos(MOINS, 0) || liste_tokens.est_separateur_pos(PLUS, 0) {
        let mut op1: (AstExpression, u32) = (AstExpression::AstNombreInteger(0), 0);
        while liste_tokens.est_separateur_pos(MOINS, 0) || liste_tokens.est_separateur_pos(PLUS, 0)
        {
            let token1 = liste_tokens.next().unwrap();
            op1 = expr_xor(liste_tokens);
            if let Token::TokenSeparateur(separateur, _) = token1 {
                let type_operateur: AstOperateurUnaireType;
                if separateur == MOINS {
                    type_operateur = AstOperateurUnaireType::Moins;
                } else {
                    type_operateur = AstOperateurUnaireType::Plus;
                }
                op1 = (
                    AstExpression::AstOperateurUnnaire(type_operateur, Rc::new(op1.0)),
                    op1.1 + 1,
                )
            } else {
                panic!("Expected separator after expression")
            }
        }
        op1
    } else {
        expr_xor(liste_tokens)
    }
}

// expression X ^ Y
fn expr_xor(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    let mut op1 = expr_literaux(liste_tokens);

    while liste_tokens.est_separateur_pos(EXP, 0) {
        let token1 = liste_tokens.next().unwrap();
        let op2 = expr_literaux(liste_tokens);
        if let Token::TokenSeparateur(separateur, _) = token1 {
            op1 = (
                AstExpression::AstOperateurBinaire(
                    calcul_type_operation_binaire(separateur),
                    Rc::new(op1.0),
                    Rc::new(op2.0),
                ),
                op1.1 + op2.1 + 1,
            )
        } else {
            panic!("Expected separator after expression")
        }
    }
    op1
}

fn expr_literaux(liste_tokens: &mut TokenIterator) -> (AstExpression, u32) {
    if liste_tokens.est_separateur(PARENTO) {
        let mut nb_token: u32 = 0;
        nb_token += 1;
        liste_tokens.next();
        let res = parsing_expression(liste_tokens);
        nb_token += res.1;
        if liste_tokens.est_separateur(PARENTF) {
            nb_token += 1;
            (res.0, nb_token)
        } else {
            panic!("parenthese manquante: {:?}", liste_tokens);
        }
    } else if liste_tokens.est_identifiant() {
        if let Token::TokenIdentifiant(ident, _) = liste_tokens.next().unwrap() {
            return (AstExpression::AstIdentifiant(ident.clone()), 1);
        } else {
            panic!("identifiant inconnu");
        }
    } else if liste_tokens.est_nombre() {
        if let Token::TokenNombre(nombrei16, nombrei32, nombref32, nombref64, type_nombre, _) =
            liste_tokens.next().unwrap()
        {
            match type_nombre {
                Entier => (AstExpression::AstNombreInteger(nombrei16), 1),
                Long => (AstExpression::AstNombreLong(nombrei32), 1),
                Single => (AstExpression::AstNombreSingle(nombref32), 1),
                TypeNombre::Double => (AstExpression::AstNombreDouble(nombref64), 1),
            }
        } else {
            panic!("nombre inconnu: {:?}", liste_tokens.get());
        }
    } else if liste_tokens.est_chaine() {
        if let Token::TokenChaine(chaine, _) = liste_tokens.next().unwrap() {
            return (AstExpression::AstChaine(chaine.clone()), 1);
        } else {
            panic!("chaine inconnue: {:?}", liste_tokens.get());
        }
    } else if liste_tokens.est_mot_cle_fonction() {
        if let Token::TokenMotReserve(ident, _) = liste_tokens.next().unwrap() {
            let mut nb_token: u32 = 1;
            let mut liste_expressions: Vec<AstExpression> = Vec::new();

            if liste_tokens.est_separateur(PARENTO) {
                nb_token += 1;
                liste_tokens.next();
                loop {
                    let res = parsing_expression(liste_tokens);
                    nb_token += res.1;
                    liste_expressions.push(res.0);
                    if liste_tokens.est_separateur(VIRGULE) {
                        nb_token += 1;
                        liste_tokens.next();
                    } else {
                        break;
                    }
                }
                if liste_tokens.est_separateur(PARENTF) {
                    nb_token += 1;
                } else {
                    panic!("parenthèse fermante attendue");
                }
            } else {}
            return (
                AstExpression::AstFonction(AstNomMethode::AstNomReserve(ident), liste_expressions),
                nb_token,
            );
        } else {
            panic!("identifiant inconnu");
        }
    } else {
        panic!("expression inconnue: {:?}", liste_tokens);
    }
}

fn calcul_type_operation_binaire(type_operation: TypeSeparateur) -> AstOperateurBinaireType {
    match type_operation {
        PLUS => AstOperateurBinaireType::Plus,
        MOINS => Moins,
        FOIS => Multiplication,
        DIVR => DivisionReel,
        DIVE => DivisionEntiere,
        SUP => Superieur,
        SUPE => SuperieurEgal,
        INF => Inferieur,
        INFE => InferieurEgal,
        DIFF => Different,
        EQU => Egal,
        EXP => Exponentiel,
        _ => {
            panic!("unsupported type separateur: {:?}", type_operation);
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::basic::struct_basic::AstInstruction::AstAppelMethode;

    #[test]
    fn test_parser1() {
        let fichier = "x=1".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![AstAffectation(
                "x".to_string(),
                AstExpression::AstNombreInteger(1),
            )]
        );
    }

    #[test]
    fn test_parser2() {
        let fichier = "x = 1 EQV 2".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![AstAffectation(
                "x".to_string(),
                AstExpression::AstOperateurBinaire(
                    AstOperateurBinaireType::Eqv,
                    Rc::new(AstExpression::AstNombreInteger(1)),
                    Rc::new(AstExpression::AstNombreInteger(2))
                ),
            )]
        );
    }

    #[test]
    fn test_parser_print1() {
        let fichier = "print 1".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![AstAppelMethode(
                "print".to_string(),
                vec![AstExpression::AstNombreInteger(1)],
                vec![],
            )]
        );
    }

    #[test]
    fn test_parser_print2() {
        let fichier = "print 1 EQV 2".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![AstAppelMethode(
                "print".to_string(),
                vec![AstExpression::AstOperateurBinaire(
                    AstOperateurBinaireType::Eqv,
                    Rc::new(AstExpression::AstNombreInteger(1)),
                    Rc::new(AstExpression::AstNombreInteger(2))
                )],
                vec![],
            )]
        );
    }

    #[test]
    fn test_parser_print3() {
        let fichier = "print 3 > 2 EQV 0".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![AstAppelMethode(
                "print".to_string(),
                vec![AstExpression::AstOperateurBinaire(
                    AstOperateurBinaireType::Eqv,
                    Rc::new(AstExpression::AstOperateurBinaire(
                        Superieur,
                        Rc::new(AstExpression::AstNombreInteger(3)),
                        Rc::new(AstExpression::AstNombreInteger(2))
                    )),
                    Rc::new(AstExpression::AstNombreInteger(0))
                )],
                vec![],
            )]
        );
    }

    #[test]
    fn test_parser_label1() {
        let fichier = "label1: print 1".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![
                AstInstrLabel(AstLabel("label1".to_string())),
                AstAppelMethode(
                    "print".to_string(),
                    vec![AstExpression::AstNombreInteger(1)],
                    vec![],
                )
            ]
        );
    }

    #[test]
    fn test_parser_label2() {
        let fichier = "123 print 1".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![
                AstInstrLabel(AstLabel("123".to_string())),
                AstAppelMethode(
                    "print".to_string(),
                    vec![AstExpression::AstNombreInteger(1)],
                    vec![],
                )
            ]
        );
    }

    #[test]
    fn test_parser_label3() {
        let fichier = "label5:".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![AstInstrLabel(AstLabel("label5".to_string()), )]
        );
    }

    #[test]
    fn test_parser_label4() {
        let fichier = "4".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![AstInstrLabel(AstLabel("4".to_string()), )]
        );
    }

    #[test]
    fn test_parser_input() {
        let fichier = "input abc".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![AstInput(None, vec!["abc".to_string()], true, false)]
        );
    }

    #[test]
    fn test_parser_input2() {
        let fichier = "input \"aaa\"; xxx".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![AstInput(
                Some("aaa".to_string()),
                vec!["xxx".to_string()],
                true,
                true
            )]
        );
    }

    #[test]
    fn test_parser_input3() {
        let fichier = "input ; \"gg\"; t".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![AstInput(
                Some("gg".to_string()),
                vec!["t".to_string()],
                false,
                true
            )]
        );
    }

    #[test]
    fn test_parser_input4() {
        let fichier = "input \"f\", a,b,c".to_string();
        let liste_tokens = lexer(fichier);
        let ast = parsing_programme(liste_tokens);

        assert_eq!(
            ast.liste_instructions,
            vec![AstInput(
                Some("f".to_string()),
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                true,
                false
            )]
        );
    }
}
