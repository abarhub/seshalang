use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
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

pub fn system_color(foreground: Option<i16>, background: Option<i16>) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    match (foreground, background) {
        (Some(foreground_color), Some(background_color)) => execute!(
            stdout,
            SetForegroundColor(calcul_couleur(foreground_color)?),
            SetBackgroundColor(calcul_couleur(background_color)?)
        )?,
        (Some(foreground_color), None) => {
            execute!(stdout, SetForegroundColor(calcul_couleur(foreground_color)?))?
        }
        _ => {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "Erreur dans les parametres de system_color",
            ));
        }
    }
    Ok(())
}

fn calcul_couleur(couleur: i16) -> Result<Color, io::Error> {
    match couleur {
        0 => Ok(Color::Black),
        1 => Ok(Color::DarkBlue),
        2 => Ok(Color::DarkGreen),
        3 => Ok(Color::DarkCyan),
        4 => Ok(Color::DarkRed),
        5 => Ok(Color::DarkMagenta),
        6 => Ok(Color::DarkYellow),
        7 => Ok(Color::Grey),
        8 => Ok(Color::DarkGrey),
        9 => Ok(Color::Blue),
        10 => Ok(Color::Green),
        11 => Ok(Color::Cyan),
        12 => Ok(Color::Red),
        13 => Ok(Color::Magenta),
        14 => Ok(Color::Yellow),
        15 => Ok(Color::White),
        _ => Err(io::Error::new(ErrorKind::InvalidData, "Couleur invalide")),
    }
}
