use std::collections::HashMap;
use std::sync::OnceLock;

include!(concat!(env!("OUT_DIR"), "/emoji_names.rs"));

const FE0F: char = '\u{FE0F}';

struct Table {
    map: HashMap<&'static str, &'static str>,
    max_len: usize, // longest key in chars
}

fn tables() -> &'static HashMap<&'static str, Table> {
    static TABLES: OnceLock<HashMap<&'static str, Table>> = OnceLock::new();
    TABLES.get_or_init(|| {
        LANGUAGES
            .iter()
            .map(|&(code, entries)| {
                let map: HashMap<&str, &str> = entries.iter().copied().collect();
                let max_len = entries.iter().map(|(k, _)| k.chars().count()).max().unwrap_or(1);
                (code, Table { map, max_len })
            })
            .collect()
    })
}

pub fn available_languages() -> Vec<&'static str> {
    LANGUAGES.iter().map(|&(code, _)| code).collect()
}

/// Replaces every emoji in `input` with its CLDR short name in `lang`.
/// Returns None if that language was not enabled as a feature.
pub fn replace_emoji(input: &str, lang: &str) -> Option<String> {
    let table = tables().get(lang)?;
    let chars: Vec<char> = input.chars().collect();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;

    while i < chars.len() {
        let max_end = (i + table.max_len * 2).min(chars.len());
        let mut matched: Option<(usize, &str)> = None;

        for end in (i + 1..=max_end).rev() {
            let candidate: String = chars[i..end].iter().filter(|&&c| c != FE0F).collect();
            if let Some(name) = table.map.get(candidate.as_str()) {
                matched = Some((end, name));
                break;
            }
        }

        match matched {
            Some((end, name)) => {
                out.push_str(name);
                i = end;
            }
            None => {
                out.push(chars[i]);
                i += 1;
            }
        }
    }

    Some(out)
}