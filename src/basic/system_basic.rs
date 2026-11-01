use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal,
};
use std::io::{self, ErrorKind, Write};

pub fn system_clear() -> Result<(), io::Error> {
    let mut stdout = io::stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    Ok(())
}

pub fn system_print(s: String) -> Result<(), io::Error> {
    print!("{}", s);
    io::stdout().flush()?;
    Ok(())
}

pub fn system_println(s: String) -> Result<(), io::Error> {
    println!("{}", s);
    Ok(())
}

pub fn system_locate(
    x: Option<i16>,
    y: Option<i16>,
    affichage_cursor: Option<bool>,
) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    if x.is_some() && y.is_some() {
        execute!(stdout, MoveTo(x.unwrap() as u16, y.unwrap() as u16))?;
    } else if x.is_some() || y.is_some() {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "Erreur dans les parametres de system_locate",
        ));
    }
    if affichage_cursor.is_some() {
        if affichage_cursor.unwrap() {
            execute!(stdout, Show)?;
        } else {
            execute!(stdout, Hide)?;
        }
    }

    Ok(())
}

pub fn system_get_cursor() -> Result<(i16, i16), io::Error> {
    let (col, ligne) = cursor::position()?;
    Ok((col as i16, ligne as i16))
}
