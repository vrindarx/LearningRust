use std::io::{stdout, Write};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
};

fn main() -> Result<()> {
    // using the macro
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Blue),
        Print("Styled text here."),
        ResetColor
    )?;

    // or using functions
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::White))?
        .execute(Print("Styled text here."))?
        .execute(ResetColor)?;
    
    Ok(())
}