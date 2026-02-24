//! [Heroicons](https://heroicons.com) as [`plait`](https://crates.io/crates/plait) components.
//!
//! # Usage
//!
//! Icons are organized into four modules, matching the Heroicons styles:
//!
//! | Module      | Size | Style   |
//! |-------------|------|---------|
//! | [`outline`] | 24px | Outline |
//! | [`solid`]   | 24px | Solid   |
//! | [`mini`]    | 20px | Solid   |
//! | [`micro`]   | 16px | Solid   |
//!
//! Each icon is a component function named in PascalCase after the original SVG filename (e.g. `academic-cap.svg`
//! becomes `AcademicCap`).
//!
//! ```rust
//! use plait::component;
//! use plait_heroicons::outline;
//!
//! component! {
//!     pub fn Navbar() {
//!         nav {
//!             @outline::BellAlert() {}
//!             @outline::UserCircle() {}
//!         }
//!     }
//! }
//! ```
//!
//! You can use icons from different styles in the same component:
//!
//! ```rust
//! use plait::component;
//! use plait_heroicons::{solid, mini};
//!
//! component! {
//!     pub fn Controls() {
//!         div {
//!             @solid::PlayCircle() {}
//!             @mini::ChevronDown() {}
//!         }
//!     }
//! }
//! ```

/// 24px outline icons.
pub mod outline {
    use plait::component;
    include!(concat!(env!("OUT_DIR"), "/outline.rs"));
}

/// 24px solid icons.
pub mod solid {
    use plait::component;
    include!(concat!(env!("OUT_DIR"), "/solid.rs"));
}

/// 20px solid (mini) icons.
pub mod mini {
    use plait::component;
    include!(concat!(env!("OUT_DIR"), "/mini.rs"));
}

/// 16px solid (micro) icons.
pub mod micro {
    use plait::component;
    include!(concat!(env!("OUT_DIR"), "/micro.rs"));
}
