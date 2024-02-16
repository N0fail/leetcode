use super::Solver;

#[derive(Debug)]
enum Command {
    Push(i32),
    Pop,
    Peek,
    Empty,
}

#[derive(Debug)]
enum CommandResult {
    Push,
    Pop(i32),
    Peek(i32),
    Empty(bool),
}

#[derive(Debug)]
pub struct Solution {
    input: Vec<Command>,
    solution: Vec<CommandResult>,
}

struct MyQueue {
    read_stack: Vec<i32>,
    write_stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            read_stack: vec![],
            write_stack: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.write_stack.push(x);
    }

    fn transfer_if_needed(&mut self) {
        if self.read_stack.is_empty() {
            loop {
                let x = self.write_stack.pop();
                match x {
                    None => break,
                    Some(el) => self.read_stack.push(el),
                }
            }
        }
    }

    fn pop(&mut self) -> i32 {
        self.transfer_if_needed();
        return self.read_stack.pop().unwrap();
    }

    fn peek(&mut self) -> i32 {
        self.transfer_if_needed();
        return *self.read_stack.last().unwrap();
    }

    fn empty(&self) -> bool {
        return self.read_stack.is_empty() && self.write_stack.is_empty();
    }
}

impl Solver for Solution {
    fn read_inputs() -> Self {
        return Solution {
            input: vec![
                Command::Push(1),
                Command::Push(2),
                Command::Peek,
                Command::Pop,
                Command::Empty,
            ],
            solution: vec![],
        };
    }

    fn solve(mut self) {
        let mut queue = MyQueue::new();
        for command in self.input {
            match command {
                Command::Push(elem) => {
                    queue.push(elem);
                    self.solution.push(CommandResult::Push)
                }
                Command::Pop => self.solution.push(CommandResult::Pop(queue.pop())),
                Command::Peek => self.solution.push(CommandResult::Peek(queue.peek())),
                Command::Empty => self.solution.push(CommandResult::Empty(queue.empty())),
            }
        }

        dbg!(self.solution);
    }
}
