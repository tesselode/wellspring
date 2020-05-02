use ggez::{
	event::MouseButton,
	graphics,
	nalgebra::{Point2, Vector2},
	Context, GameResult,
};
use wellspring::*;

struct MainState {
	particle_system: ParticleSystem<graphics::Text>,
}

impl MainState {
	pub fn new(ctx: &mut Context) -> GameResult<Self> {
		let text = graphics::Text::new("Wow!");
		let mut particle_system = ParticleSystem::new(
			text,
			ParticleSystemSettings {
				position: Point2::new(400.0, 300.0),
				particle_lifetime: 1.0..2.0,
				colors: vec![
					graphics::Color::new(1.0, 1.0, 1.0, 1.0),
					graphics::Color::new(1.0, 1.0, 1.0, 0.0),
				],
				speed: 0.0..150.0,
				spin: -2.0..2.0,
				..Default::default()
			},
		);
		particle_system.stop();
		Ok(Self { particle_system })
	}
}

impl ggez::event::EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		self.particle_system.update(ctx);
		Ok(())
	}

	fn mouse_button_down_event(
		&mut self,
		_ctx: &mut Context,
		_button: MouseButton,
		x: f32,
		y: f32,
	) {
		self.particle_system.settings.position = Point2::new(x, y);
		self.particle_system.emit(50);
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, graphics::BLACK);
		graphics::draw(ctx, &self.particle_system, graphics::DrawParam::new())?;
		let text = graphics::Text::new(format!(
			"Number of particles: {}",
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
