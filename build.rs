use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use convert_case::{Case, Casing};

struct IconVariant {
    dir: &'static str,
    output_file: &'static str,
}

const VARIANTS: &[IconVariant] = &[
    IconVariant {
        dir: "optimized/24/outline",
        output_file: "outline.rs",
    },
    IconVariant {
        dir: "optimized/24/solid",
        output_file: "solid.rs",
    },
    IconVariant {
        dir: "optimized/20/solid",
        output_file: "mini.rs",
    },
    IconVariant {
        dir: "optimized/16/solid",
        output_file: "micro.rs",
    },
];

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo::rerun-if-changed=optimized");

    for variant in VARIANTS {
        let code = generate_variant(variant.dir);
        let out_path = Path::new(&out_dir).join(variant.output_file);
        fs::write(&out_path, code).unwrap();
    }
}

fn generate_variant(dir: &str) -> String {
    let mut icons: BTreeMap<String, String> = BTreeMap::new();

    let mut entries: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("Failed to read directory {dir}: {e}"))
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "svg")
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let stem = path.file_stem().unwrap().to_string_lossy().to_string();
        let component_name = stem.to_case(Case::Pascal);
        let svg_content = fs::read_to_string(&path).unwrap();
        let component_code = generate_component(&component_name, &svg_content);
        icons.insert(component_name, component_code);
    }

    let mut output = String::new();
    for code in icons.values() {
        output.push_str(code);
        output.push('\n');
    }
    output
}

fn generate_component(name: &str, svg_content: &str) -> String {
    let doc = roxmltree::Document::parse(svg_content).unwrap();
    let svg_element = doc.root_element();

    // Build SVG attributes — xmlns is a namespace declaration so roxmltree
    // doesn't expose it as a regular attribute; add it manually.
    let mut svg_attrs = vec!["xmlns: \"http://www.w3.org/2000/svg\"".to_string()];
    for attr in svg_element.attributes() {
        let attr_str = format_attribute(attr.name(), attr.value());
        svg_attrs.push(attr_str);
    }
    svg_attrs.push("#attrs".to_string());

    // Build child elements
    let children = generate_children(&svg_element);

    let svg_attrs_str = svg_attrs.join(", ");

    format!(
        "component! {{\n    pub fn {name}() {{\n        svg({svg_attrs_str}) {{\n{children}        }}\n    }}\n}}\n"
    )
}

fn generate_children(parent: &roxmltree::Node) -> String {
    let mut output = String::new();

    for child in parent.children() {
        if child.is_element() {
            let tag = child.tag_name().name();
            let mut attrs = Vec::new();
            for attr in child.attributes() {
                attrs.push(format_attribute(attr.name(), attr.value()));
            }
            let attrs_str = if attrs.is_empty() {
                String::new()
            } else {
                format!("({})", attrs.join(", "))
            };

            // Check if this element has child elements
            let has_children = child.children().any(|c| c.is_element());
            if has_children {
                let nested = generate_children(&child);
                output.push_str(&format!(
                    "            {tag}{attrs_str} {{\n{nested}            }}\n"
                ));
            } else {
                output.push_str(&format!("            {tag}{attrs_str} {{}}\n"));
            }
        }
    }

    output
}

fn format_attribute(name: &str, value: &str) -> String {
    // Attributes that need string literal syntax because plait lowercases identifiers
    let needs_string_literal = name
        .chars()
        .any(|c| c.is_ascii_uppercase() || c == '@');

    if needs_string_literal {
        // Use string literal: "viewBox": "value"
        let escaped_value = value.replace('\\', "\\\\").replace('"', "\\\"");
        format!("\"{name}\": \"{escaped_value}\"")
    } else {
        // Convert kebab-case to underscore (plait converts back to hyphens)
        let ident = name.replace('-', "_");
        let escaped_value = value.replace('\\', "\\\\").replace('"', "\\\"");
        format!("{ident}: \"{escaped_value}\"")
    }
}
