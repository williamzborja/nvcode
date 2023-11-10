use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::HighlightEvent;
use tree_sitter_highlight::Highlighter;
use tree_sitter_rust::{language, HIGHLIGHT_QUERY, INJECTIONS_QUERY};

fn main() {
    let mut highlighter = Highlighter::new();

    let highlight_names = [
        "attribute",
        "constant",
        "function.builtin",
        "function",
        "keyword",
        "operator",
        "property",
        "punctuation",
        "punctuation.bracket",
        "punctuation.delimiter",
        "string",
        "string.special",
        "tag",
        "type",
        "type.builtin",
        "variable",
        "variable.builtin",
        "variable.parameter",
    ];

    let mut config =
        HighlightConfiguration::new(language(), HIGHLIGHT_QUERY, INJECTIONS_QUERY, "").unwrap();

    config.configure(&highlight_names);

    let highlights = highlighter
        .highlight(&config, b"let x: i32 = 8;", None, |_| None)
        .unwrap();

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                eprintln!("source: {}-{}", start, end);
            }
            HighlightEvent::HighlightStart(s) => {
                eprintln!(
                    "highlight style started: {:?} {}",
                    s,
                    highlight_names.get(s.0).unwrap()
                );
            }
            HighlightEvent::HighlightEnd => {
                eprintln!("highlight style ended");
            }
        }
    }
}

