use ggez::{graphics, nalgebra::Point2, Context, GameResult};
use rand::prelude::*;

struct Particle {
	x: f32,
	y: f32,
	velocity_x: f32,
	velocity_y: f32,
	time: f32,
	lifetime: f32,
}

impl Particle {
	fn update(&mut self, ctx: &Context) {
		let delta_time = ggez::timer::delta(ctx).as_secs_f32();
		self.time += 1.0 / self.lifetime * delta_time;
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
	rng: ThreadRng,
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
			rng: thread_rng(),
			particles: vec![],
			running: true,
			emit_timer: 1.0,
		}
	}

	pub fn emit(&mut self, count: usize) {
		let min_angle = self.angle - self.spread / 2.0;
		let max_angle = self.angle + self.spread / 2.0;
		let angle = min_angle + (max_angle - min_angle) * self.rng.gen::<f32>();
		let speed = self.min_speed + (self.max_speed - self.min_speed) * self.rng.gen::<f32>();
		let velocity_x = speed * angle.cos();
		let velocity_y = speed * angle.sin();
		for _ in 0..count {
			self.particles.push(Particle {
				x: 100.0,
				y: 100.0,
				velocity_x,
				velocity_y,
				time: 0.0,
				lifetime: self.particle_lifetime,
			});
		}
	}

	pub fn update(&mut self, ctx: &Context) {
		let delta_time = ggez::timer::delta(ctx).as_secs_f32();
		// emit new particles
		if self.running {
			self.emit_timer -= self.emission_rate * delta_time;
			while self.emit_timer <= 0.0 {
				self.emit_timer += 1.0;
				self.emit(1);
			}
		}
		// update existing particles
		for i in (0..self.particles.len()).rev() {
			let particle = &mut self.particles[i];
			particle.update(ctx);
			if particle.time >= 1.0 {
				self.particles.remove(i);
			}
		}
	}

	pub fn draw(&self, ctx: &mut Context) -> GameResult {
		for particle in &self.particles {
			particle.draw(ctx, &self.drawable)?;
		}
		Ok(())
	}
}
