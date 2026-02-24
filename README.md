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
            @outline::BellAlert() {}
            @outline::UserCircle() {}
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
            @solid::PlayCircle() {}
            @mini::ChevronDown() {}
        }
    }
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
