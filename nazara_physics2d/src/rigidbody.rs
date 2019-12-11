use std::cell::RefCell;
use std::rc::{Rc, Weak};

use nalgebra::Vector2;
use nphysics2d::object::{RigidBodyDesc, BodyStatus, DefaultBodyHandle, BodyPartHandle, DefaultBodySet};

use crate::physworld::PhysWorld;
use crate::collider::Collider;
use crate::material::Material;
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
    pub fn new(world: &mut PhysWorld<T>, mass: T, collider: Option<&Collider<T>>, material: Option<&Material<T>>) -> Self // Material and Collider are linked if collider is None Material won't be used
    {
        let handle = world.body_set.borrow_mut().insert(RigidBodyDesc::new().mass(mass).build());

        if let Some(c) = collider
        {
            world.collider_set.insert(c.create_desc(material).build(BodyPartHandle(handle, 0))); // if Material::density is not None and not zero mass and angular-inertia will be overriden
        }

        RigidBody
        {
            body: handle,
            body_set: Rc::downgrade(&world.body_set),
        }
    }

    pub fn new_static(world: &mut PhysWorld<T>, collider: Option<&Collider<T>>, material: Option<&Material<T>>) -> Self
    {
        let handle = world.body_set.borrow_mut().insert(RigidBodyDesc::new().status(BodyStatus::Static).build());

        if let Some(c) = collider
        {
            world.collider_set.insert(c.create_desc(material).build(BodyPartHandle(handle, 0)));
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