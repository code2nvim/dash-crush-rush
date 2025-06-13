pub mod camera {
    pub mod default {
        pub const POS: (f32, f32, f32) = (0.0, 30.0, 10.0);
    }
}

pub mod ground {
    pub const SIZE: (f32, f32) = (20.0, 20.0);
    pub const COLOR: crate::Color = crate::Color::srgb(0.3, 0.5, 0.3);
}

pub mod player {
    pub mod default {
        pub const RADIUS: f32 = 1.0;
        pub const COLOR: crate::Color = crate::Color::srgb(0.5, 0.5, 0.5);
    }
}
