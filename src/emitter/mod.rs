use std::fmt::Write;

use snafu::{ResultExt, Snafu};

use crate::{
    emitter::state::{State, States},
    events::Event,
};

mod iter;
mod options;
mod state;

pub use iter::*;
pub use options::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to write to output"))]
    Write { source: std::fmt::Error },
}

#[derive(Debug)]
pub struct Emitter {
    indent_level: usize,

    // TODO (Techassi): The state(s) need to be managed as a stack to keep track
    // of mapping / sequence recursion depths
    states: States,

    options: EmitterOptions,
    events: EventIter,
}

impl Emitter {
    /// Creates a new emitter which will emit characters based on the event
    /// stream using the provided `ident_size`.
    pub fn new(events: Vec<Event>, options: EmitterOptions) -> Self {
        let events = EventIter::new(events);

        Self {
            states: States::new(),
            indent_level: 0,
            options,
            events,
        }
    }

    /// Emits a human-friendly YAML character stream to the `writer`.
    pub fn emit(mut self, writer: &mut impl Write) -> Result<(), Error> {
        while let Some(event) = self.events.next() {
            match event {
                Event::StreamStart => self.states.push(State::Stream),
                Event::StreamEnd => self.states.pop(),
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

        assert!(self.states.is_empty());
        Ok(())
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

    fn emit_document_start(&mut self, writer: &mut impl Write) -> Result<(), Error> {
        writeln!(writer, "---").context(WriteSnafu)?;
        self.states.push(State::Document);
        Ok(())
    }

    fn emit_document_end(&mut self, writer: &mut impl Write) -> Result<(), Error> {
        writeln!(writer, "...").context(WriteSnafu)?;
        self.states.pop();
        Ok(())
    }

    fn emit_scalar(&mut self, writer: &mut impl Write, value: &str) -> Result<(), Error> {
        match self.states.current_mut() {
            State::Stream => todo!(),
            State::Document => todo!(),
            State::Sequence => self.emit_sequence_item(writer, value)?,
            State::Mapping(is_key) => {
                if *is_key {
                    *is_key = false;
                    self.emit_mapping_key(writer, value)?;
                } else {
                    *is_key = true;
                    self.emit_mapping_value(writer, value)?
                }
            }
        }

        Ok(())
    }

    fn emit_sequence_item(&self, writer: &mut impl Write, value: &str) -> Result<(), Error> {
        self.emit_indent(writer)?;
        writeln!(writer, "- {}", value).context(WriteSnafu)
    }

    fn emit_mapping_key(&self, writer: &mut impl Write, value: &str) -> Result<(), Error> {
        if let Some(Event::SequenceStart(_)) = self.events.peek() {
            writeln!(writer, "{}: ", value).context(WriteSnafu)
        } else {
            write!(writer, "{}: ", value).context(WriteSnafu)
        }
    }

    fn emit_mapping_value(&self, writer: &mut impl Write, value: &str) -> Result<(), Error> {
        writeln!(writer, "{}", value).context(WriteSnafu)
    }

    fn emit_sequence_start(&mut self) {
        self.indent_level += 1;
        self.states.push(State::Sequence)
    }

    fn emit_sequence_end(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }

        self.states.pop()
    }

    fn emit_mapping_start(&mut self, writer: &mut impl Write) -> Result<(), Error> {
        if let Some(Event::MappingStart(_)) = self.events.peek() {
            self.indent_level += 1;
            self.emit_indent(writer)?
        }

        self.states.push(State::Mapping(true));
        Ok(())
    }

    fn emit_mapping_end(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }

        // TODO (Techassi): Assert that the popped state is the state we expected
        self.states.pop()
    }
}
