use ggez::{
	graphics,
	nalgebra::{Point2, Vector2},
	Context, GameResult,
};
use rand::prelude::*;

struct Particle {
	lifetime: f32,
	sizes: Vec<f32>,
	time: f32,
	x: f32,
	y: f32,
	velocity_x: f32,
	velocity_y: f32,
}

impl Particle {
	fn update(&mut self, ctx: &Context) {
		let delta_time = ggez::timer::delta(ctx).as_secs_f32();
		self.time += 1.0 / self.lifetime * delta_time;
		self.x += self.velocity_x * delta_time;
		self.y += self.velocity_y * delta_time;
	}

	fn get_size(&self) -> f32 {
		if self.sizes.len() == 1 {
			return self.sizes[1];
		}
		let size_index = self.time * (self.sizes.len() - 1) as f32;
		let size_index_a = size_index.floor() as usize;
		let size_index_b = size_index.ceil() as usize;
		let size_a = self.sizes[size_index_a];
		let size_b = self.sizes[size_index_b];
		let fraction = size_index % 1.0;
		return size_a + (size_b - size_a) * fraction;
	}

	fn draw<D>(&self, ctx: &mut Context, drawable: &D) -> GameResult
	where
		D: graphics::Drawable,
	{
		let size = self.get_size();
		graphics::draw(
			ctx,
			drawable,
			graphics::DrawParam::new()
				.dest(Point2::new(self.x, self.y))
				.scale(Vector2::new(size, size))
				.offset(Point2::new(0.5, 0.5)),
		)
	}
}

pub struct ParticleSystem<D>
where
	D: graphics::Drawable,
{
	// configuration
	drawable: D,
	pub x: f32,
	pub y: f32,
	pub particle_lifetime: f32,
	pub emission_rate: f32,
	pub min_speed: f32,
	pub max_speed: f32,
	pub angle: f32,
	pub spread: f32,
	pub sizes: Vec<f32>,
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
			x: 0.0,
			y: 0.0,
			particle_lifetime: 1.0,
			emission_rate: 10.0,
			min_speed: 10.0,
			max_speed: 20.0,
			angle: 0.0,
			spread: std::f32::consts::PI * 2.0,
			sizes: vec![1.0],
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
				sizes: self.sizes.clone(),
				lifetime: self.particle_lifetime,
				time: 0.0,
				x: self.x,
				y: self.y,
				velocity_x,
				velocity_y,
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
}

impl<D> graphics::Drawable for ParticleSystem<D>
where
	D: graphics::Drawable,
{
	fn draw(&self, ctx: &mut Context, param: graphics::DrawParam) -> GameResult {
		graphics::push_transform(ctx, Some(param.to_matrix()));
		graphics::apply_transformations(ctx)?;
		for particle in &self.particles {
			particle.draw(ctx, &self.drawable)?;
		}
		graphics::pop_transform(ctx);
		graphics::apply_transformations(ctx)?;
		Ok(())
	}

	fn dimensions(&self, _ctx: &mut Context) -> Option<graphics::Rect> {
		None
	}

	fn set_blend_mode(&mut self, _mode: Option<graphics::BlendMode>) {}

	fn blend_mode(&self) -> Option<graphics::BlendMode> {
		None
	}
}
