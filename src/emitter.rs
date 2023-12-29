use std::fmt::Write;

use snafu::{ResultExt, Snafu};

use crate::events::Event;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to write to output"))]
    Write { source: std::fmt::Error },
}

#[derive(Debug)]
pub struct Emitter {
    indent_size: usize,
    events: Vec<Event>,

    mapping_level: usize,
    indent_level: usize,
}

impl Default for Emitter {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            mapping_level: 0,
            indent_level: 0,
            indent_size: 2,
        }
    }
}

impl Emitter {
    /// Creates a new emitter which will emit characters based on the event
    /// stream using the provided `ident_size`. If the indentation doesn't need
    /// to customized, use [`Emitter::default()`] to use the default 2 space
    /// indentation.
    pub fn new(events: Vec<Event>, indent_size: usize) -> Self {
        Self {
            indent_size,
            events,
            ..Default::default()
        }
    }

    /// Emits a human-friendly YAML character stream to the `writer`.
    pub fn emit(mut self, writer: &mut impl Write) -> Result<(), Error> {
        let mut iter = self.events.clone().into_iter().peekable();

        while let Some(event) = iter.next() {
            match event {
                Event::StreamStart => return Ok(()),
                Event::StreamEnd => return Ok(()),
                Event::DocumentStart => writeln!(writer, "---").context(WriteSnafu)?,
                Event::DocumentEnd => return Ok(()),
                Event::Alias(_) => todo!(),
                Event::Scalar(val) => write!(writer, "{}", val).context(WriteSnafu)?,
                Event::SequenceStart(_) => todo!(),
                Event::SequenceEnd => todo!(),
                Event::MappingStart(_) => {
                    if let Some(next) = iter.peek() {
                        if let Event::MappingStart(_) = next {
                            self.emit_mapping_start(writer, true)?
                        }
                    }
                }
                Event::MappingEnd => return Ok(()),
            }
        }
        Ok(())
    }

    fn emit_indent(&self, writer: &mut impl Write) -> Result<(), Error> {
        writer
            .write_str(&" ".repeat(self.indent_size).repeat(self.indent_level))
            .context(WriteSnafu)?;
        Ok(())
    }

    fn emit_mapping_start(&mut self, writer: &mut impl Write, indent: bool) -> Result<(), Error> {
        self.mapping_level += 1;
        if indent {
            self.indent_level += 1;
            self.emit_indent(writer)?
        }

        Ok(())
    }

    fn emit_mapping_end(&mut self) {
        self.mapping_level -= 1;
        self.indent_level -= 1;
    }
}
