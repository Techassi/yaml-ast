/// These options control the emitter behavior.
///
/// It provides a builder to selectively customize individual settings. If no
/// customization is required, use [`EmitterOptions::default()`].
#[derive(Debug)]
pub struct Options {
    pub indent_size: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Options {
    pub fn builder() -> EmitterOptionsBuilder {
        EmitterOptionsBuilder::default()
    }
}

pub struct EmitterOptionsBuilder {
    indent_size: usize,
}

impl Default for EmitterOptionsBuilder {
    fn default() -> Self {
        Self { indent_size: 2 }
    }
}

impl EmitterOptionsBuilder {
    pub fn build(self) -> Options {
        Options {
            indent_size: self.indent_size,
        }
    }
}
