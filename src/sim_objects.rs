use sdl2::{render::Canvas, video::Window, rect::Rect};

pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 600;


pub struct Particle {
	pub x_position: i32,
	pub y_position: i32,
	pub x_velocity: f32,
	pub y_velocity: f32,
}

impl Particle {
	pub fn draw(&self, target_canvas: &mut Canvas<Window>) -> Result<(), String> {
		let particle_rect = Rect::new(self.x_position, self.y_position, 20, 20);
		target_canvas.draw_rect(particle_rect)?;
		target_canvas.fill_rect(particle_rect)?;
		Ok(())
	}

	pub fn update(&mut self) {
		if self.y_position < 0 || self.y_position > SCREEN_HEIGHT - 10 {
			self.y_velocity *= -1.0;
			self.y_velocity *= 0.75;
			self.y_position = SCREEN_HEIGHT - 10;
		}
		self.y_velocity += 1.5f32;
		self.x_position += (self.x_velocity) as i32;
		self.y_position += (self.y_velocity) as i32;
	}
}