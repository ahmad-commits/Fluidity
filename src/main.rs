mod sim_objects;
use std::time::Duration;
use sim_objects::Particle;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sim_objects::{SCREEN_HEIGHT, SCREEN_WIDTH};

fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _window = video_subsystem
                .window("Fluidity Test", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
                .position_centered()
                .build()
                .map_err(|e| e.to_string())?;

    let mut canvas = _window.into_canvas()
    .present_vsync()
    .build()
    .map_err(|e| e.to_string())?;

    let rows = 2;
    let columns = 5;

    let mut particles = Vec::new();

    let mut x_pos;
    let mut y_pos = 200; 
    for _ in 0..rows { //rows
        x_pos = SCREEN_WIDTH/2;
        for _ in 0..columns { //columns
            particles.push(Particle{x_velocity: 0.0, y_velocity: 1.0, x_position: x_pos, y_position: y_pos});
            x_pos += 30
        }
        y_pos += 30
    }

    let mut is_paused = false;

    Ok('mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {is_paused = !is_paused},
                _ => {}
            }
        }

        std::thread::sleep(Duration::from_millis(10));
        if !is_paused {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            canvas.set_draw_color(Color::RED);
            for particle in &mut particles {
                particle.draw(&mut canvas)?;
                particle.update();
            }

            canvas.present();
        };

    })
}

fn main() -> Result<(), String> {
    run()?;
    Ok(())
}