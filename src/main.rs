use ggez::{graphics, nalgebra::Point2, Context, GameResult};
use ggez_particle_system::*;

struct MainState {
	particle_system: ParticleSystem<graphics::Mesh>,
}

impl MainState {
	pub fn new(ctx: &mut Context) -> Self {
		let mesh = graphics::Mesh::new_rectangle(
			ctx,
			graphics::DrawMode::fill(),
			graphics::Rect::new(-5.0, -1.0, 10.0, 2.0),
			graphics::WHITE,
		)
		.unwrap();
		let mut particle_system = ParticleSystem::new(
			mesh,
			ParticleSystemSettings {
				position: Point2::new(400.0, 300.0),
				particle_lifetime: 0.25..1.0,
				emission_rate: 100.0,
				colors: vec![
					graphics::Color::new(1.0, 1.0, 1.0, 1.0),
					graphics::Color::new(1.0, 0.0, 0.0, 2.0 / 3.0),
					graphics::Color::new(0.0, 0.0, 1.0, 0.0),
				],
				speed: 100.0..200.0,
				damping: 1.0..10.0,
				spread: std::f32::consts::PI * 2.0,
				use_relative_angle: true,
				tangential_acceleration: -200.0..200.0,
				..Default::default()
			},
		);
		particle_system.emit(10);
		Self { particle_system }
	}
}

impl ggez::event::EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		self.particle_system.update(ctx);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, graphics::BLACK);
		graphics::draw(ctx, &self.particle_system, graphics::DrawParam::new())?;
		let text = graphics::Text::new(format!("{}", self.particle_system.count()));
		graphics::draw(ctx, &text, graphics::DrawParam::new())?;
		graphics::present(ctx)
	}
}

fn main() -> GameResult {
	let (mut ctx, mut event_loop) =
		ggez::ContextBuilder::new("particle-test", "tesselode").build()?;
	let mut main_state = MainState::new(&mut ctx);
	ggez::event::run(&mut ctx, &mut event_loop, &mut main_state)
}
