use super::*;   // accès à tout le module parent, y compris le privé
use crate::basic::struct_basic::AstInstruction::AstAppelMethode;

#[test]
fn test_parser1() {
    let fichier = "x=1".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![AstAffectation("x".to_string(), AstNombreInteger(1),)]
    );
}

#[test]
fn test_parser2() {
    let fichier = "x = 1 EQV 2".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![AstAffectation(
            "x".to_string(),
            AstOperateurBinaire(
                AstOperateurBinaireType::Eqv,
                Rc::new(AstNombreInteger(1)),
                Rc::new(AstNombreInteger(2))
            ),
        )]
    );
}

#[test]
fn test_parser_print1() {
    let fichier = "print 1".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![AstAppelMethode(
            "print".to_string(),
            vec![AstNombreInteger(1)],
            vec![],
        )]
    );
}

#[test]
fn test_parser_print2() {
    let fichier = "print 1 EQV 2".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![AstAppelMethode(
            "print".to_string(),
            vec![AstOperateurBinaire(
                AstOperateurBinaireType::Eqv,
                Rc::new(AstNombreInteger(1)),
                Rc::new(AstNombreInteger(2))
            )],
            vec![],
        )]
    );
}

#[test]
fn test_parser_print3() {
    let fichier = "print 3 > 2 EQV 0".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![AstAppelMethode(
            "print".to_string(),
            vec![AstOperateurBinaire(
                AstOperateurBinaireType::Eqv,
                Rc::new(AstOperateurBinaire(
                    Superieur,
                    Rc::new(AstNombreInteger(3)),
                    Rc::new(AstNombreInteger(2))
                )),
                Rc::new(AstNombreInteger(0))
            )],
            vec![],
        )]
    );
}

#[test]
fn test_parser_print4() {
    let fichier = "print 1;2".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![AstAppelMethode(
            "print".to_string(),
            vec![AstNombreInteger(1), AstNombreInteger(2)],
            vec![false],
        )]
    );
}

#[test]
fn test_parser_print5() {
    let fichier = "print 1,2".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![AstAppelMethode(
            "print".to_string(),
            vec![AstNombreInteger(1), AstNombreInteger(2)],
            vec![true],
        )]
    );
}

#[test]
fn test_parser_print6() {
    let fichier = "print 1;2;".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![AstAppelMethode(
            "print".to_string(),
            vec![AstNombreInteger(1), AstNombreInteger(2)],
            vec![false, false],
        )]
    );
}

#[test]
fn test_parser_print7() {
    let fichier = "print T(1);".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![AstAppelMethode(
            "print".to_string(),
            vec![AstFonction(
                AstNomFonction("T".to_string()),
                vec![AstNombreInteger(1)]
            )],
            vec![false],
        )]
    );
}

#[test]
fn test_parser_label1() {
    let fichier = "label1: print 1".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![
            AstInstrLabel(AstLabel("label1".to_string())),
            AstAppelMethode("print".to_string(), vec![AstNombreInteger(1)], vec![],)
        ]
    );
}

#[test]
fn test_parser_label2() {
    let fichier = "123 print 1".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![
            AstInstrLabel(AstLabel("123".to_string())),
            AstAppelMethode("print".to_string(), vec![AstNombreInteger(1)], vec![],)
        ]
    );
}

#[test]
fn test_parser_label3() {
    let fichier = "label5:".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![AstInstrLabel(AstLabel("label5".to_string()),)]
    );
}

#[test]
fn test_parser_label4() {
    let fichier = "4".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![AstInstrLabel(AstLabel("4".to_string()),)]
    );
}

#[test]
fn test_parser_input() {
    let fichier = "input abc".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

    assert_eq!(
        ast.liste_instructions,
        vec![AstInput(None, vec!["abc".to_string()], true, false)]
    );
}

#[test]
fn test_parser_input2() {
    let fichier = "input \"aaa\"; xxx".to_string();
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

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
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

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
    let liste_tokens = lexer(fichier).unwrap();
    let ast = parsing_programme(liste_tokens).unwrap();

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