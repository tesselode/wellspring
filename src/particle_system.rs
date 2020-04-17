use ggez::{
	graphics,
	graphics::Color,
	nalgebra::{Point2, Vector2},
	Context, GameResult,
};
use rand::prelude::*;

fn lerp(a: f32, b: f32, amount: f32) -> f32 {
	a + (b - a) * amount
}

struct Particle {
	lifetime: f32,
	sizes: Vec<f32>,
	colors: Vec<Color>,
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
			return self.sizes[0];
		}
		let size_index = self.time * (self.sizes.len() - 1) as f32;
		let size_index_a = size_index.floor() as usize;
		let size_index_b = size_index.ceil() as usize;
		let size_a = self.sizes[size_index_a];
		let size_b = self.sizes[size_index_b];
		let fraction = size_index % 1.0;
		lerp(size_a, size_b, fraction)
	}

	fn get_color(&self) -> Color {
		if self.colors.len() == 1 {
			return self.colors[0];
		}
		let color_index = self.time * (self.colors.len() - 1) as f32;
		let color_index_a = color_index.floor() as usize;
		let color_index_b = color_index.ceil() as usize;
		let color_a = self.colors[color_index_a];
		let color_b = self.colors[color_index_b];
		let fraction = color_index % 1.0;
		return Color::new(
			lerp(color_a.r, color_b.r, fraction),
			lerp(color_a.g, color_b.g, fraction),
			lerp(color_a.b, color_b.b, fraction),
			lerp(color_a.a, color_b.a, fraction),
		);
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
				.offset(Point2::new(0.5, 0.5))
				.color(self.get_color()),
		)
	}
}

pub struct ParticleSystemSettings {
	pub x: f32,
	pub y: f32,
	pub min_particle_lifetime: f32,
	pub max_particle_lifetime: f32,
	pub emission_rate: f32,
	pub min_speed: f32,
	pub max_speed: f32,
	pub angle: f32,
	pub spread: f32,
	pub sizes: Vec<f32>,
	pub colors: Vec<Color>,
}

impl Default for ParticleSystemSettings {
	fn default() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			min_particle_lifetime: 1.0,
			max_particle_lifetime: 1.0,
			emission_rate: 10.0,
			min_speed: 10.0,
			max_speed: 100.0,
			angle: 0.0,
			spread: std::f32::consts::PI * 2.0,
			sizes: vec![1.0],
			colors: vec![graphics::WHITE],
		}
	}
}

pub struct ParticleSystem<D>
where
	D: graphics::Drawable,
{
	drawable: D,
	pub settings: ParticleSystemSettings,
	rng: ThreadRng,
	particles: Vec<Particle>,
	running: bool,
	emit_timer: f32,
}

impl<D> ParticleSystem<D>
where
	D: graphics::Drawable,
{
	pub fn new(drawable: D, settings: ParticleSystemSettings) -> Self {
		Self {
			drawable,
			settings,
			rng: thread_rng(),
			particles: vec![],
			running: true,
			emit_timer: 1.0,
		}
	}

	pub fn running(&self) -> bool {
		self.running
	}

	pub fn count(&self) -> usize {
		self.particles.len()
	}

	pub fn start(&mut self) {
		if self.running {
			return;
		}
		self.running = true;
		self.emit_timer = 1.0;
	}

	pub fn stop(&mut self) {
		self.running = false;
	}

	pub fn emit(&mut self, count: usize) {
		let lifetime = lerp(
			self.settings.min_particle_lifetime,
			self.settings.max_particle_lifetime,
			self.rng.gen::<f32>(),
		);
		let angle = lerp(
			self.settings.angle - self.settings.spread / 2.0,
			self.settings.angle + self.settings.spread / 2.0,
			self.rng.gen::<f32>(),
		);
		let speed = lerp(
			self.settings.min_speed,
			self.settings.max_speed,
			self.rng.gen::<f32>(),
		);
		let velocity_x = speed * angle.cos();
		let velocity_y = speed * angle.sin();
		for _ in 0..count {
			self.particles.push(Particle {
				sizes: self.settings.sizes.clone(),
				colors: self.settings.colors.clone(),
				lifetime,
				time: 0.0,
				x: self.settings.x,
				y: self.settings.y,
				velocity_x,
				velocity_y,
			});
		}
	}

	pub fn update(&mut self, ctx: &Context) {
		let delta_time = ggez::timer::delta(ctx).as_secs_f32();
		// emit new particles
		if self.running {
			self.emit_timer -= self.settings.emission_rate * delta_time;
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
