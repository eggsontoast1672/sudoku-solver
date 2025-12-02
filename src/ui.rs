//! This module contains tools related to UI widgets.

use raylib::math::Rectangle;

pub trait Widget {
    fn get_rect(&self) -> Rectangle;
}
