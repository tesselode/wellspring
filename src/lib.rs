//! # Wellspring
//!
//! A small library for creating particle effects with [ggez](ggez.rs).
//!
//! Particle systems in this library work very similarly to particle systems
//! in [LÖVE](https://www.love2d.org/wiki/ParticleSystem). One notable difference
//! is that parameter changes to particle systems are only applied to new
//! particles, not existing ones. So, for example, you can change the colors
//! or sizes for new particles, and existing particles won't suddenly change
//! colors or sizes.
//!
//! Unlike in LÖVE, particle systems are configured by using a
//! `ParticleSystemSettings` struct, which is passed into `ParticleSystem::new()`.
//! Settings can be changed after the fact by modifying `ParticleSystem.settings`.
//!
//! Parameters that have a minimum and maximum value are represented
//! using Rust's range syntax.
//!
//! # Example
//! ```
//! struct MainState {
//!     particle_system: ParticleSystem<graphics::Mesh>,
//! }
//!
//! impl MainState {
//!     pub fn new(ctx: &mut Context) -> Self {
//!         // create a rectangle mesh to draw particles with
//!         let mesh = graphics::Mesh::new_rectangle(
//!             ctx,
//!             graphics::DrawMode::fill(),
//!             graphics::Rect::new(-5.0, -1.0, 10.0, 2.0),
//!             graphics::WHITE,
//!         )
//!         .unwrap();
//!         // create the particle system
//!         let particle_system = ParticleSystem::new(
//!             mesh,
//!             ParticleSystemSettings {
//!                 position: Point2::new(400.0, 300.0),
//!                 particle_lifetime: 0.25..1.0,
//!                 emission_rate: 100.0,
//!                 colors: vec![
//!                     graphics::Color::new(1.0, 1.0, 1.0, 1.0),
//!                     graphics::Color::new(1.0, 0.0, 0.0, 2.0 / 3.0),
//!                     graphics::Color::new(0.0, 0.0, 1.0, 0.0),
//!                 ],
//!                 speed: 100.0..200.0,
//!                 spread: std::f32::consts::PI * 2.0,
//!                 use_relative_angle: true,
//!                 tangential_acceleration: -200.0..200.0,
//!                 ..Default::default()
//!             },
//!         );
//!         Self { particle_system }
//!     }
//! }
//!
//! impl ggez::event::EventHandler for MainState {
//!     fn update(&mut self, ctx: &mut Context) -> GameResult {
//!         // update the particle system
//!         self.particle_system.update(ctx);
//!         Ok(())
//!     }
//!
//!     fn draw(&mut self, ctx: &mut Context) -> GameResult {
//!         graphics::clear(ctx, graphics::BLACK);
//!         // draw the particle system
//!         graphics::draw(ctx, &self.particle_system, graphics::DrawParam::new())?;
//!         graphics::present(ctx)
//!     }
//! }
//! ```
use ggez::{
	graphics,
	graphics::Color,
	nalgebra::{Point2, Rotation2, Vector2},
	Context, GameResult,
};
use rand::prelude::*;
use std::ops::{Add, Mul, Range, Sub};

fn lerp<T>(a: T, b: T, amount: f32) -> T
where
	T: Add<T, Output = T> + Sub<T, Output = T> + Mul<f32, Output = T> + Copy,
{
	a + (b - a) * amount
}

fn get_rand_in_range<T>(range: &Range<T>, rng: &mut ThreadRng) -> T
where
	T: Add<T, Output = T> + Sub<T, Output = T> + Mul<f32, Output = T> + Copy,
{
	lerp(range.start, range.end, rng.gen::<f32>())
}

struct Particle {
	lifetime: f32,
	sizes: Vec<f32>,
	colors: Vec<Color>,
	use_relative_angle: bool,
	time: f32,
	position: Point2<f32>,
	velocity: Vector2<f32>,
	damping: f32,
	acceleration: Vector2<f32>,
	radial_acceleration: f32,
	tangential_acceleration: f32,
	angle: f32,
	spin: f32,
	offset: Point2<f32>,
}

impl Particle {
	fn update(&mut self, ctx: &Context, emitter_position: Point2<f32>) {
		let mut radial_vector = self.position - emitter_position;
		if radial_vector.norm() != 0.0 {
			radial_vector = radial_vector.normalize();
		}
		let tangential_vector = Vector2::new(-radial_vector.y, radial_vector.x);
		let delta_time = ggez::timer::delta(ctx).as_secs_f32();
		self.time += 1.0 / self.lifetime * delta_time;
		self.velocity += self.acceleration * delta_time;
		self.velocity += self.radial_acceleration * radial_vector * delta_time;
		self.velocity += self.tangential_acceleration * tangential_vector * delta_time;
		self.velocity *= 1.0 / (1.0 + self.damping * delta_time);
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
				.offset(self.offset)
				.color(self.get_color()),
		)
	}
}

/// The length of time a particle system will keep
/// emitting particles.
#[derive(Debug, Clone)]
pub enum EmitterLifetime {
	/// The system will emit particles forever.
	Infinite,
	/// The system will emit particles for the specified
	/// number of seconds and then stop.
	Finite(f32),
}

/// The area in which a particle system will emit particles.
#[derive(Debug, Clone)]
pub enum EmitterShape {
	/// The particle system will emit particles at a single point.
	Point,
	/// The particle system will emit particles at any point within
	/// a rectangle of the given size and rotation (in radians).
	Rectangle(Vector2<f32>, f32),
	/// The particle system will emit particles at any point within
	/// an ellipse of the given size and rotation (in radians).
	Ellipse(Vector2<f32>, f32),
	/// The particle system will emit particles along the border
	/// of a rectangle of the given size and rotation (in radians).
	RectangleBorder(Vector2<f32>, f32),
	/// The particle system will emit particles along the border
	/// of an ellipse of the given size and rotation (in radians).
	EllipseBorder(Vector2<f32>, f32),
}

/// A configuration for a `ParticleSystem`.
#[derive(Debug, Clone)]
pub struct ParticleSystemSettings {
	/// The center of the emitter.
	pub position: Point2<f32>,
	/// How long the emitter will keep running.
	pub emitter_lifetime: EmitterLifetime,
	/// How long new particles will be visible.
	pub particle_lifetime: Range<f32>,
	/// The number of particles the emitter produces per second.
	pub emission_rate: f32,
	/// The area in which the emitter spawns particles.
	pub shape: EmitterShape,
	/// The initial speed of new particles.
	pub speed: Range<f32>,
	/// The initial direction of new particles (in radians).
	pub angle: f32,
	/// How much the initial angle of new particles varies (in radians).
	pub spread: f32,
	/// The sizes of new particles over their lifetime.
	pub sizes: Vec<f32>,
	/// The colors of new particles over their lifetime.
	pub colors: Vec<Color>,
	/// The angular velocity of new particle.
	pub spin: Range<f32>,
	/// Whether new particles' angles should always be the same as the
	/// direction of their movement.
	pub use_relative_angle: bool,
	/// The amount that new particles are slowed down each frame.
	pub damping: Range<f32>,
	/// The constant acceleration of new particles along the x and y axis.
	pub acceleration: Range<Vector2<f32>>,
	/// The acceleration of new particles relative to the center of the emitter.
	pub radial_acceleration: Range<f32>,
	/// The acceleration of new particles perpendicular to their current velocity.
	pub tangential_acceleration: Range<f32>,
	/// The offset for scaling and rotating new particles.
	pub offset: Point2<f32>,
}

impl Default for ParticleSystemSettings {
	fn default() -> Self {
		Self {
			position: Point2::new(0.0, 0.0),
			emitter_lifetime: EmitterLifetime::Infinite,
			particle_lifetime: 1.0..1.0,
			emission_rate: 10.0,
			shape: EmitterShape::Point,
			speed: 10.0..100.0,
			angle: 0.0,
			spread: std::f32::consts::PI * 2.0,
			sizes: vec![1.0],
			colors: vec![graphics::WHITE],
			spin: 0.0..0.0,
			use_relative_angle: false,
			damping: 0.0..0.0,
			acceleration: Vector2::new(0.0, 0.0)..Vector2::new(0.0, 0.0),
			radial_acceleration: 0.0..0.0,
			tangential_acceleration: 0.0..0.0,
			offset: Point2::new(0.5, 0.5),
		}
	}
}

/// Manages and displays particles.
///
/// More specifically, a `ParticleSystem` is a "world" containing:
/// - particles
/// - an "emitter" that produces particles at a regular interval
/// or on demand
/// - rules governing how the particles move
pub struct ParticleSystem<D>
where
	D: graphics::Drawable,
{
	drawable: D,
	/// The settings defining how the particle system behaves.
	pub settings: ParticleSystemSettings,
	rng: ThreadRng,
	particles: Vec<Particle>,
	running: bool,
	emit_timer: f32,
	time: f32,
}

impl<D> ParticleSystem<D>
where
	D: graphics::Drawable,
{
	/// Creates a new particle system using the specified drawable object
	/// to display each particle.
	pub fn new(drawable: D, settings: ParticleSystemSettings) -> Self {
		Self {
			drawable,
			settings,
			rng: thread_rng(),
			particles: vec![],
			running: true,
			emit_timer: 1.0,
			time: 0.0,
		}
	}

	/// Returns whether the particle emitter is currently creating new particles.
	pub fn running(&self) -> bool {
		self.running
	}

	/// Returns the number of particles in the system.
	pub fn count(&self) -> usize {
		self.particles.len()
	}

	/// Starts the particle emitter.
	pub fn start(&mut self) {
		if self.running {
			return;
		}
		self.running = true;
		self.emit_timer = 1.0;
		self.time = 0.0;
	}

	/// Stops the particle emitter.
	pub fn stop(&mut self) {
		self.running = false;
	}

	fn get_particle_position_offset(
		emitter_shape: &EmitterShape,
		rng: &mut ThreadRng,
	) -> Vector2<f32> {
		match emitter_shape {
			EmitterShape::Point => Vector2::new(0.0, 0.0),
			EmitterShape::Rectangle(size, angle) => {
				Rotation2::new(*angle)
					* Vector2::new(
						lerp(-size.x / 2.0, size.x / 2.0, rng.gen::<f32>()),
						lerp(-size.y / 2.0, size.y / 2.0, rng.gen::<f32>()),
					)
			}
			EmitterShape::Ellipse(size, angle) => {
				let particle_angle = 2.0 * std::f32::consts::PI * rng.gen::<f32>();
				let distance = rng.gen::<f32>();
				Rotation2::new(*angle)
					* Vector2::new(
						distance * particle_angle.cos() * size.x,
						distance * particle_angle.sin() * size.y,
					)
			}
			EmitterShape::RectangleBorder(size, angle) => {
				let top_left = Vector2::new(-size.x / 2.0, -size.y / 2.0);
				let top_right = Vector2::new(size.x / 2.0, -size.y / 2.0);
				let bottom_right = Vector2::new(size.x / 2.0, size.y / 2.0);
				let bottom_left = Vector2::new(-size.x / 2.0, size.y / 2.0);
				let side_boundaries = [
					size.x,
					size.x + size.y,
					size.x * 2.0 + size.y,
					size.x * 2.0 + size.y * 2.0,
				];
				let amount = side_boundaries[3] * rng.gen::<f32>();
				let offset = if amount > side_boundaries[2] {
					lerp(
						bottom_left,
						top_left,
						(amount - side_boundaries[2]) / (side_boundaries[3] - side_boundaries[2]),
					)
				} else if amount > side_boundaries[1] {
					lerp(
						bottom_right,
						bottom_left,
						(amount - side_boundaries[1]) / (side_boundaries[2] - side_boundaries[1]),
					)
				} else if amount > side_boundaries[0] {
					lerp(
						top_right,
						bottom_right,
						(amount - side_boundaries[0]) / (side_boundaries[1] - side_boundaries[0]),
					)
				} else {
					lerp(top_left, top_right, amount / side_boundaries[0])
				};
				Rotation2::new(*angle) * offset
			}
			EmitterShape::EllipseBorder(size, angle) => {
				let particle_angle = 2.0 * std::f32::consts::PI * rng.gen::<f32>();
				Rotation2::new(*angle)
					* Vector2::new(particle_angle.cos() * size.x, particle_angle.sin() * size.y)
			}
		}
	}

	/// Immediately emits the specified number of particles.
	pub fn emit(&mut self, count: usize) {
		for _ in 0..count {
			let angle = lerp(
				self.settings.angle - self.settings.spread / 2.0,
				self.settings.angle + self.settings.spread / 2.0,
				self.rng.gen::<f32>(),
			);
			let speed = get_rand_in_range(&self.settings.speed, &mut self.rng);
			let velocity = Vector2::new(speed * angle.cos(), speed * angle.sin());
			let position = self.settings.position
				+ Self::get_particle_position_offset(&self.settings.shape, &mut self.rng);
			self.particles.push(Particle {
				sizes: self.settings.sizes.clone(),
				colors: self.settings.colors.clone(),
				lifetime: get_rand_in_range(&self.settings.particle_lifetime, &mut self.rng),
				time: 0.0,
				position,
				velocity,
				damping: get_rand_in_range(&self.settings.damping, &mut self.rng),
				acceleration: get_rand_in_range(&self.settings.acceleration, &mut self.rng),
				radial_acceleration: get_rand_in_range(
					&self.settings.radial_acceleration,
					&mut self.rng,
				),
				tangential_acceleration: get_rand_in_range(
					&self.settings.tangential_acceleration,
					&mut self.rng,
				),
				angle: 0.0,
				spin: get_rand_in_range(&self.settings.spin, &mut self.rng),
				use_relative_angle: self.settings.use_relative_angle,
				offset: self.settings.offset,
			});
		}
	}

	/// Updates the particle emitter and the individual particles in the system.
	pub fn update(&mut self, ctx: &Context) {
		let delta_time = ggez::timer::delta(ctx).as_secs_f32();
		// emit new particles
		if self.running {
			self.emit_timer -= self.settings.emission_rate * delta_time;
			while self.emit_timer <= 0.0 {
				self.emit_timer += 1.0;
				self.emit(1);
			}
			self.time += delta_time;
			if let EmitterLifetime::Finite(time) = self.settings.emitter_lifetime {
				if self.time >= time {
					self.stop();
				}
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
	fn draw(&self, ctx: &mut Context, _param: graphics::DrawParam) -> GameResult {
		for particle in &self.particles {
			particle.draw(ctx, &self.drawable)?;
		}
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
