use ggez::{
	graphics,
	graphics::Color,
	nalgebra::{Point2, Vector2},
	Context, GameResult,
};
use rand::prelude::*;
use std::ops::{Add, Mul, Sub};

fn lerp<T>(a: T, b: T, amount: f32) -> T
where
	T: Add<T, Output = T> + Sub<T, Output = T> + Mul<f32, Output = T> + Copy,
{
	a + (b - a) * amount
}

pub struct Range<T>(pub T, pub T);

impl<T> Range<T> {
	pub fn single(value: T) -> Self
	where
		T: Copy,
	{
		Range(value, value)
	}

	fn get_rand(&self, rng: &mut ThreadRng) -> T
	where
		T: Add<T, Output = T> + Sub<T, Output = T> + Mul<f32, Output = T> + Copy,
	{
		lerp(self.0, self.1, rng.gen::<f32>())
	}
}

struct Particle {
	lifetime: f32,
	sizes: Vec<f32>,
	colors: Vec<Color>,
	use_relative_angle: bool,
	time: f32,
	position: Point2<f32>,
	velocity: Vector2<f32>,
	acceleration: Vector2<f32>,
	radial_acceleration: f32,
	tangential_acceleration: f32,
	angle: f32,
	spin: f32,
}

impl Particle {
	fn update(&mut self, ctx: &Context, emitter_position: Point2<f32>) {
		let mut radial_vector: Vector2<f32> = (self.position - emitter_position).into();
		if radial_vector.norm() != 0.0 {
			radial_vector = radial_vector.normalize();
		}
		let tangential_vector = Vector2::new(-radial_vector.y, radial_vector.x);
		let delta_time = ggez::timer::delta(ctx).as_secs_f32();
		self.time += 1.0 / self.lifetime * delta_time;
		self.velocity += self.acceleration * delta_time;
		self.velocity += self.radial_acceleration * radial_vector * delta_time;
		self.velocity += self.tangential_acceleration * tangential_vector * delta_time;
		self.position += self.velocity * delta_time;
		self.angle += self.spin * delta_time;
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

	fn get_angle(&self) -> f32 {
		if self.use_relative_angle {
			self.velocity.y.atan2(self.velocity.x)
		} else {
			self.angle
		}
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
				.dest(self.position)
				.scale(Vector2::new(size, size))
				.rotation(self.get_angle())
				.offset(Point2::new(0.5, 0.5))
				.color(self.get_color()),
		)
	}
}

pub struct ParticleSystemSettings {
	pub position: Point2<f32>,
	pub particle_lifetime: Range<f32>,
	pub emission_rate: f32,
	pub speed: Range<f32>,
	pub angle: f32,
	pub spread: f32,
	pub sizes: Vec<f32>,
	pub colors: Vec<Color>,
	pub spin: Range<f32>,
	pub use_relative_angle: bool,
	pub acceleration: Range<Vector2<f32>>,
	pub radial_acceleration: Range<f32>,
	pub tangential_acceleration: Range<f32>,
}

impl Default for ParticleSystemSettings {
	fn default() -> Self {
		Self {
			position: Point2::new(0.0, 0.0),
			particle_lifetime: Range::single(1.0),
			emission_rate: 10.0,
			speed: Range(10.0, 100.0),
			angle: 0.0,
			spread: std::f32::consts::PI * 2.0,
			sizes: vec![1.0],
			colors: vec![graphics::WHITE],
			spin: Range::single(0.0),
			use_relative_angle: false,
			acceleration: Range::single(Vector2::new(0.0, 0.0)),
			radial_acceleration: Range::single(0.0),
			tangential_acceleration: Range::single(0.0),
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
		let angle = lerp(
			self.settings.angle - self.settings.spread / 2.0,
			self.settings.angle + self.settings.spread / 2.0,
			self.rng.gen::<f32>(),
		);
		let speed = self.settings.speed.get_rand(&mut self.rng);
		let velocity = Vector2::new(speed * angle.cos(), speed * angle.sin());
		for _ in 0..count {
			self.particles.push(Particle {
				sizes: self.settings.sizes.clone(),
				colors: self.settings.colors.clone(),
				lifetime: self.settings.particle_lifetime.get_rand(&mut self.rng),
				time: 0.0,
				position: self.settings.position,
				velocity,
				acceleration: self.settings.acceleration.get_rand(&mut self.rng),
				radial_acceleration: self.settings.radial_acceleration.get_rand(&mut self.rng),
				tangential_acceleration: self
					.settings
					.tangential_acceleration
					.get_rand(&mut self.rng),
				angle: 0.0,
				spin: self.settings.spin.get_rand(&mut self.rng),
				use_relative_angle: self.settings.use_relative_angle,
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
			particle.update(ctx, self.settings.position);
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
