#[derive(Debug)]
pub struct States(Vec<State>);

impl States {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn current(&self) -> &State {
        // TODO (Techassi): Handle unwrap
        self.0.last().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut State {
        // TODO (Techassi): Handle unwrap
        self.0.last_mut().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, state: State) {
        self.0.push(state)
    }

    pub fn pop(&mut self) {
        // TODO (Techassi): Handle the unwrap
        self.0.pop().unwrap();
    }
}

// TODO (Techassi): Also handle flow style
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum State {
    #[default]
    Stream,
    Document,
    Sequence,
    Mapping(bool),
}
