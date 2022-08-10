pub mod context;
pub mod observer;
pub mod type_reg;

#[cfg(test)]
mod test {
    use crate::context::Context;

    #[derive(Default)]
    struct Node {
        pub(crate) counter: i32,
    }

    impl Node {
        fn render(&self) {
            println!("{}", self.counter);
            // clear screen
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
    }

    #[test]
    fn counter() {
        let mut context = Context::default();

        context
            .add_state(0) // counter
            .add_entity(Node::default()) // constant entity which can be mutated but not observed
            .add_observer(|node: &mut Node, counter: &i32| {
                node.counter = *counter;
                node.render();
            });

        {
            // producer logic
            let mut buf = String::new();
            let stdin = std::io::stdin();

            loop {
                // increment counter on each enter
                stdin.read_line(&mut buf).unwrap();
                *context.get_state_mut::<i32>().unwrap() += 1;

                context.notify::<i32>();
            }
        }
    }
}
