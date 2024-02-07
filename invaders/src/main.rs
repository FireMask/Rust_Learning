use std::{error::Error, io, sync::mpsc, thread, time::{Duration, Instant}};
use crossterm::{cursor::{Hide, Show}, event::{self, Event, KeyCode}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use invaders::{frame::{self, new_frame, Drawable}, gui::GUI, invaders::Invaders, player::Player, render};
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");

    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame
        }
    });
            
    //Gameloop
    let mut player = Player::new();
    let mut gui = GUI::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        //Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        //Input
        while event::poll(Duration::from_millis(1/60))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        //Updates
        player.update(delta);

        gui.update(&mut player);
    
        if invaders.update(delta) {
            audio.play("move");
        }

        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        for shot in invaders.missiles.iter_mut() {
            shot.update(delta);
        }


        //Draw & Render
        for shot in invaders.missiles.iter() {
            shot.draw(&mut curr_frame);
        }

        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders, &gui];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        //Win or Lose
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom() { 
            audio.play("lose");
            break 'gameloop;
        }

        if invaders.detect_hits(&mut player) {
            break 'gameloop;
        }
    }

    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}
