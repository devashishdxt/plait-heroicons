use plait::{ToHtml, html};

fn assert_same_drawing(actual: roxmltree::Node<'_, '_>, source: roxmltree::Node<'_, '_>) {
    assert_eq!(actual.tag_name(), source.tag_name());
    let expected: Vec<_> = source
        .attributes()
        .filter(|attr| !matches!(attr.name(), "aria-hidden" | "data-slot"))
        .map(|attr| (attr.name(), attr.value()))
        .collect();
    let actual_attrs: Vec<_> = actual
        .attributes()
        .map(|attr| (attr.name(), attr.value()))
        .collect();
    assert_eq!(actual_attrs, expected);
    let actual_children: Vec<_> = actual.children().filter(|node| node.is_element()).collect();
    let source_children: Vec<_> = source.children().filter(|node| node.is_element()).collect();
    assert_eq!(actual_children.len(), source_children.len());
    for (actual, source) in actual_children.into_iter().zip(source_children) {
        assert_same_drawing(actual, source);
    }
}

macro_rules! variant_contract {
    ($module:ident, $source:literal) => {
        mod $module {
            use super::*;
            use plait_heroicons::$module::AcademicCap;

            #[test]
            fn omitted_attributes_preserve_drawing_without_accessibility_defaults() {
                let rendered = html! { @AcademicCap() {} }.to_html();
                let actual = roxmltree::Document::parse(&rendered).unwrap();
                let source = roxmltree::Document::parse(include_str!($source)).unwrap();
                assert_same_drawing(actual.root_element(), source.root_element());
            }

            #[test]
            fn decorative_attributes_reach_svg_once_inside_native_control() {
                let class = String::from("example-icon");
                let class = class.as_str();
                let rendered = html! {
                    button(type: "button", aria_label: "Education") {
                        @AcademicCap(; class: class, data_slot: "icon", aria_hidden: "true") {}
                    }
                }.to_html();
                // XML parsing also rejects duplicate attributes on any element.
                let doc = roxmltree::Document::parse(&rendered).unwrap();
                let button = doc.root_element();
                assert_eq!(button.tag_name().name(), "button");
                assert_eq!(button.attribute("aria-label"), Some("Education"));
                let svg = button.first_element_child().unwrap();
                assert_eq!(svg.tag_name().name(), "svg");
                assert_eq!(svg.attribute("class"), Some(class));
                assert_eq!(svg.attribute("data-slot"), Some("icon"));
                assert_eq!(svg.attribute("aria-hidden"), Some("true"));
                assert_eq!(rendered.matches("data-slot=").count(), 1);
                assert_eq!(rendered.matches("aria-hidden=").count(), 1);
            }

            #[test]
            fn meaningful_standalone_icon_has_caller_owned_semantics() {
                let label = String::from("Education & training <available>");
                let label = label.as_str();
                let rendered = html! {
                    @AcademicCap(; role: "img", aria_label: label, aria_hidden: "false", data_slot: "status") {}
                }.to_html();
                let doc = roxmltree::Document::parse(&rendered).unwrap();
                let svg = doc.root_element();
                assert_eq!(svg.tag_name().name(), "svg");
                assert_eq!(svg.attribute("role"), Some("img"));
                assert_eq!(svg.attribute("aria-label"), Some(label));
                assert_eq!(svg.attribute("aria-hidden"), Some("false"));
                assert_eq!(svg.attribute("data-slot"), Some("status"));
                assert!(!rendered.contains("<available>"));
            }
        }
    };
}

variant_contract!(outline, "../optimized/24/outline/academic-cap.svg");
variant_contract!(solid, "../optimized/24/solid/academic-cap.svg");
variant_contract!(mini, "../optimized/20/solid/academic-cap.svg");
variant_contract!(micro, "../optimized/16/solid/academic-cap.svg");
