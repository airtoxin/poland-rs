fn parser(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

struct Stack {
    v: Vec<f32>,
}

impl Stack {
    fn new() -> Stack {
        Stack { v: vec![] }
    }

    fn push(&mut self, s: f32) {
        self.v.push(s);
    }

    fn pop(&mut self) -> f32 {
        self.v.pop().unwrap()
    }
}

fn is_number(n: &str) -> bool {
    match n.parse::<f32>() {
        Ok(_) => true,
        _ => false,
    }
}

fn noop() {}

fn main() {
    let mut stack = Stack::new();

    for token in parser("3 5 5 + print - print") {
        match token {
            "+" => {
                let v = stack.pop() + stack.pop();
                stack.push(v);
            }
            "-" => {
                let v = stack.pop() - stack.pop();
                stack.push(v);
            }
            "print" => {
                println!("{:?}", stack.v);
            }
            n if is_number(n) => match n.parse::<f32>() {
                Ok(f) => stack.push(f),
                _ => noop(),
            },
            a => println!("Error {:?}", a),
        }
    }
}
