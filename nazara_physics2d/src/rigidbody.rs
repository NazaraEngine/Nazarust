use std::cell::RefCell;
use std::rc::{Rc, Weak};

use nalgebra::{Vector2, Point2};
use nphysics2d::object::{RigidBodyDesc, BodyStatus, DefaultBodyHandle, BodyPartHandle, DefaultColliderHandle, DefaultBodySet};
use nphysics2d::object;
use nphysics2d::math::{Velocity, Inertia};

use crate::physworld::PhysWorld;
use crate::collider::Collider;
use crate::Number;

pub enum RigidBodyStatus
{
    Static,
    Dynamic,
}

pub struct RigidBody<T: Number>
{
    body: DefaultBodyHandle,
    body_set: Weak<RefCell<DefaultBodySet<T>>>,
}

impl<T: Number> RigidBody<T>
{
    pub fn new(world: &mut PhysWorld<T>, mass: T, status: RigidBodyStatus, collider: Option<&Collider<T>>) -> Self
    {
        let mut desc = RigidBodyDesc::new().mass(mass);

        if let RigidBodyStatus::Static = status
        {
            desc.set_status(BodyStatus::Static);
        }

        let handle = world.body_set.borrow_mut().insert(desc.build());

        if let Some(c) = collider
        {
            world.collider_set.insert(c.create_desc().build(BodyPartHandle(handle, 0)));
        }

        RigidBody
        {
            body: handle,
            body_set: Rc::downgrade(&world.body_set),
        }
    }

    pub fn get_position(&self) -> Vector2<T>
    {
        self.body_set.upgrade().unwrap().borrow().rigid_body(self.body).unwrap().position().translation.vector
    }
}