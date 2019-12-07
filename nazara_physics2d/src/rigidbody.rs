use nalgebra::{Vector2, Point2, RealField};
use nphysics2d::object::{BodyStatus, RigidBodyDesc, DefaultBodyHandle};
use nphysics2d::math::{Velocity, Inertia};

use crate::physworld::PhysWorld;

pub struct RigidBody
{
    handle: DefaultBodyHandle,
}

impl RigidBody
{
    pub fn new<T: RealField>(mut world: PhysWorld<T>) -> RigidBody
    {
        let body = RigidBodyDesc::new().build();
        RigidBody
        {
            handle: world.body_set.insert(body),
        }
    }
}