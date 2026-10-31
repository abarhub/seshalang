use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal,
};
use std::io::{self, Write};

pub fn system_clear() -> Result<(), io::Error> {
    let mut stdout = io::stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    Ok(())
}

pub fn system_print(s:String)-> Result<(), io::Error>{
    print!("{}",s);
    io::stdout().flush()?;
    Ok(())
}


pub fn system_println(s:String)-> Result<(), io::Error>{
    println!("{}",s);
    Ok(())
}
