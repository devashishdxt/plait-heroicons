use plait::html;

#[test]
fn outline_academic_cap_has_correct_svg_attrs() {
    use plait_heroicons::outline::AcademicCap;

    let rendered = html! { @AcademicCap() {} }.to_string();

    assert!(rendered.contains(r#"xmlns="http://www.w3.org/2000/svg""#));
    assert!(rendered.contains(r#"fill="none""#));
    assert!(rendered.contains(r#"viewBox="0 0 24 24""#));
    assert!(rendered.contains(r#"stroke-width="1.5""#));
    assert!(rendered.contains(r#"stroke="currentColor""#));
    assert!(rendered.contains(r#"aria-hidden="true""#));
    assert!(rendered.contains(r#"data-slot="icon""#));
    assert!(rendered.contains("stroke-linecap"));
    assert!(rendered.contains("stroke-linejoin"));
}

#[test]
fn solid_academic_cap_has_correct_svg_attrs() {
    use plait_heroicons::solid::AcademicCap;

    let rendered = html! { @AcademicCap() {} }.to_string();

    assert!(rendered.contains(r#"xmlns="http://www.w3.org/2000/svg""#));
    assert!(rendered.contains(r#"viewBox="0 0 24 24""#));
    assert!(rendered.contains(r#"fill="currentColor""#));
    assert!(rendered.contains(r#"aria-hidden="true""#));
    assert!(rendered.contains(r#"data-slot="icon""#));
    // Solid icons should NOT have stroke attrs
    assert!(!rendered.contains("stroke-width"));
    assert!(!rendered.contains(r#"fill="none""#));
}

#[test]
fn mini_has_correct_viewbox() {
    use plait_heroicons::mini::AcademicCap;

    let rendered = html! { @AcademicCap() {} }.to_string();

    assert!(rendered.contains(r#"viewBox="0 0 20 20""#));
    assert!(rendered.contains(r#"fill="currentColor""#));
}

#[test]
fn micro_has_correct_viewbox() {
    use plait_heroicons::micro::AcademicCap;

    let rendered = html! { @AcademicCap() {} }.to_string();

    assert!(rendered.contains(r#"viewBox="0 0 16 16""#));
    assert!(rendered.contains(r#"fill="currentColor""#));
}

#[test]
fn attrs_forwarding_works() {
    use plait_heroicons::outline::AcademicCap;

    let rendered = html! { @AcademicCap(; class: "w-6 h-6 text-blue-500") {} }.to_string();

    assert!(rendered.contains(r#"class="w-6 h-6 text-blue-500""#));
    assert!(rendered.contains(r#"xmlns="http://www.w3.org/2000/svg""#));
}

#[test]
fn micro_stop_renders_rect_element() {
    use plait_heroicons::micro::Stop;

    let rendered = html! { @Stop() {} }.to_string();

    assert!(rendered.contains("<rect"));
    assert!(rendered.contains(r#"width="10""#));
    assert!(rendered.contains(r#"height="10""#));
    assert!(rendered.contains(r#"rx="1.5""#));
}
