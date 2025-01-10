use std::io::Write;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct ColorWriter {
    stdout: StandardStream,
}

impl ColorWriter {
    pub fn new() -> Self {
        ColorWriter {
            stdout: StandardStream::stdout(ColorChoice::Auto),
        }
    }

    pub fn writer(&mut self) -> &mut StandardStream {
        &mut self.stdout
    }

    pub fn write_colored(&mut self, text: &str, color: termcolor::Color) -> std::io::Result<()> {
        self.stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(&mut self.stdout, "{}", text)?;
        self.stdout.reset()?;
        Ok(())
    }

    pub fn write_header(&mut self, text: &str) -> std::io::Result<()> {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(termcolor::Color::Cyan))
            .set_bold(true);
        self.stdout.set_color(&spec)?;
        writeln!(&mut self.stdout, "\n=== {} ===", text)?;
        self.stdout.reset()?;
        Ok(())
    }

    pub fn write_error(&mut self, text: &str) -> std::io::Result<()> {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(termcolor::Color::Red))
            .set_bold(true);
        self.stdout.set_color(&spec)?;
        write!(&mut self.stdout, "{}", text)?;
        self.stdout.reset()?;
        Ok(())
    }

    pub fn write_success(&mut self, text: &str) -> std::io::Result<()> {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(termcolor::Color::Green))
            .set_bold(true);
        self.stdout.set_color(&spec)?;
        write!(&mut self.stdout, "{}", text)?;
        self.stdout.reset()?;
        Ok(())
    }

    pub fn write_warning(&mut self, text: &str) -> std::io::Result<()> {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(termcolor::Color::Yellow))
            .set_bold(true);
        self.stdout.set_color(&spec)?;
        write!(&mut self.stdout, "{}", text)?;
        self.stdout.reset()?;
        Ok(())
    }
}
