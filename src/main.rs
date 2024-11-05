// Boilerplate code to just get a window on screen

use sdl2::event::Event;


fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _window = video_subsystem
                .window("Fluidity Test", 500, 500)
                .position_centered()
                .build()
                .map_err(|e| e.to_string())?;

    Ok('mainloop: loop {
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