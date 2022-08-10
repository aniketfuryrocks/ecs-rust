//! Context own state, readers (observers) and writers (writers).
use std::any::TypeId;

use crate::observer::{Callable, IntoCallable};
use crate::type_reg::TypeReg;

#[derive(Default)]
pub struct Context {
    states: TypeReg,
    entities: TypeReg,
    observers: Vec<Callable>,
}

impl Context {
    #[inline]
    pub fn add_state<State: 'static>(&mut self, state: State) -> &mut Self {
        self.states.add(state);
        self
    }

    #[inline]
    pub fn add_entity<Entity: 'static>(&mut self, entity: Entity) -> &mut Self {
        self.entities.add(entity);
        self
    }

    #[inline]
    pub fn add_observer<Observer: IntoCallable<Args>, Args>(
        &mut self,
        observer: Observer,
    ) -> &mut Self {
        self.observers.push(observer.into_callable());
        self
    }

    #[inline]
    pub fn get_state<State: 'static>(&self) -> Option<&State> {
        self.states.get()
    }

    // TODO: return a wrapper like RefMut which calls notify
    #[inline]
    pub fn get_state_mut<State: 'static>(&mut self) -> Option<&mut State> {
        self.states.get_mut()
    }

    #[inline]
    pub fn get_entity<Entity: 'static>(&self) -> Option<&Entity> {
        self.entities.get()
    }

    #[inline]
    pub fn get_entity_mut<Entity: 'static>(&mut self) -> Option<&mut Entity> {
        self.entities.get_mut()
    }

    pub fn notify<State: 'static>(&mut self) {
        let changed_state_id = TypeId::of::<State>();
        // temp
        for observer in &self.observers {
            observer.call(&mut self.entities, &self.states) 
        }
    }
}
