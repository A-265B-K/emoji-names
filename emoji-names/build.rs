use roxmltree::{Document, ParsingOptions};
use std::{env, fs, path::PathBuf};

fn feature_env_var(stem: &str) -> String {
    format!(
        "CARGO_FEATURE_{}",
        stem.to_ascii_uppercase().replace('-', "_")
    )
}

fn extract(doc: &Document) -> Vec<(String, String)> {
    let mut entries: Vec<(String, String)> = doc
        .descendants()
        .filter(|n| {
            n.has_tag_name("annotation") && n.attribute("type") == Some("tts")
        })
        .filter_map(|n| {
            let cp = n.attribute("cp")?;
            if cp.contains('{') {
                return None; // CLDR range placeholders
            }
            let name = n.text()?.trim().to_string();
            let key: String = cp.chars().filter(|&c| c != '\u{FE0F}').collect();
            Some((key, name))
        })
        .collect();
    entries.sort();
    entries.dedup_by(|a, b| a.0 == b.0);
    entries
}

fn main() {
    println!("cargo:rerun-if-changed=data/annotations");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut files: Vec<PathBuf> = fs::read_dir("data/annotations")
        .expect("failed to read data/annotations")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().map_or(false, |x| x == "xml"))
        .collect();
    files.sort();

    let mut tables = String::new();
    let mut registry =
        String::from("pub static LANGUAGES: &[(&str, &[(&str, &str)])] = &[\n");
    let opts = ParsingOptions {
        allow_dtd: true,
        ..ParsingOptions::default()
    };

    for (idx, path) in files.iter().enumerate() {
        let stem = path.file_stem().unwrap().to_str().unwrap();
        if stem == "root" {
            continue;
        }

        if env::var_os(feature_env_var(stem)).is_none() {
            continue;
        }

        let code = stem.replace('_', "-").to_lowercase();
        let xml =
            fs::read_to_string(path).expect("failed to read language file");
        let doc = Document::parse_with_options(&xml, opts.clone())
            .unwrap_or_else(|e| {
                panic!("failed to parse {}: {e}", path.display())
            });

        tables.push_str(&format!("static NAMES_{idx}: &[(&str, &str)] = &[\n"));
        for (key, name) in extract(&doc) {
            tables.push_str(&format!("    ({:?}, {:?}),\n", key, name));
        }
        tables.push_str("];\n");

        registry.push_str(&format!("    ({:?}, NAMES_{idx}),\n", code));
    }
    registry.push_str("];\n");

    fs::write(out_dir.join("emoji_names.rs"), tables + &registry)
        .expect("failed to write emoji_names.rs");
}
