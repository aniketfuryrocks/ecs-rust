use context::Context;

use self::observer::Observer;

mod context;
mod observer;
mod type_reg;

fn main() {
    let s = String::from("sdfg");
    take(a, (&s, &s));
    counter();
}

fn a(name: &String, name2: &String) {}

fn take<Obs: Observer<Args, ARGS_LEN>, Args, const ARGS_LEN: usize>(f: Obs, args: Args) {
    f.call(args)
}

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

            // not required
            context.get_entity_mut::<Node>().unwrap().counter =
                context.get_state::<i32>().unwrap().to_owned();
            context.get_entity::<Node>().unwrap().render();
        }
    }
}
