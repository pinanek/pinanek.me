use strum::{Display, EnumString};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum HighlightLanguageKind {
    #[strum(
        to_string = "Bash",
        serialize = "bash",
        serialize = "sh",
        serialize = "shell"
    )]
    Bash,
    #[strum(to_string = "Console", serialize = "console")]
    Console,
    #[strum(to_string = "CSS", serialize = "css")]
    Css,
    #[strum(
        to_string = "Dockerfile",
        serialize = "containerfile",
        serialize = "docker",
        serialize = "dockerfile"
    )]
    Dockerfile,
    #[strum(to_string = "HTML", serialize = "html")]
    Html,
    #[strum(to_string = "JavaScript", serialize = "javascript", serialize = "js")]
    JavaScript,
    #[strum(to_string = "JSON", serialize = "json", serialize = "jsonc")]
    Json,
    #[strum(to_string = "JSX", serialize = "jsx")]
    Jsx,
    #[strum(to_string = "Markdown", serialize = "markdown", serialize = "md")]
    Markdown,
    #[strum(to_string = "Python", serialize = "py", serialize = "python")]
    Python,
    #[strum(to_string = "Rust", serialize = "rs", serialize = "rust")]
    Rust,
    #[strum(to_string = "TSX", serialize = "tsx")]
    Tsx,
    #[strum(to_string = "TypeScript", serialize = "ts", serialize = "typescript")]
    TypeScript,
}

impl HighlightLanguageKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Bash | Self::Console => "bash",
            Self::Css => "css",
            Self::Dockerfile => "dockerfile",
            Self::Html => "html",
            Self::JavaScript => "javascript",
            Self::Json => "json",
            Self::Jsx => "jsx",
            Self::Markdown => "markdown",
            Self::Python => "python",
            Self::Rust => "rust",
            Self::Tsx => "tsx",
            Self::TypeScript => "typescript",
        }
    }
}
