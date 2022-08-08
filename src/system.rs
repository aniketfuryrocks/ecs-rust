trait System {
    fn execute(self);
}

impl<Func, A> System<(A)> for Func
where 
    Func: Fn(A) -> () + 'static {
    fn execute(self) {
        self.call()
        todo!()
    }
}

