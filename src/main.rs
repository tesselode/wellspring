mod particle_system;

use ggez::{graphics, nalgebra::Point2, Context, GameResult};
use particle_system::*;

struct MainState {
	particle_system: ParticleSystem<graphics::Mesh>,
}

impl MainState {
	pub fn new(ctx: &mut Context) -> Self {
		let mesh = graphics::Mesh::new_circle(
			ctx,
			graphics::DrawMode::fill(),
			Point2::new(0.0, 0.0),
			5.0,
			0.1,
			graphics::WHITE,
		)
		.unwrap();
		let mut particle_system = ParticleSystem::new(mesh);
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
		self.particle_system.draw(ctx)?;
		graphics::present(ctx)
	}
}

fn main() -> GameResult {
	let (mut ctx, mut event_loop) =
		ggez::ContextBuilder::new("particle-test", "tesselode").build()?;
	let mut main_state = MainState::new(&mut ctx);
	ggez::event::run(&mut ctx, &mut event_loop, &mut main_state)
}
