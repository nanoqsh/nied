mod color;
mod image;
mod make;
pub mod source;

pub use crate::{
    color::Color,
    image::{Error as ImageError, Image},
    make::make,
};
