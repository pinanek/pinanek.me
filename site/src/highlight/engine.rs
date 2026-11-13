use std::ops::RangeInclusive;

use anyhow::{Context, Result};
use comfit::{HighlightConfig, Highlighted, Highlighter, Line, RenderHtmlConfig, Token};

use crate::highlight::HighlightLanguageKind;

pub fn highlight(
    languages: &Highlighter,
    language_kind: Option<&HighlightLanguageKind>,
    content: &str,
    lines: Option<&[RangeInclusive<usize>]>,
) -> Result<String> {
    // A terminal newline ends the last code line; it doesn't start another one.
    let content = content.strip_suffix('\n').unwrap_or(content);
    let config = HighlightConfig {
        highlighted_lines: (1..=content.split('\n').count())
            .filter(|number| {
                lines.is_some_and(|ranges| ranges.iter().any(|range| range.contains(number)))
            })
            .collect(),
        ..HighlightConfig::default()
    };
    let render_config = RenderHtmlConfig {
        pre_class: Some("prose-code-block__pre scrollable".into()),
        rainbow_colors: 6,
        ..RenderHtmlConfig::default()
    };

    let Some(language_kind) = language_kind else {
        let highlighted = Highlighted {
            lines: content
                .split('\n')
                .enumerate()
                .map(|(index, text)| Line {
                    number: index + 1,
                    highlighted: config.highlighted_lines.contains(&(index + 1)),
                    tokens: vec![Token {
                        text,
                        highlight: None,
                        rainbow: None,
                    }],
                })
                .collect(),
        };

        return Ok(highlighted.render_html_classes(&render_config));
    };

    let mut document = languages
        .create_document(language_kind.name(), content)
        .with_context(|| format!("failed to parse content as {language_kind}"))?;
    let highlighted = document
        .highlight_ref(&config)
        .with_context(|| format!("failed to highlight content as {language_kind}"))?;

    Ok(highlighted.render_html_classes(&render_config))
}
