use crate::basic::exec_basic::Valeur::{
    Chaine, NombreDouble, NombreInteger, NombreLong, NombreSingle,
};
use crate::basic::fonction_basic;

use crate::basic::struct_basic::AstExpression::{AstFonction, AstOperateurUnnaire};
use crate::basic::struct_basic::AstInstruction::{
    AstAffectation, AstAffectationTableau, AstAppelMethode, AstDecl, AstDeclVariable, AstEnd,
    AstGoto, AstIf, AstInput, AstInstrLabel,
};
use crate::basic::struct_basic::AstLabelEnum::AstLabel;
use crate::basic::struct_basic::{
    AstDeclarationVariable, AstExpression, AstInstruction, AstLabelEnum, AstOperateurBinaireType,
    AstOperateurUnaireType, AstProgramme, TypeVariable,
};
use crate::basic::system_basic::{system_clear, system_print, system_println};
use crate::basic::utils_basic::Sortie;
use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;
use std::{fmt, io};

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

impl Valeur {
    fn calcul_type(&self) -> TypeVariable {
        match self {
            NombreInteger(_) => TypeVariable::Integer,
            NombreLong(_) => TypeVariable::Long,
            NombreSingle(_) => TypeVariable::Single,
            NombreDouble(_) => TypeVariable::Double,
            Chaine(_) => TypeVariable::String,
        }
    }

    fn get_integer(&self) -> i16 {
        match self {
            NombreInteger(n) => *n,
            NombreLong(n) => i16::try_from(*n).unwrap(),
            NombreSingle(n) => *n as i16,
            NombreDouble(n) => *n as i16,
            _ => panic!("Impossible de convertir la valeur en entier"),
        }
    }
}

#[derive(Debug, Clone)]
struct Variable {
    valeur: Valeur,
    type_variable: TypeVariable,
}

#[derive(Debug)]
pub struct Tableau {
    type_variable: TypeVariable,
    taille_tableau: Vec<(u16, u16)>,
    contenu: Vec<Valeur>,
}

#[derive(Debug)]
pub struct TableSymbole {
    declaration_defaut: HashMap<char, TypeVariable>,
    declaration_variable: HashMap<String, TypeVariable>,
    variables: HashMap<String, Variable>,
    tableau: HashMap<String, Tableau>,
}

impl TableSymbole {
    pub(crate) fn new() -> TableSymbole {
        let mut defaut = HashMap::new();
        for c in 'a'..'z' {
            defaut.insert(c, TypeVariable::Single);
        }
        TableSymbole {
            declaration_defaut: defaut,
            declaration_variable: HashMap::new(),
            variables: HashMap::new(),
            tableau: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, nom: String, valeur: Valeur) {
        if let Some(variable) = self.variables.get(&nom) {
            let valeur = self.convertie(&variable.type_variable, &valeur);
            self.variables.insert(
                nom,
                Variable {
                    valeur,
                    type_variable: variable.type_variable.clone(),
                },
            );
        } else {
            let type_variable = self.calcul_type(nom.clone());
            let valeur = self.convertie(&type_variable, &valeur);
            self.variables.insert(
                nom,
                Variable {
                    valeur,
                    type_variable,
                },
            );
        }
    }

    pub fn get_variable(&self, nom: &str) -> Option<&Valeur> {
        if let Some(variable) = self.variables.get(nom) {
            Some(&variable.valeur)
        } else {
            None
        }
    }

    fn convertie(&self, type_variable: &TypeVariable, valeur: &Valeur) -> Valeur {
        match (type_variable, valeur.clone()) {
            (TypeVariable::Integer, NombreInteger(_)) => valeur.clone(),
            (TypeVariable::Integer, NombreLong(n)) => NombreInteger(i16::try_from(n).unwrap()),
            (TypeVariable::Integer, NombreSingle(n)) => NombreInteger(n.round() as i16),
            (TypeVariable::Integer, NombreDouble(n)) => NombreInteger(n.round() as i16),
            (TypeVariable::Long, NombreLong(_)) => valeur.clone(),
            (TypeVariable::Long, NombreInteger(n)) => NombreLong(i32::try_from(n).unwrap()),
            (TypeVariable::Long, NombreSingle(n)) => NombreLong(n.round() as i32),
            (TypeVariable::Long, NombreDouble(n)) => NombreLong(n.round() as i32),
            (TypeVariable::Single, NombreSingle(_)) => valeur.clone(),
            (TypeVariable::Single, NombreInteger(n)) => NombreSingle(n as f32),
            (TypeVariable::Single, NombreLong(n)) => NombreSingle(n as f32),
            (TypeVariable::Single, NombreDouble(n)) => NombreSingle(n as f32),
            (TypeVariable::Double, NombreDouble(_)) => valeur.clone(),
            (TypeVariable::Double, NombreInteger(n)) => NombreDouble(n as f64),
            (TypeVariable::Double, NombreLong(n)) => NombreDouble(n as f64),
            (TypeVariable::Double, NombreSingle(n)) => NombreDouble(n as f64),
            (TypeVariable::String, Chaine(_)) => valeur.clone(),
            _ => panic!(
                "Conversion impossible de {:?} vers {:?}",
                valeur, type_variable,
            ),
        }
    }
    pub fn calcul_type(&self, nom_variable: String) -> TypeVariable {
        let nom_variable_minuscule = nom_variable.to_lowercase();
        if nom_variable_minuscule.ends_with("%") {
            TypeVariable::Integer
        } else if nom_variable.ends_with("$") {
            TypeVariable::String
        } else if nom_variable.ends_with("&") {
            TypeVariable::Long
        } else if nom_variable.ends_with("!") {
            TypeVariable::Single
        } else if nom_variable.ends_with("#") {
            TypeVariable::Double
        } else if self
            .declaration_variable
            .contains_key(&nom_variable_minuscule)
        {
            self.declaration_variable[&nom_variable_minuscule].clone()
        } else {
            if nom_variable_minuscule.starts_with("_") {
                TypeVariable::Integer
            } else {
                let c = nom_variable_minuscule.chars().next().unwrap();
                self.declaration_defaut[&c].clone()
            }
        }
    }

    fn set_declaration_defaut(&mut self, nom_variable: String, type_variable: TypeVariable) {
        let c = nom_variable.to_lowercase().chars().next().unwrap();
        self.declaration_defaut.insert(c, type_variable);
    }

    fn set_declaration_variable(
        &mut self,
        nom_variable: String,
        type_variable_opt: Option<TypeVariable>,
    ) {
        let nom_variable_minuscule = nom_variable.to_lowercase();
        let type_variable =
            if let Some(type_var) = self.get_type(nom_variable.clone(), type_variable_opt) {
                type_var
            } else {
                panic!("Type variable not found for variable: {}", nom_variable);
            };
        self.declaration_variable
            .insert(nom_variable_minuscule, type_variable);
    }

    fn get_type(
        &mut self,
        nom_variable: String,
        type_variable_opt: Option<TypeVariable>,
    ) -> Option<TypeVariable> {
        let nom_variable_minuscule = nom_variable.to_lowercase();
        if nom_variable_minuscule.ends_with("%") {
            Some(TypeVariable::Integer)
        } else if nom_variable.ends_with("$") {
            Some(TypeVariable::String)
        } else if nom_variable.ends_with("&") {
            Some(TypeVariable::Long)
        } else if nom_variable.ends_with("!") {
            Some(TypeVariable::Single)
        } else if nom_variable.ends_with("#") {
            Some(TypeVariable::Double)
        } else if type_variable_opt.is_some() {
            type_variable_opt
        } else {
            let c = nom_variable_minuscule.chars().next().unwrap();
            Some(self.declaration_defaut[&c].clone())
        }
    }

    fn set_declaration_tableau(
        &mut self,
        nom_variable: String,
        type_variable_opt: Option<TypeVariable>,
        taille_tableau: Vec<i16>,
    ) {
        let nom_variable_minuscule = nom_variable.to_lowercase();
        let type_variable =
            if let Some(type_var) = self.get_type(nom_variable.clone(), type_variable_opt) {
                type_var
            } else {
                panic!("Type variable not found for variable: {}", nom_variable);
            };
        let taille: Vec<(u16, u16)> = taille_tableau
            .iter()
            .copied()
            .map(|x| (0u16, x as u16))
            .collect();
        let taille_totale: u32 = taille_tableau.iter().map(|x| *x as u32).product();
        let contenu = match type_variable {
            TypeVariable::Integer => vec![NombreInteger(0i16); taille_totale as usize],
            TypeVariable::Long => vec![NombreLong(0i32); taille_totale as usize],
            TypeVariable::Single => vec![NombreSingle(0.0f32); taille_totale as usize],
            TypeVariable::Double => vec![NombreDouble(0.0f64); taille_totale as usize],
            TypeVariable::String => vec![Chaine("".to_string()); taille_totale as usize],
        };
        let tableau = Tableau {
            type_variable,
            taille_tableau: taille,
            contenu,
        };
        self.tableau.insert(nom_variable_minuscule, tableau);
    }

    fn set_variable_tableau(&mut self, nom: String, pos: Valeur, valeur: Valeur) {
        let pos_int = pos.get_integer();
        let nom_minuscule = nom.to_lowercase();

        if !self.tableau.contains_key(&nom_minuscule) {
            // le tableau n'existe pas, on le crée
            let taille = 10;
            self.tableau.insert(
                nom_minuscule.clone(),
                Tableau {
                    type_variable: TypeVariable::Single,
                    taille_tableau: vec![(0, 1)],
                    contenu: vec![NombreSingle(0.0f32); taille],
                },
            );
        }

        if let Some(tableau) = self.tableau.get_mut(&nom_minuscule) {
            tableau.contenu[pos_int as usize] = valeur;
        } else {
            panic!("Tableau {} n'existe pas", nom);
        }
    }

    pub fn get_variable_tableau(&self, nom: String, pos: Valeur) -> Option<Valeur> {
        let nom_minuscule = nom.to_lowercase();
        if let Some(tableau) = self.tableau.get(&nom_minuscule) {
            Some(tableau.contenu[pos.get_integer() as usize].clone())
        } else {
            panic!("Tableau {} n'existe pas", nom);
        }
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
        } else if let AstEnd() = instruction {
            break;
        } else if let AstDecl(type_variable, nom_variables) = instruction {
            declaration_defaut(&mut contexte, nom_variables, type_variable);
        } else if let AstDeclVariable(liste_variables) = instruction {
            declaration_variable(&mut contexte, liste_variables);
        } else if let AstAffectationTableau(nom_variable, liste_dimensions, valeur) = instruction {
            affectation_tableau(&mut contexte, nom_variable, liste_dimensions, valeur);
        } else if let AstInstruction::AstCls() = instruction {
            cls();
        } else if let AstInstruction::AstMid(nom_variable, exp, exp_opt, valeur) = instruction {
            affectation_contenu_variable_chaine(&mut contexte, nom_variable, exp, exp_opt, valeur);
        } else {
            panic!("instruction inconnue: {:?}", instruction);
        }
        if !pos_change {
            pos_instruction += 1;
        }
    }
}

fn affectation_contenu_variable_chaine(
    table_symbole: &mut TableSymbole,
    nom_variable: &String,
    exp: &AstExpression,
    exp_opt: &Option<AstExpression>,
    valeur: &AstExpression,
) {
    let resultat = execute_expression(&exp, table_symbole);
    let resultat_opt = match exp_opt {
        Some(exp) => Some(execute_expression(&exp, table_symbole)),
        None => None,
    };
    let valeur2 = execute_expression(valeur, table_symbole);
    let indice;
    match resultat {
        NombreInteger(n) => indice = n as usize - 1,
        _ => panic!("affectation_contenu_variable_chaine: resultat n'est pas un entier"),
    }
    let indice2;
    match resultat_opt {
        Some(NombreInteger(n)) => indice2 = Some(n),
        None => indice2 = None,
        _ => panic!("affectation_contenu_variable_chaine: resultat n'est pas un entier"),
    }
    let valeur3;
    match valeur2 {
        Chaine(s) => valeur3 = s,
        _ => panic!(
            "affectation_contenu_variable_chaine: valeur n'est pas une chaine: {}",
            valeur2
        ),
    }

    match table_symbole.get_variable(nom_variable.clone().as_str()) {
        Some(variable) => match variable {
            Chaine(chaine) => match indice2 {
                Some(n) => {
                    let s = chaine[..indice].to_string()
                        + valeur3.as_str()
                        + &chaine[indice + n as usize..];
                    table_symbole.set_variable(nom_variable.clone(), Chaine(s));
                }
                None => {
                    let s = chaine[..indice].to_string() + valeur3.as_str();
                    table_symbole.set_variable(nom_variable.clone(), Chaine(s));
                }
            },
            _ => panic!(
                "affectation_contenu_variable_chaine: variable {} n'est pas une chaine",
                nom_variable
            ),
        },
        _ => panic!(
            "affectation_contenu_variable_chaine: variable {} n'existe pas",
            nom_variable
        ),
    }
}

fn cls() {
    system_clear().expect("Erreur pour le clear");
}

fn affectation_tableau(
    table_symbole: &mut TableSymbole,
    nom_variable: &String,
    liste_parametres: &Vec<AstExpression>,
    valeur: &AstExpression,
) {
    if liste_parametres.len() != 1 {
        panic!("affectation_tableau: liste_parametres.len()!=1");
    }
    let resultat = execute_expression(&liste_parametres[0], table_symbole);
    let parametre = resultat.clone();
    let resultat2 = execute_expression(valeur, table_symbole);
    table_symbole.set_variable_tableau(nom_variable.clone(), parametre.clone(), resultat2.clone());
    if DEBUG_PRINT {
        println!(
            "affect tableau {:?} ({:?}) := {:?}",
            nom_variable.clone(),
            parametre,
            resultat2
        );
    }
}

fn declaration_variable(
    table_symbole: &mut TableSymbole,
    liste_variables: &Vec<AstDeclarationVariable>,
) {
    for variable in liste_variables.iter() {
        if variable.liste_dimensions.is_empty() {
            table_symbole.set_declaration_variable(
                variable.nom_variable.clone(),
                variable.type_variable.clone(),
            );
        } else {
            let mut liste_expr = vec![];
            for expr in variable.liste_dimensions.iter() {
                let expr_evalue = execute_expression(expr, table_symbole);
                liste_expr.push(expr_evalue.get_integer());
            }
            table_symbole.set_declaration_tableau(
                variable.nom_variable.clone(),
                variable.type_variable.clone(),
                liste_expr,
            );
        }
    }
}

fn declaration_defaut(
    contexte: &mut TableSymbole,
    nom_variables: &Vec<Vec<String>>,
    type_variable: &TypeVariable,
) {
    for nom_variable in nom_variables.iter() {
        if nom_variable.len() == 1 {
            let c = nom_variable[0].to_lowercase().chars().next().unwrap();
            contexte.set_declaration_defaut(c.to_string(), type_variable.clone());
        } else if nom_variable.len() == 2 {
            let debut = nom_variable[0].to_lowercase().chars().next().unwrap();
            let fin = nom_variable[1].to_lowercase().chars().next().unwrap();
            for c in debut..fin {
                contexte.set_declaration_defaut(c.to_string(), type_variable.clone());
            }
        } else {
            panic!("erreur dans le nombre de variables");
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
            system_print(format!("{} ? ", message)).expect("Erreur pour le print");
        } else {
            system_print(format!("{}", message)).expect("Erreur pour le print");
        }
    } else if *question {
        system_print("? ".to_string()).expect("Erreur pour le print");
    }
    io::stdout().flush().unwrap();
    if !liste_variables.is_empty() {
        for nom_var in liste_variables {
            loop {
                let mut valeur = String::new();
                io::stdin().read_line(&mut valeur).unwrap();
                valeur.pop();
                valeur = valeur.trim().to_string();
                let type_variable = contexte.calcul_type(nom_var.clone());
                if type_variable == TypeVariable::Integer {
                    if valeur.is_empty() {
                        let valeur = NombreInteger(0);
                        contexte.set_variable(nom_var.clone(), valeur);
                        break;
                    }
                    let nombre_opt = valeur.parse::<i16>();
                    if nombre_opt.is_ok() {
                        let valeur = NombreInteger(nombre_opt.unwrap());
                        contexte.set_variable(nom_var.clone(), valeur.clone());
                        break;
                    } else {
                        system_println("Redo from start".to_string())
                            .expect("Erreur pour le print");
                    }
                } else if type_variable == TypeVariable::String {
                    if valeur.is_empty() {
                        let valeur = Chaine("".to_string());
                        contexte.set_variable(nom_var.clone(), valeur);
                        break;
                    }
                    let valeur = Chaine(valeur.to_string());
                    contexte.set_variable(nom_var.clone(), valeur.clone());
                    break;
                } else if type_variable == TypeVariable::Long {
                    if valeur.is_empty() {
                        let valeur = NombreLong(0);
                        contexte.set_variable(nom_var.clone(), valeur);
                        break;
                    }
                    let nombre_opt = valeur.parse::<i32>();
                    if nombre_opt.is_ok() {
                        let valeur = NombreLong(nombre_opt.unwrap());
                        contexte.set_variable(nom_var.clone(), valeur.clone());
                        break;
                    } else {
                        system_print("Redo from start".to_string()).expect("Erreur pour le print");
                    }
                } else if type_variable == TypeVariable::Single {
                    if valeur.is_empty() {
                        let valeur = NombreSingle(0.0f32);
                        contexte.set_variable(nom_var.clone(), valeur);
                        break;
                    }
                    let nombre_opt = valeur.parse::<f32>();
                    if nombre_opt.is_ok() {
                        let valeur = NombreSingle(nombre_opt.unwrap());
                        contexte.set_variable(nom_var.clone(), valeur.clone());
                        break;
                    } else {
                        system_print("Redo from start".to_string()).expect("Erreur pour le print");
                    }
                } else if type_variable == TypeVariable::Double {
                    if valeur.is_empty() {
                        let valeur = NombreDouble(0.0f64);
                        contexte.set_variable(nom_var.clone(), valeur);
                        break;
                    }
                    let nombre_opt = valeur.parse::<f64>();
                    if nombre_opt.is_ok() {
                        let valeur = NombreDouble(nombre_opt.unwrap());
                        contexte.set_variable(nom_var.clone(), valeur.clone());
                        break;
                    } else {
                        system_print("Redo from start".to_string()).expect("Erreur pour le print");
                    }
                } else {
                    panic!("Type variable non reconnu {:?}", type_variable);
                }
            }
        }
    }
    if *retour_ligne {
        system_println("".to_string()).expect("Erreur pour le println");
    }
}

fn trouve_label(label: &AstLabelEnum, liste_instructions: &Vec<AstInstruction>) -> usize {
    let AstLabel(nom_label) = label;
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
            let type_var = table_symbole.calcul_type(ident.clone());
            if type_var == TypeVariable::Integer {
                table_symbole.set_variable(ident.clone(), NombreInteger(0));
                return NombreInteger(0);
            } else if type_var == TypeVariable::Long {
                table_symbole.set_variable(ident.clone(), NombreLong(0));
                return NombreLong(0);
            } else if type_var == TypeVariable::Single {
                table_symbole.set_variable(ident.clone(), NombreSingle(0.0));
                return NombreSingle(0.0);
            } else if type_var == TypeVariable::Double {
                table_symbole.set_variable(ident.clone(), NombreDouble(0.0));
                return NombreDouble(0.0);
            } else if type_var == TypeVariable::String {
                table_symbole.set_variable(ident.clone(), Chaine(String::new()));
                return Chaine(String::new());
            } else {
                panic!("Identifiant {} non déclaré", ident);
            }
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
                AstOperateurBinaireType::Mod => NombreInteger(
                    (n1.round() as i16)
                        .checked_rem(n2.round() as i16)
                        .unwrap_or(0),
                ),
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
            AstOperateurBinaireType::Egal => NombreInteger(if s1 == s2 { TRUE } else { FALSE }),
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
            },
            NombreLong(n) => match operateur {
                AstOperateurUnaireType::Plus => expr1,
                AstOperateurUnaireType::Moins => NombreLong(-n),
                AstOperateurUnaireType::Not => NombreLong(!n),
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
