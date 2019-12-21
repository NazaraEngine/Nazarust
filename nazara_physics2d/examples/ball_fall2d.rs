extern crate nazara_physics2d;
extern crate nalgebra;
extern crate ncollide2d;

use std::time::Instant;

use nazara_physics2d::physworld::PhysWorld;
use nazara_physics2d::rigidbody::RigidBodyBuilder;
//use nazara_physics2d::rigidbody::RigidBody;
use nazara_physics2d::collider::Collider;
use nazara_physics2d::material::Material;

use nalgebra::Vector2;
use nalgebra::Point2;

fn main()
{
	let mut world = PhysWorld::new(Vector2::y() * -9.81);

	let material = Material::new(0.1, 0.0, None);
	let collider = Collider::Circle{ radius: 4.0, offset: None };
	let body = RigidBodyBuilder::new().mass(1.2) // simply set the mass of the body
									  .collider(&collider, Some(&material)) // set a collider and optionally a material, material and collider are linked so they must be set together
									  .build(&mut world); // simply build the rigidbody into the world

	let ground_collider = Collider::Box{ size: Vector2::new(20.0, 5.0), offset: Some(Point2::new(-10.0, -30.0)) }; // the offset is an internal translation, and it won't change the body's position, here (0, 0)
	let ground = RigidBodyBuilder::new().make_static() // make the body static, so it won't move and it won't need a mass
										.collider(&ground_collider, None) // set the collider and no material
										.build(&mut world); // mwork as the other body 

	let mut instant = Instant::now(); // record the actual instant

	loop
	{
		world.step((instant.elapsed().as_nanos() as f64) / 1_000_000_000.0); // tell the physical world to simulate the elapsed time since last step
		instant = Instant::now(); // record the instant

		println!("{:?}", body.get_position().unwrap()); // print the ball position
	}
}
