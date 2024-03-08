use std::{io::{stdout, Stdout, Write}, time::Duration};

use crossterm::{
    cursor::{Hide, MoveTo}, event::{poll, read, Event, KeyCode}, style::Print, terminal::{enable_raw_mode, size}, ExecutableCommand, QueueableCommand
};

struct World {
    player_position_x: u16,
    player_position_y: u16,
}

fn draw(mut sc: &Stdout, mut world: &World) -> std::io::Result<()>  {
    sc.queue(MoveTo(world.player_position_x, world.player_position_y))?;
    sc.queue(Print("p"))?;
    sc.flush()?;
    Ok(())
}


fn main() -> std::io::Result<()> {
    // initialize the screen
    let mut sc = stdout();
    let (max_columns, max_lines) = size().unwrap();
    // hide the cursor
    sc.execute(Hide)?;
    // enable raw mode will make terminal inputs work without enter 
    enable_raw_mode()?;
    // initialize the world
    let mut world = World{
        player_position_x: max_columns / 2,
        player_position_y: max_lines - 1,
    }; 
    loop{
        // read inputs
        if poll(Duration::from_millis(10))?{
            let key = read()?;
            match key {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('q') => {
                            break;
                        },
                        KeyCode::Char('w') => {
                            // go up 
                            if world.player_position_y > 1 {
                                world.player_position_y -= 1;
                            }
                        },
                        KeyCode::Char('s') => {
                            // go up 
                            if world.player_position_y < max_lines - 1 {
                                world.player_position_y += 1;
                            }
                        },
                        KeyCode::Char('a') => {
                            // go up 
                            if world.player_position_x > 1 {
                                world.player_position_x -= 1;
                            }
                        },
                        KeyCode::Char('d') => {
                            // go up 
                            if world.player_position_x < max_columns - 1 {
                                world.player_position_x += 1;
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        //do some physics
        //draw the world
        draw(&sc, &world)?;
    }

    Ok(())
}