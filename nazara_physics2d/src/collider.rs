use na::{Vector2, Isometry2};
use ncollide2d::shape::{ShapeHandle, Ball};
use ncollide2d::world::CollisionGroups;
use nphysics2d::object::{ColliderDesc, DefaultColliderHandle};
use nphysics2d::material::{MaterialHandle, BasicMaterial};

use crate::physworld::PhysWorld;
use crate::rigidbody::RigidBody;

pub struct Collider
{
    handle: DefaultColliderHandle,
}

impl Collider
{
    pub fn new<T: RealField>(world: PhysWorld<T>, body: RigidBody)
    {

    }
}