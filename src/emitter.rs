use std::fmt::Write;

use snafu::{ResultExt, Snafu};

use crate::events::Event;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to write to output"))]
    Write { source: std::fmt::Error },
}

#[derive(Debug)]
pub struct EmitterState {
    indent_level: usize,
    indent_size: usize,
    state: State,
}

// TODO (Techassi): Also handle flow style
#[derive(Debug, Default)]
pub enum State {
    #[default]
    Initial,
    SequenceItem,
    MappingKey,
    MappingValue,
}

#[derive(Debug)]
pub struct Emitter {
    state: EmitterState,
    events: EventIter,
}

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

impl Emitter {
    /// Creates a new emitter which will emit characters based on the event
    /// stream using the provided `ident_size`. If the indentation doesn't need
    /// to customized, use [`Emitter::default()`] to use the default 2 space
    /// indentation.
    pub fn new(events: Vec<Event>, indent_size: usize) -> Self {
        let state = EmitterState {
            state: State::default(),
            indent_level: 0,
            indent_size,
        };

        let events = EventIter::new(events);

        Self { events, state }
    }

    /// Emits a human-friendly YAML character stream to the `writer`.
    pub fn emit(mut self, writer: &mut impl Write) -> Result<(), Error> {
        while let Some(event) = self.events.next() {
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

    fn emit_indent(&self, writer: &mut impl Write) -> Result<(), Error> {
        writer
            .write_str(
                &" ".repeat(self.state.indent_size)
                    .repeat(self.state.indent_level),
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
        match self.state.state {
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

                self.state.state = State::MappingValue;
            }
            State::MappingValue => {
                writeln!(writer, "{}", value).context(WriteSnafu)?;

                if !matches!(self.events.peek(), Some(Event::MappingEnd)) {
                    self.state.state = State::MappingKey;
                }
            }
        }

        Ok(())
    }

    fn emit_sequence_start(&mut self) {
        self.state.indent_level += 1;
        self.state.state = State::SequenceItem;
    }

    fn emit_sequence_end(&mut self) {
        if self.state.indent_level > 0 {
            self.state.indent_level -= 1;
        }

        self.state.state = State::Initial;
    }

    fn emit_mapping_start(&mut self, writer: &mut impl Write) -> Result<(), Error> {
        if let Some(Event::MappingStart(_)) = self.events.peek() {
            self.state.indent_level += 1;
            self.emit_indent(writer)?
        }

        self.state.state = State::MappingKey;
        Ok(())
    }

    fn emit_mapping_end(&mut self) {
        if self.state.indent_level > 0 {
            self.state.indent_level -= 1;
        }
    }
}
