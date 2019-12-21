use nphysics2d::material::BasicMaterial;

use crate::number::Number;

pub struct Material<T: Number>
{
	pub(crate) material: BasicMaterial<T>,
	pub(crate) density: Option<T>,
}

impl<T: Number> Material<T>
{
	pub fn new(restitution: T, friction: T, density: Option<T>) -> Self
	{
		Material
		{
			material: BasicMaterial::new(restitution, friction),
			density,
		}
	}
}