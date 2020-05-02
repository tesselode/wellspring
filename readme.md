# Wellspring

**Wellspring** is a particle system library for [ggez](ggez.rs) heavily inspired by [LÖVE](love2d.org)'s particle systems.

## Example

![](https://i.imgur.com/ka3UzwP.gif)

```rs
let mesh = graphics::Mesh::new_circle(
	ctx,
	graphics::DrawMode::fill(),
	Point2::new(0.0, 0.0),
	8.0,
	0.1,
	graphics::WHITE,
)?;
let particle_system = ParticleSystem::new(
	mesh,
	ParticleSystemSettings {
		position: Point2::new(400.0, 300.0),
		particle_lifetime: 0.25..1.0,
		emission_rate: 200.0,
		colors: vec![
			graphics::Color::new(1.0, 1.0, 1.0, 1.0),
			graphics::Color::new(1.0, 0.0, 0.0, 2.0 / 3.0),
			graphics::Color::new(0.0, 0.0, 1.0, 0.0),
		],
		sizes: vec![1.0, 0.0],
		speed: 0.0..150.0,
		damping: 0.1..1.0,
		spread: 0.0,
		angle: -std::f32::consts::FRAC_PI_2,
		acceleration: Vector2::new(0.0, -150.0)..Vector2::new(0.0, -300.0),
		tangential_acceleration: -150.0..150.0,
		radial_acceleration: -25.0..100.0,
		shape: EmitterShape::EllipseBorder(Vector2::new(50.0, 50.0), 0.0),
		..Default::default()
	},
);
```
This is a snippet from the [dynamic](https://github.com/tesselode/wellspring/blob/master/examples/dynamic.rs) example.
