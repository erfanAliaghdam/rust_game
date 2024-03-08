use std::io::stdout;

use crossterm::{
    style::Print,
    ExecutableCommand, 
    QueueableCommand
};

fn main() -> std::io::Result<()> {
    let mut sc = stdout();
    sc.queue(Print("Styled text here."))?;
    Ok(())
}