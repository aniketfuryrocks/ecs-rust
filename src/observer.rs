use std::any::TypeId;

pub trait Observer<Args, const ARGS_LEN: usize> {
    fn get_args() -> [TypeId; ARGS_LEN] {
        todo!()
    }

    fn call(&self, args: Args);
}

impl<Func, A, B> Observer<(A, B), 2usize> for Func
where
    Func: Fn(A, B) + 'static,
{
    //    fn get_args() -> [TypeId; 2usize] {
    //        [TypeId::of::<A>(), TypeId::of::<B>()]
    //    }

    fn call(&self, (A, B): (A, B)) {
        (self)(A, B)
    }
}
