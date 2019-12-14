use std::cell::RefCell;
use std::rc::{Rc, Weak};

use nalgebra::Vector2;
use nphysics2d::object::{RigidBodyDesc, BodyStatus, DefaultBodyHandle, BodyPartHandle, DefaultBodySet};

use crate::physworld::PhysWorld;
use crate::collider::Collider;
use crate::material::Material;
use crate::number::Number;

pub enum RigidBodyStatus
{
    Static,
    Dynamic,
}

pub struct RigidBodyBuilder<'a, T: Number>
{
    desc: RigidBodyDesc<T>,
    collider: Option<&'a Collider<T>>,
    material: Option<&'a Material<T>>,
}

impl<'a, T: Number> RigidBodyBuilder<'a, T>
{
    pub fn new() -> Self
    {
        RigidBodyBuilder
        {
            desc: RigidBodyDesc::new(),
            collider: None,
            material: None,
        }
    }

    pub fn mass(mut self, mass: T) -> Self
    {
        self.desc.set_mass(mass);
        self
    }

    pub fn make_static(mut self) -> Self
    {
        self.desc.set_status(BodyStatus::Static);
        self
    }

    pub fn collider(mut self, col: &'a Collider<T>, mat: Option<&'a Material<T>>) -> Self
    {
        self.collider = Some(col);
        self.material = mat;
        self
    }

    pub fn build(&self, world: &mut PhysWorld<T>) -> RigidBody<T>
    {
        let handle = world.body_set.borrow_mut().insert(self.desc.build());

        if let Some(c) = self.collider
        {
            world.collider_set.insert(c.create_desc(self.material).build(BodyPartHandle(handle, 0)));
        }

        RigidBody
        {
            body: handle,
            body_set: Rc::downgrade(&world.body_set),
        }
    }
}

pub struct RigidBody<T: Number>
{
    body: DefaultBodyHandle,
    body_set: Weak<RefCell<DefaultBodySet<T>>>,
}

impl<T: Number> RigidBody<T>
{
    /*pub fn new(world: &mut PhysWorld<T>, mass: T, collider: Option<&Collider<T>>, material: Option<&Material<T>>) -> Self // Material and Collider are linked if collider is None Material won't be used
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
    }*/

    pub fn get_position(&self) -> Vector2<T>
    {
        self.body_set.upgrade().unwrap().borrow().rigid_body(self.body).unwrap().position().translation.vector
    }
}