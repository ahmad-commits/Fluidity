use sdl2::{
	rect::Point,
	gfx::primitives::DrawRenderer,
	render::Canvas,
	video::Window,
	pixels::Color
};

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

const GRAVITY: f32 = 0.65;

pub struct Particle {
	pub radius: i16,
	pub x_velocity: f32,
	pub y_velocity: f32,
	pub position: Point,
}

impl Particle {
	pub fn draw(&self, target_canvas: &Canvas<Window>) -> Result<(), String> {
		target_canvas.filled_circle(self.position.x as i16, self.position.y as i16, self.radius, Color::BLUE)?;
		Ok(())
	}

	pub fn update(&mut self) {
		if self.position.y > SCREEN_HEIGHT as i32 {
			self.y_velocity *= -1.0;
			self.position.y = ((SCREEN_HEIGHT as i16) + self.radius) as i32;
		}
		self.y_velocity += GRAVITY;
		self.position.x += self.x_velocity as i32;
		self.position.y += self.y_velocity as i32;
	}
}