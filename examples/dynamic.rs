use ggez::{
	event::MouseButton,
	graphics,
	nalgebra::{Point2, Vector2},
	Context, GameResult,
};
use wellspring::*;

struct MainState {
	particle_system: ParticleSystem<graphics::Mesh>,
}

impl MainState {
	pub fn new(ctx: &mut Context) -> GameResult<Self> {
		let mesh = graphics::Mesh::new_circle(
			ctx,
			graphics::DrawMode::fill(),
			Point2::new(0.0, 0.0),
			8.0,
			0.1,
			graphics::WHITE,
		)?;
		let particle_system = ParticleSystem::new(
			mesh,
			ParticleSystemSettings {
				position: Point2::new(400.0, 300.0),
				particle_lifetime: 0.25..1.0,
				emission_rate: 200.0,
				colors: vec![
					graphics::Color::new(1.0, 1.0, 1.0, 1.0),
					graphics::Color::new(1.0, 0.0, 0.0, 2.0 / 3.0),
					graphics::Color::new(0.0, 0.0, 1.0, 0.0),
				],
				sizes: vec![1.0, 0.0],
				speed: 0.0..150.0,
				damping: 0.1..1.0,
				spread: 0.0,
				angle: -std::f32::consts::FRAC_PI_2,
				acceleration: Vector2::new(0.0, -150.0)..Vector2::new(0.0, -300.0),
				tangential_acceleration: -150.0..150.0,
				radial_acceleration: -25.0..100.0,
				shape: EmitterShape::EllipseBorder(Vector2::new(50.0, 50.0), 0.0),
				..Default::default()
			},
		);
		Ok(Self { particle_system })
	}
}

impl ggez::event::EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
			self.particle_system.settings.colors[1] =
				graphics::Color::new(0.0, 0.0, 1.0, 2.0 / 3.0);
		} else {
			self.particle_system.settings.colors[1] =
				graphics::Color::new(1.0, 0.0, 0.0, 2.0 / 3.0);
		}
		if ggez::input::mouse::button_pressed(ctx, MouseButton::Right) {
			self.particle_system.settings.sizes[0] = 2.0;
		} else {
			self.particle_system.settings.sizes[0] = 1.0;
		}
		self.particle_system.update(ctx);
		Ok(())
	}

	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
		self.particle_system.settings.position = Point2::new(x, y);
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, graphics::BLACK);
		graphics::draw(ctx, &self.particle_system, graphics::DrawParam::new())?;
		let text = graphics::Text::new(format!(
			"Number of particles: {}\nHold left mouse button for blue particles\nHold right mouse button for big particles",
			self.particle_system.count()
		));
		graphics::draw(ctx, &text, graphics::DrawParam::new())?;
		graphics::present(ctx)
	}
}

fn main() -> GameResult {
	let (mut ctx, mut event_loop) =
		ggez::ContextBuilder::new("particle-test", "tesselode").build()?;
	let mut main_state = MainState::new(&mut ctx)?;
	ggez::event::run(&mut ctx, &mut event_loop, &mut main_state)
}
