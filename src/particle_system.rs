use ggez::{graphics, nalgebra::Point2, Context, GameResult};

struct Particle {
	x: f32,
	y: f32,
	velocity_x: f32,
	velocity_y: f32,
	life: f32,
}

impl Particle {
	fn update(&mut self, ctx: &Context) {
		let delta_time = ggez::timer::delta(ctx).as_secs_f32();
		self.x += self.velocity_x * delta_time;
		self.y += self.velocity_y * delta_time;
	}

	fn draw<D>(&self, ctx: &mut Context, drawable: &D) -> GameResult
	where
		D: graphics::Drawable,
	{
		graphics::draw(
			ctx,
			drawable,
			graphics::DrawParam::new()
				.dest(Point2::new(self.x, self.y))
				.offset(Point2::new(0.5, 0.5)),
		)
	}
}

pub enum ParticleSystemLifetime {
	Infinite,
	Finite(f32),
}

pub struct ParticleSystem<D>
where
	D: graphics::Drawable,
{
	// configuration
	drawable: D,
	particle_lifetime: f32,
	emission_rate: f32,
	lifetime: ParticleSystemLifetime,
	min_speed: f32,
	max_speed: f32,
	angle: f32,
	spread: f32,
	// internal state
	particles: Vec<Particle>,
	running: bool,
	emit_timer: f32,
}

impl<D> ParticleSystem<D>
where
	D: graphics::Drawable,
{
	pub fn new(drawable: D) -> Self {
		Self {
			drawable,
			particle_lifetime: 1.0,
			emission_rate: 10.0,
			lifetime: ParticleSystemLifetime::Infinite,
			min_speed: 10.0,
			max_speed: 20.0,
			angle: 0.0,
			spread: std::f32::consts::PI * 2.0,
			particles: vec![],
			running: true,
			emit_timer: 1.0,
		}
	}

	pub fn emit(&mut self, count: usize) {
		for _ in 0..count {
			self.particles.push(Particle {
				x: 0.0,
				y: 0.0,
				velocity_x: 100.0,
				velocity_y: 100.0,
				life: 0.0,
			});
		}
	}

	pub fn update(&mut self, ctx: &Context) {
		let delta_time = ggez::timer::delta(ctx).as_secs_f32();
		if self.running {
			self.emit_timer -= self.emission_rate * delta_time;
			while self.emit_timer <= 0.0 {
				self.emit_timer += 1.0;
				self.emit(1);
			}
		}
		for particle in &mut self.particles {
			particle.update(ctx)
		}
	}

	pub fn draw(&self, ctx: &mut Context) -> GameResult {
		for particle in &self.particles {
			particle.draw(ctx, &self.drawable)?;
		}
		Ok(())
	}
}
