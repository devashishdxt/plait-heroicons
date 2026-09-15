# plait-heroicons

[Heroicons](https://heroicons.com) as [`plait`](https://crates.io/crates/plait) components.

## Usage

Icons are organized into four modules, matching the Heroicons styles:

| Module      | Size | Style   |
|-------------|------|---------|
| [`outline`] | 24px | Outline |
| [`solid`]   | 24px | Solid   |
| [`mini`]    | 20px | Solid   |
| [`micro`]   | 16px | Solid   |

Each icon is a component function named in PascalCase after the original SVG filename (e.g. `academic-cap.svg`
becomes `AcademicCap`).

```rust
use plait::component;
use plait_heroicons::outline;

component! {
    pub fn Navbar() {
        nav {
            @outline::BellAlert(; aria_hidden: "true") {}
            @outline::UserCircle(; aria_hidden: "true") {}
        }
    }
}
```

You can use icons from different styles in the same component:

```rust
use plait::component;
use plait_heroicons::{solid, mini};

component! {
    pub fn Controls() {
        div {
            @solid::PlayCircle(; aria_hidden: "true") {}
            @mini::ChevronDown(; aria_hidden: "true") {}
        }
    }
}
```

## Plait compatibility

This checkout requires Plait `0.9.0-dev.0` from the following immutable Git
revision. Consumers must use the same source and revision, not registry Plait
0.8, so generated components share the construction protocol and trait identity.

```toml
[dependencies]
plait = { git = "https://github.com/devashishdxt/plait.git", rev = "42e3b39f1f6ddf120d908d0e9a78cab6de419734", default-features = false }
```

Rebuild older component libraries with the matching Plait macro/runtime pair.
Generated `@Icon(; attributes) {}` calls keep their syntax.

## Attribute ownership and accessibility

**Migration:** generated icons no longer automatically emit `aria-hidden="true"`
or `data-slot="icon"`. Both attributes are caller-owned and forwarded to the SVG
root with no wrapper. Add `aria_hidden: "true"` for decorative icons and
`data_slot: "icon"` when your parent styles use that slot. Use `micro` for 16px
solid drawings and `mini` for 20px solid drawings; CSS sizing does not change
which drawing variant is selected.

Icon-only controls need an accessible name on the control. Meaningful standalone
icons need explicit semantics and a name; they must not be hidden from assistive
technology. For example:

```rust
use plait::{html, ToHtml};
use plait_heroicons::{micro, mini};

let content = html! {
    button(type: "button", aria_label: "Notifications") {
        @micro::BellAlert(; data_slot: "icon", aria_hidden: "true") {}
    }
    @mini::AcademicCap(; role: "img", aria_label: "Education") {}
}.to_html();
assert!(content.contains("aria-label=\"Education\""));
```

The generator still owns the SVG namespace, viewBox, fill, and any source stroke
attributes. Do not repeat those through extra attributes: Plait forwards rather
than merges attributes. Caller classes, `data-*`, `aria-*`, and `role` reach the
root; the source paths and other drawing geometry are unchanged.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
