use std::{io::{stdout, Stdout, Write}, thread, time::{self, Duration}};

use crossterm::{
    cursor::{Hide, MoveTo}, event::{poll, read, Event, KeyCode}, style::Print, terminal::{enable_raw_mode, size, Clear}, ExecutableCommand, QueueableCommand
};

struct World {
    player_position_x: u16,
    player_position_y: u16,
    max_columns: u16,
    max_rows: u16,
    map: Vec<(u16, u16)>,
    die: bool
}

fn draw(mut sc: &Stdout, mut world: &World) -> std::io::Result<()>  {
    sc.queue(Clear(crossterm::terminal::ClearType::All))?;

    for l in 0..world.map.len(){
        sc.queue(MoveTo(0, l as u16))?;
        sc.queue(Print("+".repeat(world.map[l].0 as usize)))?;
        sc.queue(MoveTo(world.map[l].1, l as u16))?;
        sc.queue(Print("+".repeat((world.max_columns - world.map[l].1) as usize)))?;
    }

    sc.queue(MoveTo(world.player_position_x, world.player_position_y))?;
    sc.queue(Print("p"))?;
    sc.flush()?;

    Ok(())
}

fn finish_game(mut sc: &Stdout) -> std::io::Result<()>  {
    sc.queue(Clear(crossterm::terminal::ClearType::All))?;
    sc.queue(MoveTo(0, 5))?;
    sc.queue(Print("Well Done!, :) ."))?;
    sc.flush()?;
    Ok(())
}

fn sleep_on_draw(sleep_milis: u16) {
    let sleep = Duration::from_millis(sleep_milis as u64);
    thread::sleep(sleep);
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
        max_columns: max_columns,
        max_rows: max_lines,
        map: vec![(max_columns/2-5, max_columns/2+5); max_lines as usize],
        die: false,
    }; 
    while !world.die{
        // read inputs
        if poll(Duration::from_millis(10))?{
            let key = read()?;
            while poll(Duration::from_millis(0))?{
                let _ = read();
            }
            match key {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('q') => {
                            break;
                        },
                        KeyCode::Char('w') | KeyCode::Up => {
                            // go up 
                            if world.player_position_y > 1 {
                                world.player_position_y -= 1;
                            }
                        },
                        KeyCode::Char('s') | KeyCode::Down => {
                            // go up 
                            if world.player_position_y < max_lines - 1 {
                                world.player_position_y += 1;
                            }
                        },
                        KeyCode::Char('a') | KeyCode::Left => {
                            // go up 
                            if world.player_position_x > 1 {
                                world.player_position_x -= 1;
                            }
                        },
                        KeyCode::Char('d') | KeyCode::Right => {
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
        sleep_on_draw(10);
    }
    finish_game(&sc)?;
    Ok(())
}