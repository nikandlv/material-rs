//! Syntax highlighting powered by syntect.
//!
//! Provides token-based highlighting for Rust, JavaScript, CSS, HTML,
//! and plain text using syntect's TextMate grammar definitions.
//! Maps base16-ocean.dark theme colors to CSS classes.

use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

/// A syntax token with its text content and CSS class.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub text: String,
    pub class: String,
}

/// Supported language for syntax highlighting.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    Rust,
    JavaScript,
    Css,
    Html,
    Plain,
}

impl Language {
    fn syntax_name(self) -> &'static str {
        match self {
            Language::Rust => "Rust",
            Language::JavaScript => "JavaScript",
            Language::Css => "CSS",
            Language::Html => "HTML",
            Language::Plain => "Plain Text",
        }
    }
}

/// Map syntect foreground colors to MD3-friendly CSS class names.
///
/// Uses approximate color ranges matched to the base16-ocean.dark theme palette:
/// - base03 `#65737e` (101,115,126) — comments
/// - base0B `#a3be8c` (163,190,140) — strings
/// - base0E `#b48ead` (180,142,173) — keywords
/// - base0D `#8fa1b3` (143,161,179) — functions
/// - base0A `#ebcb8b` (235,203,139) — types/classes
/// - base09 `#d08770` (208,135,112) — numbers/constants
/// - base0F `#ab7967` (171,121,103) — deprecated/builtin
fn scope_to_class(scopes: &[syntect::highlighting::Style]) -> &str {
    if scopes.is_empty() {
        return "";
    }

    let fg = scopes[0].foreground;

    // Match against known base16-ocean.dark palette colors with tolerance
    match (fg.r, fg.g, fg.b) {
        // Comments: ~#65737e (101,115,126) — gray
        (r, g, b) if (90..120).contains(&r) && (100..130).contains(&g) && (115..140).contains(&b) => "hl-comment",
        // Strings: ~#a3be8c (163,190,140) — green
        (r, g, b) if (140..180).contains(&r) && (175..210).contains(&g) && (120..160).contains(&b) => "hl-string",
        // Keywords: ~#b48ead (180,142,173) — purple
        (r, g, b) if (160..200).contains(&r) && (120..170).contains(&g) && (155..195).contains(&b) => "hl-keyword",
        // Functions: ~#8fa1b3 (143,161,179) — blue
        (r, g, b) if (125..165).contains(&r) && (145..180).contains(&g) && (165..200).contains(&b) => "hl-function",
        // Types: ~#ebcb8b (235,203,139) — yellow
        (r, g, b) if (215..255).contains(&r) && (185..220).contains(&g) && (120..165).contains(&b) => "hl-type",
        // Numbers: ~#d08770 (208,135,112) — orange
        (r, g, b) if (190..225).contains(&r) && (115..155).contains(&g) && (95..135).contains(&b) => "hl-number",
        // Builtin/deprecated: ~#ab7967 (171,121,103) — brown
        (r, g, b) if (155..190).contains(&r) && (105..140).contains(&g) && (85..120).contains(&b) => "hl-number",
        // Escape/special: ~#96b5b4 (150,181,180) — cyan
        (r, g, b) if (130..170).contains(&r) && (165..200).contains(&g) && (165..200).contains(&b) => "hl-type",
        // Bright text on dark bg (punctuation/operators): ~#c0c5ce or #eff1f5
        (r, g, b) if r > 180 && g > 180 && b > 180 => "hl-punctuation",
        _ => "",
    }
}

/// Tokenize source code into styled tokens using syntect.
pub fn highlight(source: &str, lang: Language) -> Vec<Token> {
    if matches!(lang, Language::Plain) {
        return vec![Token {
            text: source.to_string(),
            class: String::new(),
        }];
    }

    let syntaxes = SyntaxSet::load_defaults_newlines();
    let themes = ThemeSet::load_defaults();

    let syntax = match syntaxes.find_syntax_by_name(lang.syntax_name()) {
        Some(s) => s,
        None => syntaxes.find_syntax_plain_text(),
    };

    let mut highlighter = HighlightLines::new(syntax, &themes.themes["base16-ocean.dark"]);

    let mut tokens = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let total_lines = lines.len();

    for (idx, line) in lines.iter().enumerate() {
        if let Ok(highlighted) = highlighter.highlight_line(line, &syntaxes) {
            for (style, text) in highlighted {
                let scopes = vec![style];
                let class = scope_to_class(&scopes).to_string();
                tokens.push(Token {
                    text: text.to_string(),
                    class,
                });
            }
        } else {
            tokens.push(Token {
                text: line.to_string(),
                class: String::new(),
            });
        }
        // Add newline between lines (except after the last line)
        if idx < total_lines - 1 {
            tokens.push(Token {
                text: "\n".to_string(),
                class: String::new(),
            });
        }
    }

    tokens
}
