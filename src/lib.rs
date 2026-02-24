pub mod outline {
    use plait::component;
    include!(concat!(env!("OUT_DIR"), "/outline.rs"));
}

pub mod solid {
    use plait::component;
    include!(concat!(env!("OUT_DIR"), "/solid.rs"));
}

pub mod mini {
    use plait::component;
    include!(concat!(env!("OUT_DIR"), "/mini.rs"));
}

pub mod micro {
    use plait::component;
    include!(concat!(env!("OUT_DIR"), "/micro.rs"));
}
