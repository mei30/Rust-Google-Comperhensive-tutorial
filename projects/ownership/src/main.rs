trait State {
    fn x(self: Box<Self>) -> Box<dyn State>;
}

#[derive(Debug)]
struct Draft {
    s: String,
}

impl State for Draft {
    fn x(self: Box<Self>)-> Box<dyn State> {
        println!("Drft: {:?}", self);
        Box::new(Draft {s: String::from("HI")})

    }
}

struct Post {
    state: Box<dyn State>
}

impl Post {
    fn request(&mut self) {
        self.state = self.state.x();
    }
}

fn main() {
    let z = Box::new(Draft {s: String::from("Hello")});
    let m = z.x();
}
