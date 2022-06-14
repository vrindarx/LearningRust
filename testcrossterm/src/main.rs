use std::io::{stdout, Write};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, self, Stylize},
    event,
    terminal, cursor, Result
};

fn main() -> Result<()> {

    ////
    // // using the macro
    // execute!(
    //     stdout(),
    //     SetForegroundColor(Color::White),
    //     SetBackgroundColor(Color::Blue),
    //     Print("Styled text here."),
    //     ResetColor
    // )?;

    // // or using functions
    // stdout()
    //     .execute(SetForegroundColor(Color::Blue))?
    //     .execute(SetBackgroundColor(Color::White))?
    //     .execute(Print("Styled text here."))?
    //     .execute(ResetColor)?;

    ////

    // let mut stdout = stdout();

    // stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    // for y in 0..40 {
    //     for x in 0..150 {
    //     if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
    //         // in this loop we are more efficient by not flushing the buffer.
    //         stdout
    //         .queue(cursor::MoveTo(x,y))?
    //         .queue(style::PrintStyledContent( "█".blue()))?;
    //     }
    //     }
    // }
    // stdout.flush()?;



    Ok(())

}