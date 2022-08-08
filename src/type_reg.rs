//pub type Observers = Vec<Observer>;
//
//pub trait Observer {}
//
//impl<Func, A> Observer for Func
//    where Func: Fn(A) -> () + 'static {
//}

use hashbrown::HashMap;
use std::any::{Any, TypeId};

/// Type Registry
#[derive(Default)]
pub struct TypeReg {
    reg: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeReg {
    #[inline]
    pub fn add<Item: 'static>(&mut self, item: Item) -> Option<Item> {
        let old_item = self.reg.insert(item.type_id(), Box::new(item));

        if let Some(old_item) = old_item {
            return Some(*old_item.downcast().unwrap());
        }

        None
    }

    pub fn get<Item: 'static>(&self) -> Option<&Item> {
        if let Some(item_any) = self.reg.get(&TypeId::of::<Item>()) {
            return Some(item_any.downcast_ref().unwrap());
        }
        None
    }

    pub fn get_mut<Item: 'static>(&mut self) -> Option<&mut Item> {
        if let Some(item_any) = self.reg.get_mut(&TypeId::of::<Item>()) {
            return Some(item_any.downcast_mut().unwrap());
        }
        None
    }
}
