use nalgebra::{RealField, Vector2};
use nalgebra::geometry::Point2;
use nphysics2d::object::ColliderDesc;
use ncollide2d::shape;

pub enum Collider<T: RealField>
{
    //Box(Vector2<T> /*size*/, Option<Point2<T>> /*offset*/), // TODO: understand cuboïd collider
    Circle(T /*radius*/, Option<Point2<T>> /*offset*/),
    Capsule(T /*half_height*/, T /*radius*/),
    //Compound(Vec<Collider<T>> /*colliders*/),
    Segment(Point2<T> /*first_point*/, Point2<T> /*second_point*/),
    //Triangle(Point2<T> /*first_point*/, Point2<T> /*second_point*/, Point2<T> /*third_point*/), because shape::Triangle doesn't impl shape trait
}

impl<T: RealField> Collider<T>
{
    pub(crate) fn create_desc(&self) -> ColliderDesc<T>
    {
        match self
        {
            &Collider::Circle(radius, offset) => 
            {
                let mut desc = ColliderDesc::new(shape::ShapeHandle::new(shape::Ball::new(radius)));
                if let Some(offset) = offset
                {
                    desc.set_translation(offset.coords);
                }
                desc
            },
            &Collider::Capsule(half_height, radius) =>
                ColliderDesc::new(shape::ShapeHandle::new(shape::Capsule::new(half_height, radius))),
            //&Compound(colliders) =>
            &Collider::Segment(first_point, second_point) =>
                ColliderDesc::new(shape::ShapeHandle::new(shape::Segment::new(first_point, second_point))),
            //&Triangle(first_point, second_point, third_point) =>
            //    ColliderDesc::new(shape::ShapeHandle::new(shape::Triangle::new(first_point, second_point, third_point))),
        }
    }
}