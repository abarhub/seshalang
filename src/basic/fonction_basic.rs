use crate::basic::exec_basic;
use crate::basic::exec_basic::Valeur::{
    Chaine, NombreDouble, NombreInteger, NombreLong, NombreSingle,
};
use crate::basic::exec_basic::{TableSymbole, Valeur};
use crate::basic::lexer_basic::TypeMotCle;

use crate::basic::struct_basic::AstNomMethode::AstNomReserve;
use crate::basic::struct_basic::{AstExpression, AstNomMethode};
use chrono::Local;

pub fn execute_fonction(
    nom_methode: AstNomMethode,
    liste_expressions: Vec<AstExpression>,
    table_symbole: &mut TableSymbole,
) -> Valeur {
    match nom_methode {
        AstNomReserve(mot_reserve) => match mot_reserve {
            TypeMotCle::Abs => {
                match verification_un_parametre(
                    "abs()".to_string(),
                    liste_expressions,
                    table_symbole,
                ) {
                    NombreInteger(n) => NombreInteger(n.abs()),
                    NombreLong(n) => NombreLong(n.abs()),
                    NombreSingle(n) => NombreSingle(n.abs()),
                    NombreDouble(valeur) => NombreDouble(valeur.abs()),
                    _ => panic!("La fonction abs() attend un nombre en argument"),
                }
            }
            TypeMotCle::Asc => {
                match verification_un_parametre(
                    "asc()".to_string(),
                    liste_expressions,
                    table_symbole,
                ) {
                    Chaine(s) => NombreInteger(s.chars().next().unwrap() as i16),
                    _ => panic!("La fonction asc() attend une chaine en argument"),
                }
            }
            TypeMotCle::Chr => {
                match verification_un_parametre(
                    "char$()".to_string(),
                    liste_expressions,
                    table_symbole,
                ) {
                    NombreInteger(n) => Chaine(format!("{}", u8::try_from(n).unwrap() as char)),
                    NombreLong(n) => Chaine(format!("{}", u8::try_from(n).unwrap() as char)),
                    _ => panic!("La fonction char$() attend un entier en argument"),
                }
            }
            TypeMotCle::Sin => appliquer_double(
                f64::sin,
                liste_expressions,
                table_symbole,
                "sin()".to_string(),
            ),
            TypeMotCle::Cos => appliquer_double(
                f64::cos,
                liste_expressions,
                table_symbole,
                "cos()".to_string(),
            ),
            TypeMotCle::Tan => appliquer_double(
                f64::tan,
                liste_expressions,
                table_symbole,
                "tan()".to_string(),
            ),
            TypeMotCle::Atn => appliquer_double(
                f64::atan,
                liste_expressions,
                table_symbole,
                "atn()".to_string(),
            ),
            TypeMotCle::Cdbl => {
                match verification_un_parametre(
                    "cdbl()".to_string(),
                    liste_expressions,
                    table_symbole,
                ) {
                    NombreInteger(n) => NombreDouble(n as f64),
                    NombreLong(n) => NombreDouble(n as f64),
                    _ => panic!("La fonction cdbl() attend un entier en argument"),
                }
            }
            TypeMotCle::Csng => match verification_un_parametre(
                "csng()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                NombreInteger(n) => NombreSingle(n as f32),
                NombreLong(n) => NombreSingle(n as f32),
                _ => panic!("La fonction csng() attend un entier en argument"),
            },
            TypeMotCle::Clng => match verification_un_parametre(
                "clng()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                NombreInteger(n) => NombreLong(n as i32),
                NombreLong(n) => NombreLong(n),
                NombreSingle(n) => NombreLong(n as i32),
                NombreDouble(n) => NombreLong(n as i32),
                _ => panic!("La fonction clng() attend un entier en argument"),
            },
            TypeMotCle::Cint => match verification_un_parametre(
                "cint()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                NombreInteger(n) => NombreInteger(n),
                NombreLong(n) => NombreInteger(n as i16),
                NombreSingle(n) => NombreInteger(n as i16),
                NombreDouble(n) => NombreInteger(n as i16),
                _ => panic!("La fonction cint() attend un entier en argument"),
            },
            TypeMotCle::Mki => match verification_un_parametre(
                "mki$()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                NombreInteger(n) => Chaine(n.to_string()),
                _ => panic!("La fonction mki$() attend un integer en argument"),
            },
            TypeMotCle::Mkl => match verification_un_parametre(
                "mkl$()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                NombreLong(n) => Chaine(n.to_string()),
                _ => panic!("La fonction mkl$() attend un long en argument"),
            },
            TypeMotCle::Mks => match verification_un_parametre(
                "mks$()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                NombreSingle(n) => Chaine(n.to_string()),
                _ => panic!("La fonction mks$() attend un single en argument"),
            },
            TypeMotCle::Mkd => match verification_un_parametre(
                "mkd$()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                NombreDouble(n) => Chaine(n.to_string()),
                _ => panic!("La fonction mki$() attend un double en argument"),
            },
            TypeMotCle::Cvi => match verification_un_parametre(
                "Cvi()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Chaine(s) => NombreInteger(s.parse::<i16>().unwrap()),
                _ => panic!("La fonction Cvi() attend une chaine en argument"),
            },
            TypeMotCle::Cvl => match verification_un_parametre(
                "Cvl()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Chaine(s) => NombreLong(s.parse::<i32>().unwrap()),
                _ => panic!("La fonction Cvl() attend une chaine en argument"),
            },
            TypeMotCle::Cvs => match verification_un_parametre(
                "Cvs()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Chaine(s) => NombreSingle(s.parse::<f32>().unwrap()),
                _ => panic!("La fonction Cvs() attend une chaine en argument"),
            },
            TypeMotCle::Cvd => match verification_un_parametre(
                "Cvd()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Chaine(s) => NombreDouble(s.parse::<f64>().unwrap()),
                _ => panic!("La fonction Cvd() attend une chaine en argument"),
            },
            TypeMotCle::Fix => match verification_un_parametre(
                "fix()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                NombreInteger(n) => NombreInteger(n),
                NombreLong(n) => NombreInteger(n as i16),
                NombreSingle(n) => NombreInteger(n.floor() as i16),
                NombreDouble(n) => NombreInteger(n.floor() as i16),
                _ => panic!("La fonction fix() attend un entier en argument"),
            },
            TypeMotCle::Int => match verification_un_parametre(
                "int()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                NombreInteger(n) => NombreInteger(n),
                NombreLong(n) => NombreInteger(n as i16),
                NombreSingle(n) => NombreInteger(n.round() as i16),
                NombreDouble(n) => NombreInteger(n.round() as i16),
                _ => panic!("La fonction int() attend un entier en argument"),
            },
            TypeMotCle::Date => {
                if liste_expressions.len() != 0 {
                    panic!("La fonction date$ n'attend aucun argument");
                }
                Chaine(Local::now().format("%m-%d-%Y").to_string())
            }
            TypeMotCle::Time => {
                if liste_expressions.len() != 0 {
                    panic!("La fonction time$ n'attend aucun argument");
                }
                Chaine(Local::now().format("%H:%M:%S").to_string())
            }
            TypeMotCle::Instr => calcul_instr(liste_expressions, table_symbole),
            TypeMotCle::Hex => {
                match verification_un_parametre(
                    "hex$()".to_string(),
                    liste_expressions,
                    table_symbole,
                ) {
                    NombreInteger(n) => Chaine(format!("{:x}", n)),
                    NombreLong(n) => Chaine(format!("{:x}", n)),
                    NombreSingle(n) => Chaine(format!("{:x}", n.round() as i32)),
                    NombreDouble(n) => Chaine(format!("{:x}", n.round() as i32)),
                    _ => panic!("La fonction hex() attend exactement un argument"),
                }
            }
            TypeMotCle::Oct => {
                match verification_un_parametre(
                    "oct$()".to_string(),
                    liste_expressions,
                    table_symbole,
                ) {
                    NombreInteger(n) => Chaine(format!("{:o}", n)),
                    NombreLong(n) => Chaine(format!("{:o}", n)),
                    NombreSingle(n) => Chaine(format!("{:o}", n.round() as i32)),
                    NombreDouble(n) => Chaine(format!("{:o}", n.round() as i32)),
                    _ => panic!("La fonction oct() attend exactement un argument"),
                }
            }
            _ => panic!("La fonction {:?} n'existe pas", mot_reserve),
        },
        AstNomMethode::AstNomFonction(nom) => {
            if liste_expressions.len() == 1 {
                let e1 = liste_expressions.get(0).unwrap();
                let valeur = exec_basic::execute_expression(&(e1.clone()), table_symbole);

                if let Some(val) = table_symbole.get_variable_tableau(nom.clone(), valeur.clone()) {
                    val
                } else {
                    panic!("La variable tableau {} n'existe pas", nom);
                }
            } else {
                panic!("La fonction {} attend exactement un argument", nom);
            }
        }
    }
}

fn verification_un_parametre(
    nom: String,
    liste_expressions: Vec<AstExpression>,
    table_symbole: &mut TableSymbole,
) -> Valeur {
    if liste_expressions.len() != 1 {
        panic!("La fonction {} attend exactement un argument", nom);
    }
    let e1 = liste_expressions.get(0).unwrap();
    exec_basic::execute_expression(&(e1.clone()), table_symbole)
}

fn appliquer_double<F: Fn(f64) -> f64>(
    f: F,
    liste_expressions: Vec<AstExpression>,
    table_symbole: &mut TableSymbole,
    nom: String,
) -> Valeur {
    match verification_un_parametre(nom.clone(), liste_expressions, table_symbole) {
        NombreInteger(n) => NombreDouble(f(n as f64)),
        NombreLong(n) => NombreDouble(f(n as f64)),
        NombreSingle(n) => NombreDouble(f(n as f64)),
        NombreDouble(valeur) => NombreDouble(f(valeur)),
        _ => panic!("La fonction {} attend un nombre en argument", nom),
    }
}

fn calcul_instr(liste_expressions: Vec<AstExpression>, table_symbole: &mut TableSymbole) -> Valeur {
    if liste_expressions.len() != 2 && liste_expressions.len() != 3 {
        panic!(
            "La fonction instr attend 2 ou 3 arguments (nb arguments: {})",
            liste_expressions.len()
        );
    }
    let pos_debut: usize;
    let chaine_a_chercher: String;
    let chaine_recherche: String;
    let resultat: i16;
    let param_debut: usize;
    if liste_expressions.len() == 2 {
        pos_debut = 1;
        param_debut = 0;
    } else {
        let e1 = liste_expressions.get(0).unwrap();
        let expr1 = exec_basic::execute_expression(&(e1.clone()), table_symbole);
        match expr1 {
            NombreInteger(n) => pos_debut = n as usize,
            _ => panic!("La fonction instr attend un nombre en argument"),
        }
        param_debut = 1;
    }
    let e1 = liste_expressions.get(param_debut).unwrap();
    let expr1 = exec_basic::execute_expression(&(e1.clone()), table_symbole);
    match expr1 {
        Chaine(s) => chaine_a_chercher = s,
        _ => panic!(
            "Le parametre {} doit être une chaine de caractères",
            param_debut + 1
        ),
    }
    let e2 = liste_expressions.get(param_debut + 1).unwrap();
    let expr2 = exec_basic::execute_expression(&(e2.clone()), table_symbole);
    match expr2 {
        Chaine(s) => chaine_recherche = s,
        _ => panic!(
            "Le parametre {} doit être une chaine de caractères",
            param_debut + 2
        ),
    }
    if pos_debut >= 1 && pos_debut < chaine_a_chercher.len() {
        if let Some(pos) = chaine_a_chercher[pos_debut - 1..].find(&chaine_recherche) {
            resultat = pos as i16 + 1 + (pos_debut as i16 - 1);
        } else {
            resultat = 0;
        }
    } else {
        resultat = 0;
    }
    NombreInteger(resultat)
}
