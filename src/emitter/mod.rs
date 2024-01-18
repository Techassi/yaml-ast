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
    pub indent_level: usize,
    pub is_root: bool,
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

    pub fn from_events(&mut self, events: Vec<Event>) -> String {
        for event in events {
            match event {
                Event::StreamStart => todo!(),
                Event::StreamEnd => todo!(),
                Event::DocumentStart => todo!(),
                Event::DocumentEnd => todo!(),
                Event::MappingStart => todo!(),
                Event::MappingPairStart => todo!(),
                Event::MappingKeyStart => todo!(),
                Event::MappingKeyEnd => todo!(),
                Event::MappingValueStart => todo!(),
                Event::MappingValueEnd => todo!(),
                Event::MappingPairEnd => todo!(),
                Event::MappingEnd => todo!(),
                Event::SequenceStart => todo!(),
                Event::SequenceItemStart => todo!(),
                Event::SequenceItemEnd => todo!(),
                Event::SequenceEnd => todo!(),
                Event::Scalar(_) => todo!(),
                Event::Comment(_) => todo!(),
            }
        }

        self.writer.clone()
    }

    pub fn emit(&mut self, s: &str) -> Result<(), Error> {
        self.writer.write_str(s)?;
        Ok(())
    }

    pub fn emit_indent(&mut self) -> Result<(), Error> {
        if self.state.indent_level == 0 {
            return Ok(());
        }

        let indent = " ".repeat(self.state.indent_level * self.options.indent_size);
        self.writer.write_str(&indent)?;

        Ok(())
    }

    pub fn emit_newline(&mut self) -> Result<(), Error> {
        self.writer.write_str("\n")?;
        Ok(())
    }

    pub fn increase_indent(&mut self) {
        self.state.indent_level += 1;
    }

    pub fn decrease_indent(&mut self) {
        self.state.indent_level -= 1;
    }
}
