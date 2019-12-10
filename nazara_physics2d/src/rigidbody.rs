use std::cell::RefCell;
use std::rc::{Rc, Weak};

use nalgebra::{Vector2, Point2, RealField};
use nphysics2d::object::{RigidBodyDesc, ColliderDesc, DefaultBodyHandle, BodyPartHandle, DefaultColliderHandle, DefaultBodySet};
use nphysics2d::object;
use nphysics2d::math::{Velocity, Inertia};

use crate::physworld::PhysWorld;
use crate::collider::Collider;

pub struct RigidBody<T: RealField>
{
    body: DefaultBodyHandle,
    body_set: Weak<RefCell<DefaultBodySet<T>>>,
    //collider_handles: Vec<DefaultColliderHandle>,
}

impl<T: RealField> RigidBody<T>
{
    pub fn new(world: &mut PhysWorld<T>, mass: T, collider: Option<&Collider<T>>) -> Self
    {
        let handle = world.body_set.borrow_mut().insert(RigidBodyDesc::new().mass(mass).build());

        if let Some(c) = collider
        {
            world.collider_set.insert(c.create_desc().build(BodyPartHandle(handle, 0)));
        }

        RigidBody
        {
            body: handle,
            body_set: Rc::downgrade(&world.body_set),
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
        self.body_set.upgrade().unwrap().borrow().rigid_body(self.body).unwrap().position().translation.vector
    }

    /*pub fn add_collider(&mut self, collider: &Collider<T>)
    {
        let collider = collider.create_desc().build(BodyPartHandle(self.handle, 0));
        self.collider_handles.push(self.world.collider_set.insert(collider));
    }*/
}