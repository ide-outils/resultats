use std::{
    collections::HashMap,
    fs,
    ops::Range,
    panic::Location,
    sync::{Arc, LazyLock, RwLock},
};

use crate::ast::{Genre, Node};

type Files = HashMap<String, File>;
static CACHE: LazyLock<Arc<RwLock<Files>>> = LazyLock::new(|| Default::default());

pub fn code_from_location(location: &Location, parent_genre: Option<&Genre>) -> LineSpan {
    let path = location.file();
    let line = location.line() as usize;
    let column = location.column() as usize;
    let read = CACHE.read().unwrap();
    let file = read.get(path);
    let file = match file {
        Some(file) => file,
        None => {
            drop(read);
            let mut write = CACHE.write().unwrap();
            write.insert(path.to_string(), File::from_location(&location));
            drop(write);
            return code_from_location(location, parent_genre);
        }
    };
    // TODO: parent_genre
    file.get_span(line, column)
}

#[derive(Clone)]
pub struct LineSpan {
    pub lines: Range<usize>,
    pub columns: Range<usize>,
    pub parent: usize,
    pub genre: Genre,
    pub code: &'static [u8],
    pub full_lines_code: &'static [u8],
}
impl LineSpan {
    fn new(
        Node { parent, span, genre }: &Node,
        file_code: &'static [u8],
        lines_code: (&[u8], &[u8]),
        full_lines_code: &'static [u8],
    ) -> Self {
        // Span code
        let code = &file_code[span.byte_range()];
        let (start, end) = (span.start(), span.end());
        let lines = (start.line - 1)..(end.line);
        // println!("node : {:?} : {:?} ; {:?}", span, start, end);
        let columns_to_u8 = |column_utf8: usize, line_code: &[u8]| {
            // println!("Code : {}", str::from_utf8(code).unwrap());
            // println!("Line : {}", str::from_utf8(line_code).unwrap());
            // println!("column {}", column_utf8);
            let line = String::from_utf8_lossy(line_code);
            line[..column_utf8].as_bytes().len()
        };
        let columns = columns_to_u8(start.column, lines_code.0)..columns_to_u8(end.column, lines_code.1);
        Self {
            code,
            lines,
            columns,
            parent: parent.clone(),
            genre: genre.clone(),
            full_lines_code,
        }
    }
}

pub struct File {
    code: &'static [u8],
    lines: Vec<Line>,
    // TODO: parents genre
    #[allow(dead_code)]
    spans: Vec<LineSpan>,
}

#[derive(Default)]
struct Line {
    bytes_range: std::ops::Range<usize>,
    spans: Vec<LineSpan>,
}
impl Line {
    fn get_span(&self, column: &usize) -> Option<&LineSpan> {
        self.spans
            .iter()
            .rev()
            .find(|span| span.columns.contains(column))
    }
    fn get_code(&self, code: &'static [u8]) -> &[u8] {
        &code[self.bytes_range.clone()]
    }
}

impl File {
    pub fn new(code: &'static [u8]) -> Self {
        let nodes = crate::ast::parse_nodes(code);
        let mut lines = vec![];
        let mut start = 0;
        for (i, &byte) in code.iter().enumerate() {
            if byte == b'\n' {
                lines.push(Line {
                    bytes_range: start..i,
                    ..Default::default()
                });
                start = i + 1;
            }
        }
        let spans = vec![];
        for node in nodes {
            let start_line = node.span.start().line - 1;
            let end_line = node.span.end().line - 1;
            let lines_str = (lines[start_line].get_code(code), lines[end_line].get_code(code));
            let range = lines[start_line].bytes_range.start..lines[end_line].bytes_range.end;
            let full_lines_code = &code[range];
            let span = LineSpan::new(&node, code, lines_str, full_lines_code);
            lines[start_line].spans.push(span);
            // TODO : handle parent's kind
            // lines[start_line].spans.push(span.clone());
            // spans.push(span);
        }
        Self { code, lines, spans }
    }

    pub fn get_unchecked(&self, line_index: usize) -> &'static [u8] {
        let range = self.lines[line_index].bytes_range.clone();
        &self.code[range]
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static [u8]> {
        (0..self.lines.len()).map(move |i| self.get_unchecked(i))
    }
    fn from_location(location: &Location) -> Self {
        let code = fs::read(location.file()).unwrap();
        Self::new(Vec::leak(code))
    }
    fn get_span(&self, line: usize, column: usize) -> LineSpan {
        let mut line = line;
        let lines = &self.lines;
        let mut span = lines[line].get_span(&column);
        while let None = span {
            if line == 0 {
                panic!("No span found.");
            }
            line -= 1;
            span = lines[line].get_span(&column);
        }
        span.unwrap().clone()
    }
}
