use nalgebra::Vector2;
use nalgebra::geometry::Point2;
use nphysics2d::object::ColliderDesc;
use nphysics2d::material::MaterialHandle;
use ncollide2d::shape;

use crate::material::Material;
use crate::number::Number;

pub enum Collider<T: Number>
{
    Box{ size: Vector2<T>, offset: Option<Point2<T>> },
    Circle{ radius: T, offset: Option<Point2<T>> },
    Capsule{ radius: T, half_height: T },
    //Compound(colliders: Vec<Collider<T>> ),
    Segment{ first_point: Point2<T> , second_point: Point2<T> },
    //Triangle(first_point: Point2<T>, second_point: Point2<T>, third_point: Point2<T>), because shape::Triangle doesn't impl shape trait for no reason
}

impl<T: Number> Collider<T>
{
    pub(crate) fn create_desc(&self, material: Option<&Material<T>>) -> ColliderDesc<T>
    {
        let mut description = match self
        {
            &Collider::Box{ size, offset } =>
            {
                let mut desc = ColliderDesc::new(shape::ShapeHandle::new(shape::Cuboid::new(size.unscale((2.0).into())))); // /2 because Cuboid expect half size
                if let Some(offset) = offset
                {
                    desc.set_translation(offset.coords);
                }
                desc
            }
            &Collider::Circle{ radius, offset } => 
            {
                let mut desc = ColliderDesc::new(shape::ShapeHandle::new(shape::Ball::new(radius)));
                if let Some(offset) = offset
                {
                    desc.set_translation(offset.coords);
                }
                desc
            },
            &Collider::Capsule{ radius, half_height } =>
                ColliderDesc::new(shape::ShapeHandle::new(shape::Capsule::new(half_height, radius))),
            //&Compound(colliders) =>
            &Collider::Segment{ first_point, second_point } =>
                ColliderDesc::new(shape::ShapeHandle::new(shape::Segment::new(first_point, second_point))),
            //&Triangle(first_point, second_point, third_point) =>
            //    ColliderDesc::new(shape::ShapeHandle::new(shape::Triangle::new(first_point, second_point, third_point))),
        };
        if let Some(material) = material
        {
            if let Some(density) = material.density
            {
                description.set_density(density);
            }
            description.set_material(MaterialHandle::new(material.material));
        }
        description
    }
}