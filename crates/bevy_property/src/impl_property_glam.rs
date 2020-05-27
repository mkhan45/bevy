use crate::{impl_property, Property, Serializable};
use glam::{Mat3, Mat4, Quat, Vec2, Vec3};

impl_property!(Vec2);
impl_property!(Vec3);
impl_property!(Mat3);
impl_property!(Mat4);
impl_property!(Quat);
