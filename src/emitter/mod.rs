use std::fmt::Write;

use snafu::Snafu;

mod iter;
mod options;
mod state;

pub use iter::*;
pub use options::*;

use crate::events::Event;

#[derive(Debug, Snafu)]
pub enum Error {
    // TODO (Techassi): Is there any better way to support error propagation
    // outside of this module without importing the (non-public) context
    // selectors.
    #[snafu(display("failed to write to output"), context(false))]
    Write { source: std::fmt::Error },
}

#[derive(Debug, Default)]
pub struct State {
    pub mapping_level: usize,
    pub indent_level: usize,
    pub index: usize,
}

pub struct Emitter {
    options: Options,
    writer: String,
    state: State,
}

impl Emitter {
    pub fn new(options: Options) -> Self {
        Self {
            state: State::default(),
            writer: String::new(),
            options,
        }
    }

    pub fn from_events(&mut self, events: Vec<Event>) -> Result<String, Error> {
        for (index, event) in events.iter().enumerate() {
            self.state.index = index;

            match event {
                Event::StreamStart => continue,
                Event::StreamEnd => continue,
                Event::DocumentStart => self.emit_document_start()?,
                Event::DocumentEnd => self.emit_document_end()?,
                Event::MappingStart => self.emit_mapping_start(&events)?,
                Event::MappingPairStart => continue,
                Event::MappingKeyStart => self.emit_mapping_key_start()?,
                Event::MappingKeyEnd => self.emit_mapping_key_end()?,
                Event::MappingValueStart => continue,
                Event::MappingValueEnd => continue,
                Event::MappingPairEnd => self.emit_newline()?,
                Event::MappingEnd => self.emit_mapping_end(),
                Event::SequenceStart => todo!(),
                Event::SequenceItemStart => todo!(),
                Event::SequenceItemEnd => todo!(),
                Event::SequenceEnd => todo!(),
                Event::Scalar(s) => self.emit_scalar(s)?,
                Event::Comment(c) => self.emit_comment(c)?,
            }
        }

        Ok(self.writer.drain(..self.writer.len()).collect())
    }

    fn next_event<'a>(&'a self, events: &'a [Event]) -> Option<&Event> {
        events.get(self.state.index + 1)
    }

    fn prev_event<'a>(&'a self, events: &'a [Event]) -> Option<&Event> {
        events.get(self.state.index - 1)
    }

    fn increase_indent(&mut self) {
        self.state.indent_level += 1;
    }

    fn decrease_indent(&mut self) {
        self.state.indent_level -= 1;
    }

    fn emit_indent(&mut self) -> Result<(), Error> {
        if self.state.indent_level == 0 {
            return Ok(());
        }

        let indent = " ".repeat(self.state.indent_level * self.options.indent_size);
        self.writer.write_str(&indent)?;

        Ok(())
    }

    fn emit_newline(&mut self) -> Result<(), Error> {
        self.writer.write_str("\n")?;
        Ok(())
    }

    fn emit_document_start(&mut self) -> Result<(), Error> {
        self.writer.write_str("---")?;
        self.emit_newline()?;
        Ok(())
    }

    fn emit_document_end(&mut self) -> Result<(), Error> {
        self.writer.write_str("...")?;
        self.emit_newline()?;
        Ok(())
    }

    fn emit_mapping_start<'a>(&'a mut self, events: &'a [Event]) -> Result<(), Error> {
        if self.state.mapping_level == 0 {
            self.state.mapping_level += 1;
            return Ok(());
        }

        self.state.mapping_level += 1;
        self.emit_indent()?;
        self.increase_indent();
        Ok(())
    }

    fn emit_mapping_end(&mut self) {
        self.state.mapping_level -= 1;

        if self.state.indent_level > 0 {
            self.decrease_indent()
        }
    }

    fn emit_mapping_key_start(&mut self) -> Result<(), Error> {
        self.emit_indent()?;
        Ok(())
    }

    fn emit_mapping_key_end(&mut self) -> Result<(), Error> {
        self.writer.write_str(": ")?;
        Ok(())
    }

    fn emit_scalar(&mut self, scalar: &String) -> Result<(), Error> {
        self.writer.write_str(scalar)?;
        Ok(())
    }

    fn emit_comment(&mut self, comment: &String) -> Result<(), Error> {
        self.emit_indent()?;
        write!(self.writer, "# {}", comment)?;
        self.emit_newline()?;
        Ok(())
    }
}
