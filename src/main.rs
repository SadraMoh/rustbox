// Stacks

struct StackNode<T> {
    value: T,
    previous: Option<Box<StackNode<T>>>,
}

#[derive(Debug)]
struct Stack<T> {
    members: Vec<T>,
    top: Option<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            members: Vec::<T>::new(),
            top: None,
        }
    }

    pub fn push(&mut self, val: T) {
        self.members.push(val)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.members.pop()
    } 

    pub fn top(&self) -> Option<&T> {
        self.members.last()
    }
}

fn main() {
    let mut books = Stack::<&str>::new();
    books.push("Chemistry");
    books.push("Biology");
    books.push("Literature");

    println!("{:#?}", books);
}
