use std::fmt::Write;

use snafu::{ResultExt, Snafu};

use crate::events::Event;

mod iter;
mod options;

pub use iter::*;
pub use options::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to write to output"))]
    Write { source: std::fmt::Error },
}

// TODO (Techassi): Also handle flow style
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum State {
    #[default]
    Initial,
    SequenceItem,
    MappingKey,
    MappingValue,
}

#[derive(Debug)]
pub struct Emitter {
    indent_level: usize,

    // TODO (Techassi): The state(s) need to be managed as a stack to keep track
    // of mapping / sequence recursion depths
    last_state: State,
    state: State,

    options: EmitterOptions,
    events: EventIter,
}

impl Emitter {
    /// Creates a new emitter which will emit characters based on the event
    /// stream using the provided `ident_size`.
    pub fn new(events: Vec<Event>, options: EmitterOptions) -> Self {
        let events = EventIter::new(events);

        Self {
            last_state: State::default(),
            state: State::default(),
            indent_level: 0,
            options,
            events,
        }
    }

    /// Emits a human-friendly YAML character stream to the `writer`.
    pub fn emit(mut self, writer: &mut impl Write) -> Result<(), Error> {
        while let Some(event) = self.events.next() {
            dbg!(&self.state, &self.last_state);
            match event {
                Event::StreamStart => continue,
                Event::StreamEnd => break,
                Event::DocumentStart => self.emit_document_start(writer)?,
                Event::DocumentEnd => self.emit_document_end(writer)?,
                Event::Alias(_) => todo!(),
                Event::Scalar(value) => self.emit_scalar(writer, &value)?,
                Event::SequenceStart(_) => self.emit_sequence_start(),
                Event::SequenceEnd => self.emit_sequence_end(),
                Event::MappingStart(_) => self.emit_mapping_start(writer)?,
                Event::MappingEnd => self.emit_mapping_end(),
            }
        }
        Ok(())
    }

    fn transition_to(&mut self, new_state: State) {
        if new_state != self.state {
            self.last_state = self.state
        }

        self.state = new_state
    }

    fn transition_to_last(&mut self) {
        self.state = self.last_state
    }

    fn emit_indent(&self, writer: &mut impl Write) -> Result<(), Error> {
        writer
            .write_str(
                &" ".repeat(self.options.indent_size)
                    .repeat(self.indent_level),
            )
            .context(WriteSnafu)?;
        Ok(())
    }

    fn emit_document_start(&self, writer: &mut impl Write) -> Result<(), Error> {
        writeln!(writer, "---").context(WriteSnafu)
    }

    fn emit_document_end(&self, writer: &mut impl Write) -> Result<(), Error> {
        writeln!(writer, "...").context(WriteSnafu)
    }

    fn emit_scalar(&mut self, writer: &mut impl Write, value: &str) -> Result<(), Error> {
        match self.state {
            State::Initial => todo!(),
            State::SequenceItem => {
                self.emit_indent(writer)?;
                writeln!(writer, "- {}", value).context(WriteSnafu)?;
            }
            State::MappingKey => {
                if let Some(Event::SequenceStart(_)) = self.events.peek() {
                    writeln!(writer, "{}: ", value).context(WriteSnafu)?;
                } else {
                    write!(writer, "{}: ", value).context(WriteSnafu)?;
                }

                self.transition_to(State::MappingValue);
            }
            State::MappingValue => {
                writeln!(writer, "{}", value).context(WriteSnafu)?;

                if !matches!(self.events.peek(), Some(Event::MappingEnd)) {
                    self.transition_to(State::MappingKey)
                }
            }
        }

        Ok(())
    }

    fn emit_sequence_start(&mut self) {
        self.indent_level += 1;
        self.transition_to(State::SequenceItem)
    }

    fn emit_sequence_end(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }

        self.transition_to_last()
    }

    fn emit_mapping_start(&mut self, writer: &mut impl Write) -> Result<(), Error> {
        if let Some(Event::MappingStart(_)) = self.events.peek() {
            self.indent_level += 1;
            self.emit_indent(writer)?
        }

        self.transition_to(State::MappingKey);
        Ok(())
    }

    fn emit_mapping_end(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }

        self.transition_to_last()
    }
}
