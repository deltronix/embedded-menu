use embedded_graphics::primitives::Rectangle;
use embedded_layout::{object_chain::ChainElement, prelude::*};

use crate::MenuItem;

/// Menu-related extensions for object chain elements
pub trait MenuItemCollection<R>: ChainElement {
    fn bounds_of(&self, nth: u32) -> Rectangle;
    fn title_of(&self, nth: u32) -> &str;
    fn details_of(&self, nth: u32) -> &str;
    fn interact_with(&mut self, nth: u32) -> R;
    fn count(&self) -> usize;
}

impl<I, R> MenuItemCollection<R> for Chain<I>
where
    I: MenuItem<Data = R> + View,
{
    fn bounds_of(&self, nth: u32) -> Rectangle {
        debug_assert!(nth == 0);
        self.object.bounds()
    }

    fn interact_with(&mut self, nth: u32) -> R {
        debug_assert!(nth == 0);
        self.object.interact()
    }

    fn title_of(&self, nth: u32) -> &str {
        debug_assert!(nth == 0);
        self.object.title()
    }

    fn details_of(&self, nth: u32) -> &str {
        debug_assert!(nth == 0);
        self.object.details()
    }

    fn count(&self) -> usize {
        1
    }
}

impl<I, LE, R> MenuItemCollection<R> for Link<I, LE>
where
    I: MenuItem<Data = R> + View,
    LE: MenuItemCollection<R>,
{
    fn bounds_of(&self, nth: u32) -> Rectangle {
        if nth == 0 {
            self.object.bounds()
        } else {
            self.parent.bounds_of(nth - 1)
        }
    }

    fn interact_with(&mut self, nth: u32) -> R {
        if nth == 0 {
            self.object.interact()
        } else {
            self.parent.interact_with(nth - 1)
        }
    }

    fn title_of(&self, nth: u32) -> &str {
        if nth == 0 {
            self.object.title()
        } else {
            self.parent.title_of(nth - 1)
        }
    }

    fn details_of(&self, nth: u32) -> &str {
        if nth == 0 {
            self.object.details()
        } else {
            self.parent.details_of(nth - 1)
        }
    }

    fn count(&self) -> usize {
        1 + self.parent.count()
    }
}
