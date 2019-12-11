extern crate nazara_physics2d;
extern crate nalgebra;
extern crate ncollide2d;

use std::time::Instant;

use nazara_physics2d::physworld::PhysWorld;
use nazara_physics2d::rigidbody::RigidBody;
use nazara_physics2d::rigidbody::RigidBodyStatus;
use nazara_physics2d::collider::Collider;

use nalgebra::Vector2;
use nalgebra::Point2;

fn main()
{
    let mut world = PhysWorld::new(Vector2::y() * -9.81);

    let collider = Collider::Circle{ radius: 4.0, offset: None };
    let body = RigidBody::new(&mut world, 1.0, RigidBodyStatus::Dynamic, Some(&collider));

    let ground_collider = Collider::Box{ size: Vector2::new(20.0, 5.0), offset: Some(Point2::new(-10.0, -30.0)) };
    let ground = RigidBody::new(&mut world, 0.0, RigidBodyStatus::Static, Some(&ground_collider)); // mass is useles for static bodies 

    let mut instant = Instant::now();
    loop
    {
        world.step((instant.elapsed().as_micros() as f64)/1_000_000.0);
        instant = Instant::now();

        println!("{:?}", body.get_position());
    }
}
