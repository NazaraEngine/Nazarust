extern crate nazara_physics2d;
extern crate nalgebra;
extern crate ncollide2d;

use nazara_physics2d::physworld::PhysWorld;
use nazara_physics2d::rigidbody::RigidBody;
use nazara_physics2d::collider::Collider;
use nalgebra::Vector2;

fn main()
{
    let mut world = PhysWorld::new(Vector2::y() * -9.81);

    let collider = Collider::Circle(4.0, None);
    let body = RigidBody::new(&mut world, 1.0, Some(&collider));

    loop
    {
        world.step(0.5);
        println!("{:?}", body.get_position());
    }
}
