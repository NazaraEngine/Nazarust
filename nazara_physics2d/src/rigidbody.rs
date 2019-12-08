use nalgebra::{Vector2, Point2, RealField};
use nphysics2d::object::{RigidBodyDesc, ColliderDesc, DefaultBodyHandle, BodyPartHandle, DefaultColliderHandle};
use nphysics2d::object;
use nphysics2d::math::{Velocity, Inertia};

use crate::physworld::PhysWorld;
use crate::collider::Collider;

pub struct RigidBody<'a, T: RealField>
{
    body: &'a mut object::RigidBody<T>,
    //collider_handles: Vec<DefaultColliderHandle>,
}

impl<'a, T: RealField> RigidBody<'a, T>
{
    pub fn new(world: &'a mut PhysWorld<T>, mass: T, collider: Option<&Collider<T>>) -> Self
    {
        let handle = world.body_set.insert(RigidBodyDesc::new().mass(mass).build());

        if let Some(c) = collider
        {
            world.collider_set.insert(c.create_desc().build(BodyPartHandle(handle, 0)));
        }

        RigidBody
        {
            body: world.body_set.rigid_body_mut(handle).unwrap(),
            /*collider_handles: 
                match collider
                {
                    Some(c) => vec![world.collider_set.insert(c.create_desc().build(BodyPartHandle(handle, 0)))],
                    None => Vec::new(),
                },*/
        }
    }

    pub fn get_position(&self) -> Vector2<T>
    {
        self.body.position().translation.vector
    }

    /*pub fn add_collider(&mut self, collider: &Collider<T>)
    {
        let collider = collider.create_desc().build(BodyPartHandle(self.handle, 0));
        self.collider_handles.push(self.world.collider_set.insert(collider));
    }*/
}