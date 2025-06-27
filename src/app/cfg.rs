pub mod bind {
    pub const FIRE: crate::MouseButton = crate::MouseButton::Left;
    pub const MOV_F: crate::KeyCode = crate::KeyCode::KeyW;
    pub const MOV_L: crate::KeyCode = crate::KeyCode::KeyA;
    pub const MOV_B: crate::KeyCode = crate::KeyCode::KeyS;
    pub const MOV_R: crate::KeyCode = crate::KeyCode::KeyD;
}

pub mod bullet {
    pub const COLOR: crate::Color = crate::Color::srgb(0.0, 0.0, 0.5);
    pub const INTERVAL: f32 = 0.3;
    pub const RADIUS: f32 = 0.2;
    pub const SPEED: f32 = 20.0;
}

pub mod camera {
    pub const POS: (f32, f32, f32) = (0.0, 30.0, 30.0);
}

pub mod ground {
    pub const COLOR: crate::Color = crate::Color::srgb(0.0, 0.5, 0.0);
    pub const SIZE: f32 = 20.0;
}

pub mod light {
    pub const BRIGHTNESS: f32 = 300.0;
    pub const ILLUMINANCE: f32 = 3000.0;
}

pub mod player {
    pub mod default {
        pub const COLOR: crate::Color = crate::Color::srgb(0.5, 0.5, 0.5);
        pub const RADIUS: f32 = 1.0;
        pub const SPEED: f32 = 10.0;
    }
    pub mod gear {
        pub const COLOR: crate::Color = crate::Color::BLACK;
        pub const POS: (f32, f32, f32) = (0.0, 0.4, 0.8);
        pub const SIZE: (f32, f32, f32) = (1.6, 0.8, 0.1);
    }
}
