use crate::events::Event;

#[derive(Debug)]
pub struct EventIter {
    events: Vec<Event>,
    index: usize,
}

impl EventIter {
    pub fn new(events: Vec<Event>) -> Self {
        Self { events, index: 0 }
    }

    pub fn next(&mut self) -> Option<Event> {
        let event = self.events.get(self.index);
        self.index += 1;
        event.cloned()
    }

    pub fn peek(&self) -> Option<&Event> {
        self.events.get(self.index)
    }

    pub fn peek_as(&self, ty: Event) -> Option<&Event> {
        match self.events.get(self.index) {
            Some(e) if e == &ty => Some(e),
            _ => None,
        }
    }
}
