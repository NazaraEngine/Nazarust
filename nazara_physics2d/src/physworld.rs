use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::Vector2;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};

use crate::Number;

pub struct PhysWorld<T: Number>
{
    mechanical_world: DefaultMechanicalWorld<T>,
    geometric_world: DefaultGeometricalWorld<T>,
    pub(crate) body_set: Rc<RefCell<DefaultBodySet<T>>>,
    pub(crate) collider_set: DefaultColliderSet<T>,
    joint_set: DefaultJointConstraintSet<T>,
    force_generator_set: DefaultForceGeneratorSet<T>,
}

impl<T: Number> PhysWorld<T>
{
    pub fn new(gravity: Vector2<T>) -> PhysWorld<T>
    {
        PhysWorld
        {
            mechanical_world: DefaultMechanicalWorld::new(gravity),
            geometric_world: DefaultGeometricalWorld::new(),
            body_set: Rc::new(RefCell::new(DefaultBodySet::new())),
            collider_set: DefaultColliderSet::new(),
            joint_set: DefaultJointConstraintSet::new(),
            force_generator_set: DefaultForceGeneratorSet::new(),
        }
    }

    pub fn step(&mut self, timestep: T)
    {
        self.mechanical_world.set_timestep(timestep);
        self.mechanical_world.step(&mut self.geometric_world, &mut (*self.body_set.borrow_mut()), &mut self.collider_set, &mut self.joint_set, &mut self.force_generator_set);
    }
}