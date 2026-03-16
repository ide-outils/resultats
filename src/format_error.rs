use std::io::{self, Write};

use crate::Error;

#[derive(Debug, Default)]
pub struct SimpleReportHandler {
    with_cause_chain: bool,
    footer: Option<String>,
}

impl SimpleReportHandler {
    pub const fn new() -> Self {
        Self {
            footer: None,
            with_cause_chain: true,
        }
    }

    pub const fn with_cause_chain(mut self, enable: bool) -> Self {
        self.with_cause_chain = enable;
        self
    }

    pub fn with_footer(mut self, footer: String) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn render_error(&self, f: &mut impl Write, error: &Error) -> io::Result<()> {
        self.render_header(f, error)?;
        if self.with_cause_chain {
            self.render_causes(f, error)?;
        }
        self.render_content(f, error)?;
        self.render_labels(f, error)?;
        self.render_footer(f, error)?;
        if let Some(footer) = &self.footer {
            writeln!(f, "{}", footer)?;
        }
        Ok(())
    }

    fn render_header(&self, f: &mut impl Write, error: &Error) -> io::Result<()> {
        writeln!(f, "{}[{}]: {}", error.severity, error.code, error.display)?;
        writeln!(
            f,
            "  --> {}:{}",
            error.loc.location().file(),
            error.loc.location().line()
        )?;
        Ok(())
    }

    fn render_causes(&self, f: &mut impl Write, error: &Error) -> io::Result<()> {
        let mut current = error;
        while let Some(source) = &current.source {
            writeln!(f, "Caused by: {}", source.display)?;
            current = source;
        }
        Ok(())
    }

    fn render_content(&self, f: &mut impl Write, error: &Error) -> io::Result<()> {
        let span = error.loc.related_code();
        if span.lines.start + 1 == span.lines.end {
            // single line
            f.write_all(span.full_lines_code)?;
            f.write_all(b"\n")?;
            let start = span.columns.start;
            let show_related: String = (0..span.columns.end)
                .map(|i| if i < start { ' ' } else { '^' })
                .collect();
            writeln!(f, "{show_related}")?;
        } else {
            // multi lines
            f.write_all(span.full_lines_code)?;
            f.write_all(b"\n")?;
        }
        Ok(())
    }

    fn render_labels(&self, f: &mut impl Write, error: &Error) -> io::Result<()> {
        if !error.labels.is_empty() {
            writeln!(f, "Labels:")?;
            for label in &error.labels {
                writeln!(
                    f,
                    "  --> {}:{}: {}",
                    label.loc.location().file(),
                    label.loc.location().line(),
                    label.label
                )?;
            }
        }
        Ok(())
    }

    fn render_footer(&self, f: &mut impl Write, error: &Error) -> io::Result<()> {
        if let Some(group) = &error.groupe {
            writeln!(f, "Related errors:")?;
            writeln!(f, "{:?}", group.errors)?;
        }
        if let Some(url) = error.url.as_ref() {
            writeln!(f, "For more details, see: {}", url)?;
        }
        Ok(())
    }
}
