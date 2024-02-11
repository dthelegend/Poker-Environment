use std::collections::VecDeque;

#[derive(Clone)]
pub struct Playlist<T>(pub VecDeque<T>, pub VecDeque<T>, pub Vec<T>);

impl <T> Playlist<T> {

    pub fn new(input: Vec<T>) -> Self {
        let input_len = input.len();
        Playlist(
            VecDeque::with_capacity(input_len),
            input.into(),
            Vec::with_capacity(input_len)
        )
    }

    pub fn next<F>(&mut self, mut f: F) -> bool where F: FnMut(&mut T) -> bool {
        let mut next_item = self.1.pop_front()
            .expect("There should always be a next item");
        let should_keep = f(&mut next_item);

        if should_keep {
            self.0.push_back(next_item);
        }
        else {
            self.2.push(next_item);
        }

        should_keep
    }

    pub fn restart(&mut self) {
        self.1.append(&mut self.0);
    }

    pub fn is_finished(&self) -> bool {
        self.1.len() == 0
    }

    pub fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    pub fn complete_len(&self) -> usize {
        self.0.len() + self.1.len() + self.2.len()
    }

    pub fn into_lists(mut self) -> (Vec<T>, Vec<T>) {
        self.restart();

        (self.1.into(), self.2)
    }
}

impl <'a, T> Playlist<T> {
    pub fn peek_next(&'a self) -> &'a T {
        self.1.front().expect("Playlist invariant requires there to be a next player")
    }
}
