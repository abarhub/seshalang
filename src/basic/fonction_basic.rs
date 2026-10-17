use crate::basic::exec_basic;
use crate::basic::exec_basic::Valeur;
use crate::basic::lexer_basic::TypeMotCle;
use crate::basic::main_basic::AstNomMethode::AstNomReserve;
use crate::basic::main_basic::{AstExpression, AstNomMethode};
use std::collections::HashMap;

pub fn execute_fonction(
    nom_methode: AstNomMethode,
    liste_expressions: Vec<AstExpression>,
    table_symbole: &mut HashMap<String, Valeur>,
) -> Valeur {
    match nom_methode {
        AstNomReserve(mot_reserve) => match mot_reserve {
            TypeMotCle::Abs => {
                match verification_un_parametre(
                    "abs()".to_string(),
                    liste_expressions,
                    table_symbole,
                ) {
                    Valeur::NombreInteger(n) => Valeur::NombreInteger(n.abs()),
                    Valeur::NombreLong(n) => Valeur::NombreLong(n.abs()),
                    Valeur::NombreSingle(n) => Valeur::NombreSingle(n.abs()),
                    Valeur::NombreDouble(valeur) => Valeur::NombreDouble(valeur.abs()),
                    _ => panic!("La fonction abs() attend un nombre en argument"),
                }
            }
            TypeMotCle::Asc => {
                match verification_un_parametre(
                    "asc()".to_string(),
                    liste_expressions,
                    table_symbole,
                ) {
                    Valeur::Chaine(s) => Valeur::NombreInteger(s.chars().next().unwrap() as i16),
                    _ => panic!("La fonction asc() attend une chaine en argument"),
                }
            }
            TypeMotCle::Chr => {
                match verification_un_parametre(
                    "char$()".to_string(),
                    liste_expressions,
                    table_symbole,
                ) {
                    Valeur::NombreInteger(n) => {
                        Valeur::Chaine(format!("{}", u8::try_from(n).unwrap() as char))
                    }
                    Valeur::NombreLong(n) => {
                        Valeur::Chaine(format!("{}", u8::try_from(n).unwrap() as char))
                    }
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
                    Valeur::NombreInteger(n) => Valeur::NombreDouble(n as f64),
                    Valeur::NombreLong(n) => Valeur::NombreDouble(n as f64),
                    _ => panic!("La fonction cdbl() attend un entier en argument"),
                }
            }
            TypeMotCle::Csng => match verification_un_parametre(
                "csng()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Valeur::NombreInteger(n) => Valeur::NombreSingle(n as f32),
                Valeur::NombreLong(n) => Valeur::NombreSingle(n as f32),
                _ => panic!("La fonction csng() attend un entier en argument"),
            },
            TypeMotCle::Clng => match verification_un_parametre(
                "clng()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Valeur::NombreInteger(n) => Valeur::NombreLong(n as i32),
                Valeur::NombreLong(n) => Valeur::NombreLong(n),
                Valeur::NombreSingle(n) => Valeur::NombreLong(n as i32),
                Valeur::NombreDouble(n) => Valeur::NombreLong(n as i32),
                _ => panic!("La fonction clng() attend un entier en argument"),
            },
            TypeMotCle::Cint => match verification_un_parametre(
                "cint()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Valeur::NombreInteger(n) => Valeur::NombreInteger(n),
                Valeur::NombreLong(n) => Valeur::NombreInteger(n as i16),
                Valeur::NombreSingle(n) => Valeur::NombreInteger(n as i16),
                Valeur::NombreDouble(n) => Valeur::NombreInteger(n as i16),
                _ => panic!("La fonction cint() attend un entier en argument"),
            },
            TypeMotCle::Mki => match verification_un_parametre(
                "mki$()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Valeur::NombreInteger(n) => Valeur::Chaine(n.to_string()),
                _ => panic!("La fonction mki$() attend un integer en argument"),
            },
            TypeMotCle::Mkl => match verification_un_parametre(
                "mkl$()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Valeur::NombreLong(n) => Valeur::Chaine(n.to_string()),
                _ => panic!("La fonction mkl$() attend un long en argument"),
            },
            TypeMotCle::Mks => match verification_un_parametre(
                "mks$()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Valeur::NombreSingle(n) => Valeur::Chaine(n.to_string()),
                _ => panic!("La fonction mks$() attend un single en argument"),
            },
            TypeMotCle::Mkd => match verification_un_parametre(
                "mkd$()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Valeur::NombreDouble(n) => Valeur::Chaine(n.to_string()),
                _ => panic!("La fonction mki$() attend un double en argument"),
            },
            TypeMotCle::Cvi => match verification_un_parametre(
                "Cvi()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Valeur::Chaine(s) => Valeur::NombreInteger(s.parse::<i16>().unwrap()),
                _ => panic!("La fonction Cvi() attend une chaine en argument"),
            },
            TypeMotCle::Cvl => match verification_un_parametre(
                "Cvl()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Valeur::Chaine(s) => Valeur::NombreLong(s.parse::<i32>().unwrap()),
                _ => panic!("La fonction Cvl() attend une chaine en argument"),
            },
            TypeMotCle::Cvs => match verification_un_parametre(
                "Cvs()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Valeur::Chaine(s) => Valeur::NombreSingle(s.parse::<f32>().unwrap()),
                _ => panic!("La fonction Cvs() attend une chaine en argument"),
            },
            TypeMotCle::Cvd => match verification_un_parametre(
                "Cvd()".to_string(),
                liste_expressions,
                table_symbole,
            ) {
                Valeur::Chaine(s) => Valeur::NombreDouble(s.parse::<f64>().unwrap()),
                _ => panic!("La fonction Cvd() attend une chaine en argument"),
            },
            _ => panic!("La fonction {:?} n'existe pas", mot_reserve),
        },
        _ => panic!("Méthode inconnue: {:?}", nom_methode),
    }
}

fn verification_un_parametre(
    nom: String,
    liste_expressions: Vec<AstExpression>,
    table_symbole: &mut HashMap<String, Valeur>,
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
    table_symbole: &mut HashMap<String, Valeur>,
    nom: String,
) -> Valeur {
    match verification_un_parametre(nom.clone(), liste_expressions, table_symbole) {
        Valeur::NombreInteger(n) => Valeur::NombreDouble(f(n as f64)),
        Valeur::NombreLong(n) => Valeur::NombreDouble(f(n as f64)),
        Valeur::NombreSingle(n) => Valeur::NombreDouble(f(n as f64)),
        Valeur::NombreDouble(valeur) => Valeur::NombreDouble(f(valeur)),
        _ => panic!("La fonction {} attend un nombre en argument", nom),
    }
}
