mod game_objects;
use game_objects::Particle;
use sdl2::{event::Event, pixels::Color, rect::Point};

use game_objects::{SCREEN_HEIGHT, SCREEN_WIDTH};

fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _window = video_subsystem
                .window("Fluidity Test", SCREEN_WIDTH, SCREEN_HEIGHT)
                .position_centered()
                .build()
                .map_err(|e| e.to_string())?;

    let mut canvas = _window.into_canvas()
    .present_vsync()
    .build()
    .map_err(|e| e.to_string())?;

    // our first particle in the game
    let mut first_particle = Particle{radius: 10, x_velocity: 0.0, y_velocity: 1.0, position: Point::new(100, 100)};

    Ok('mainloop: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        first_particle.draw(&canvas)?;
        first_particle.update();
        canvas.present();
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
    })
}

fn main() -> Result<(), String> {
    run()?;
    Ok(())
}