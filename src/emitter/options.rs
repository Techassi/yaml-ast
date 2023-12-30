/// These options control the emitter behavior.
///
/// It provides a builder to selectively customize individual settings. If no
/// customization is required, use [`EmitterOptions::default()`].
#[derive(Debug)]
pub struct EmitterOptions {
    pub indent_size: usize,
}

impl Default for EmitterOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl EmitterOptions {
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
    pub fn build(self) -> EmitterOptions {
        EmitterOptions {
            indent_size: self.indent_size,
        }
    }
}
