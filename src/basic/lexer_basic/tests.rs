use super::*;
use crate::basic::lexer_basic::TypeMotCle::{Cdbl, Chr, Mod};
use crate::basic::lexer_basic::TypeNombre::{Double, Entier, Long, Single};
use crate::basic::lexer_basic::TypeSeparateur::{
    DIFF, DIVE, DIVR, EQU, EXP, FOIS, INF, INFE, MOINS, PARENTF, PARENTO, PLUS, SUP, SUPE,
};

#[test]
fn test_lexer_nombre() {
    let fichier = "123".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenNombre(
            123,
            0,
            0.,
            0.,
            Entier,
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_nombre2() {
    let fichier = "123.456".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenNombre(
            0,
            0,
            123.456,
            0.,
            Single,
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_nombre3() {
    let fichier = "123.456e2".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenNombre(
            0,
            0,
            0.,
            12345.6,
            Double,
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_nombre4() {
    let fichier = "10%".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenNombre(
            10,
            0,
            0.,
            0.,
            Entier,
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_nombre5() {
    let fichier = "10&".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenNombre(
            0,
            10,
            0.,
            0.,
            Long,
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_nombre6() {
    let fichier = "10!".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenNombre(
            0,
            0,
            10.,
            0.,
            Single,
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_nombre7() {
    let fichier = "10#".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenNombre(
            0,
            0,
            0.,
            10.,
            Double,
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_identifiant() {
    let fichier = "abc".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenIdentifiant(
            "abc".to_string(),
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_identifiant2() {
    let fichier = "abc$".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenIdentifiant(
            "abc$".to_string(),
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_identifiant3() {
    let fichier = "toto tata% 96325".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![
            Token::TokenIdentifiant("toto".to_string(), PositionToken::new(1, 0)),
            Token::TokenIdentifiant("tata%".to_string(), PositionToken::new(1, 5)),
            Token::TokenNombre(0, 96325, 0., 0., Long, PositionToken::new(1, 11))
        ]]
    );
}

#[test]
fn test_lexer_identifiant4() {
    let fichier = "azertyu123_bbb".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenIdentifiant(
            "azertyu123_bbb".to_string(),
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_identifiant5() {
    let fichier = "aaa
bbbb
cccc"
        .to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![
            vec![Token::TokenIdentifiant(
                "aaa".to_string(),
                PositionToken::new(1, 0)
            )],
            vec![Token::TokenIdentifiant(
                "bbbb".to_string(),
                PositionToken::new(2, 0)
            )],
            vec![Token::TokenIdentifiant(
                "cccc".to_string(),
                PositionToken::new(3, 0)
            )]
        ]
    );
}

#[test]
fn test_lexer_identifiant6() {
    let fichier = "aaa$ bbb! ccc% ddd# eeee&".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![
            Token::TokenIdentifiant("aaa$".to_string(), PositionToken::new(1, 0)),
            Token::TokenIdentifiant("bbb!".to_string(), PositionToken::new(1, 5)),
            Token::TokenIdentifiant("ccc%".to_string(), PositionToken::new(1, 10)),
            Token::TokenIdentifiant("ddd#".to_string(), PositionToken::new(1, 15)),
            Token::TokenIdentifiant("eeee&".to_string(), PositionToken::new(1, 20))
        ]]
    );
}

#[test]
fn test_lexer_identifiant7() {
    let fichier = "aaa MOD bbb".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![
            Token::TokenIdentifiant("aaa".to_string(), PositionToken::new(1, 0)),
            Token::TokenMotReserve(Mod, PositionToken::new(1, 4)),
            Token::TokenIdentifiant("bbb".to_string(), PositionToken::new(1, 8))
        ]]
    );
}

#[test]
fn test_lexer_identifiant8() {
    let fichier = "chr$".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenMotReserve(Chr, PositionToken::new(1, 0))]]
    );
}

#[test]
fn test_lexer_identifiant9() {
    let fichier = "CDBL".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenMotReserve(Cdbl, PositionToken::new(1, 0))]]
    );
}

#[test]
fn test_lexer_separateur() {
    let fichier = "2+3".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![
            Token::TokenNombre(2, 0, 0., 0., Entier, PositionToken::new(1, 0)),
            Token::TokenSeparateur(PLUS, PositionToken::new(1, 1)),
            Token::TokenNombre(3, 0, 0., 0., Entier, PositionToken::new(1, 2))
        ]]
    );
}

#[test]
fn test_lexer_separateur2() {
    let fichier = "+ - * / \\ ^ ( ) < > = >= <= <>".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![
            Token::TokenSeparateur(PLUS, PositionToken::new(1, 0)),
            Token::TokenSeparateur(MOINS, PositionToken::new(1, 2)),
            Token::TokenSeparateur(FOIS, PositionToken::new(1, 4)),
            Token::TokenSeparateur(DIVR, PositionToken::new(1, 6)),
            Token::TokenSeparateur(DIVE, PositionToken::new(1, 8)),
            Token::TokenSeparateur(EXP, PositionToken::new(1, 10)),
            Token::TokenSeparateur(PARENTO, PositionToken::new(1, 12)),
            Token::TokenSeparateur(PARENTF, PositionToken::new(1, 14)),
            Token::TokenSeparateur(INF, PositionToken::new(1, 16)),
            Token::TokenSeparateur(SUP, PositionToken::new(1, 18)),
            Token::TokenSeparateur(EQU, PositionToken::new(1, 20)),
            Token::TokenSeparateur(SUPE, PositionToken::new(1, 22)),
            Token::TokenSeparateur(INFE, PositionToken::new(1, 25)),
            Token::TokenSeparateur(DIFF, PositionToken::new(1, 28))
        ]]
    );
}

#[test]
fn test_lexer_guillemet() {
    let fichier = "\"bbb\"".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenChaine(
            "bbb".to_string(),
            PositionToken::new(1, 0)
        ),]]
    );
}

#[test]
fn test_lexer_guillemet2() {
    let fichier = "\"\"".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenChaine(
            "".to_string(),
            PositionToken::new(1, 0)
        ),]]
    );
}

#[test]
fn test_lexer_commentaire() {
    let fichier = "REM aaa bbb ccc".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(liste_tokens.len(), 0);
}

#[test]
fn test_lexer_commentaire2() {
    let fichier = "' aaa bbb ccc".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(liste_tokens.len(), 0);
}

#[test]
fn test_lexer_commentaire3() {
    let fichier = "i REM xxxx".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenIdentifiant(
            "i".to_string(),
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_commentaire4() {
    let fichier = "XXX ' zzz".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenIdentifiant(
            "XXX".to_string(),
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_commentaire5() {
    let fichier = "a=3 rem www".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![
            Token::TokenIdentifiant("a".to_string(), PositionToken::new(1, 0)),
            Token::TokenSeparateur(EQU, PositionToken::new(1, 1)),
            Token::TokenNombre(3, 0, 0., 0., Entier, PositionToken::new(1, 2))
        ]]
    );
}

#[test]
fn test_lexer_commentaire6() {
    let fichier = "remember".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenIdentifiant(
            "remember".to_string(),
            PositionToken::new(1, 0)
        )]]
    );
}

#[test]
fn test_lexer_commentaire7() {
    let fichier = "REM$".to_string();
    let liste_tokens = lexer(fichier);

    assert_eq!(
        liste_tokens,
        vec![vec![Token::TokenIdentifiant(
            "REM$".to_string(),
            PositionToken::new(1, 0)
        )]]
    );
}