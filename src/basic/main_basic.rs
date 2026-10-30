use crate::basic::exec_basic::execute;
use crate::basic::parsing_basic::parse_basic;
use crate::basic::utils_basic::Sortie;
use std::fs;

pub fn parse_basic_file(fichier: String) -> std::io::Result<()> {
    let contenu = fs::read_to_string(fichier)?;

    let programme = parse_basic(contenu)?;

    let mut sortie = Sortie::new(true);

    execute(programme, &mut sortie);
    Ok(())
}

#[cfg(test)]
mod tests;

