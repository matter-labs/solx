//!
//! A simple pretty printer that outputs text via a type implementing [`Write`]
//!

use anyhow::Result;

use super::IPrinter;

///
/// State of the printer
///
pub struct WritePrinter<W: std::fmt::Write> {
    indent: u32,
    line_start: bool,
    writer: W,
}

impl<W: std::fmt::Write> WritePrinter<W> {
    const INDENT_CHARACTER: &'static str = "  ";

    ///
    /// Creates a new [`Printer`].
    ///
    pub fn new(writer: W) -> WritePrinter<W> {
        WritePrinter {
            indent: 0,
            line_start: true,
            writer,
        }
    }

    fn indent_reset(&mut self) {
        self.line_start = true;
    }

    fn indent(&mut self) -> Result<()> {
        if self.line_start {
            for _ in 0..self.indent {
                write!(&mut self.writer, "{}", Self::INDENT_CHARACTER)?;
            }
            self.line_start = false;
        };
        Ok(())
    }
}

impl<W: std::fmt::Write> IPrinter for WritePrinter<W> {
    fn print(&mut self, s: &str) -> Result<()> {
        self.indent()?;
        write!(&mut self.writer, "{}", s)?;
        Ok(())
    }

    fn println(&mut self, s: &str) -> Result<()> {
        self.print(s)?;
        self.print("\n")?;
        self.indent_reset();
        Ok(())
    }

    fn increase_indent(&mut self) -> Result<()> {
        self.indent += 1;
        Ok(())
    }

    fn decrease_indent(&mut self) -> Result<()> {
        if self.indent > 0 {
            self.indent -= 1;
            Ok(())
        } else {
            anyhow::bail!(
                "Internal error: Attempted to decrease the printing indentation below zero."
            )
        }
    }
}
