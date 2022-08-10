use std::any::{self, Any, TypeId};

use crate::type_reg::TypeReg;

pub struct Callable {
    /// takes ent&&ities, states
    run_fn: Box<dyn Fn(&mut TypeReg, &TypeReg)>,
    args: Vec<TypeId>,
}

impl Callable {
    pub fn call(&self, entities: &mut TypeReg, states: &TypeReg) {
        (*self.run_fn)(entities, states)
    }

    pub fn get_args(&self) -> &Vec<TypeId> {
        &self.args
    }
}

pub trait IntoCallable<Args> {
    fn into_callable(self) -> Callable;
}

impl<Func, A, B> IntoCallable<(A, B)> for Func
where
    Func: Fn(&mut A, &B) + 'static,
    A: Any,
    B: Any,
{
    fn into_callable(self) -> Callable {
        Callable {
            run_fn: Box::new(move |entities: &mut TypeReg, states: &TypeReg| {
                (self)(
                    entities.get_mut::<A>().unwrap_or_else(|| {
                        panic!("no entity of type {} found", any::type_name::<A>())
                    }),
                    states.get::<B>().unwrap_or_else(|| {
                        panic!("no state of type {} found", any::type_name::<B>())
                    }),
                )
            }),
            args: vec![TypeId::of::<A>()],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::type_reg::TypeReg;

    use super::IntoCallable;

    fn sample_callable(name: &mut i32, name2: &String) {
        println!("{name} {name2}")
    }

    #[test]
    fn test() {
        let mut state_reg = TypeReg::default();
        state_reg.add(String::from("other"));
        state_reg.add(0);

        let mut entity_reg = TypeReg::default();
        entity_reg.add(1);

        take(sample_callable, &mut entity_reg, &state_reg);
    }

    fn take<Obs: IntoCallable<Args>, Args>(f: Obs, entity_reg: &mut TypeReg, reg: &TypeReg) {
        let callable = f.into_callable();
        eprintln!("{:?}", callable.get_args());
        callable.call(entity_reg, reg)
    }
}
