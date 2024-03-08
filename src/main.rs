use std::{io::{stdout, Stdout, Write, Result}, thread, time::{self, Duration}};

use crossterm::{
    cursor::{Hide, MoveTo}, event::{poll, read, Event, KeyCode}, style::Print, terminal::{enable_raw_mode, size, Clear}, ExecutableCommand, QueueableCommand
};
use rand::Rng;


struct World {
    player_position_x: u16,
    player_position_y: u16,
    max_columns: u16,
    max_rows: u16,
    map: Vec<(u16, u16)>,
    died: bool,
    next_right: u16,
    next_left: u16,
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
    sc.queue(Print("^"))?;
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


fn physics(mut world: World) -> Result<World>{
    let mut rng = rand::thread_rng();
    // check if player hits the ground
    if world.player_position_x < world.map[world.player_position_y as usize].0 ||
        world.player_position_x > world.map[world.player_position_y as usize].1 {
            world.died = true;
        }

    for l in (0..world.map.len() - 1).rev() {
        world.map[l+1] = world.map[l];
    }
    if world.next_left > world.map[0].0 {
        world.map[0].0 += 1;
    }
    if world.next_left < world.map[0].0 {
        world.map[0].0 -= 1;
    }
    if world.next_right > world.map[0].1 {
        world.map[0].1 += 1;
    }
    if world.next_right < world.map[0].1 {
        world.map[0].1 -= 1;
    }
    if world.next_left == world.map[0].0 {
        world.next_left = rng.gen_range(
            world.next_left-5..world.next_left+5
        );
    }
    if world.next_right == world.map[0].1 {
        world.next_right = rng.gen_range(
            world.next_right-5..world.next_right+5
        );
    }
    Ok(world)
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
        died: false,
        next_left: max_columns / 2 -7,
        next_right: max_columns / 2 + 7
    }; 
    while !world.died{
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
        world = physics(world).unwrap();
        //draw the world
        draw(&sc, &world)?;
        sleep_on_draw(50);
    }
    finish_game(&sc)?;
    Ok(())
}