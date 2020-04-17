mod particle_system;

use ggez::{event::KeyCode, graphics, input::keyboard::KeyMods, Context, GameResult};
use particle_system::*;

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
				x: 400.0,
				y: 300.0,
				min_particle_lifetime: 0.25,
				max_particle_lifetime: 1.0,
				emission_rate: 100.0,
				colors: vec![
					graphics::Color::new(1.0, 1.0, 1.0, 1.0),
					graphics::Color::new(1.0, 0.0, 0.0, 2.0 / 3.0),
					graphics::Color::new(0.0, 0.0, 1.0, 0.0),
				],
				min_speed: 10.0,
				max_speed: 100.0,
				use_relative_angle: true,
				min_acceleration_y: 500.0,
				max_acceleration_y: 500.0,
				min_radial_acceleration: 300.0,
				max_radial_acceleration: 500.0,
				min_tangential_acceleration: -300.0,
				max_tangential_acceleration: 300.0,
				..Default::default()
			},
		);
		particle_system.emit(10);
		Self { particle_system }
	}
}

impl ggez::event::EventHandler for MainState {
	fn key_down_event(
		&mut self,
		_ctx: &mut Context,
		keycode: KeyCode,
		_keymods: KeyMods,
		_repeat: bool,
	) {
		match keycode {
			KeyCode::Space => {
				if self.particle_system.running() {
					self.particle_system.stop();
				} else {
					self.particle_system.start();
				}
			}
			_ => {}
		}
	}

	fn update(&mut self, ctx: &mut Context) -> GameResult {
		let mouse_position = ggez::input::mouse::position(ctx);
		self.particle_system.settings.x = mouse_position.x;
		self.particle_system.settings.y = mouse_position.y;
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
