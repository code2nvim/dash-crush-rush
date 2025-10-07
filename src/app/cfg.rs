pub mod bind {
    pub const FIRE: crate::MouseButton = crate::MouseButton::Left;
    pub const SHIELD: crate::MouseButton = crate::MouseButton::Right;
    pub const JUMP: crate::KeyCode = crate::KeyCode::Space;
    pub const MOV_F: crate::KeyCode = crate::KeyCode::KeyW;
    pub const MOV_L: crate::KeyCode = crate::KeyCode::KeyA;
    pub const MOV_B: crate::KeyCode = crate::KeyCode::KeyS;
    pub const MOV_R: crate::KeyCode = crate::KeyCode::KeyD;
}

pub mod boundary {
    pub const SIZE: f32 = 35.0;
}

pub mod bullet {
    pub const COLOR: crate::Color = crate::Color::srgb(0.0, 0.0, 0.6);
    pub const INTERVAL: f32 = 0.15;
    pub const RADIUS: f32 = 0.2;
    pub const SPEED: f32 = 35.0;
}

pub mod camera {
    pub const POS: (f32, f32, f32) = (0.0, 35.0, 35.0);
}

pub mod enemy {
    pub const COLOR: crate::Color = crate::Color::srgb(0.6, 0.0, 0.0);
    pub const INTERVAL: f32 = 2.0;
    pub const RADIUS: f32 = 1.2;
    pub const SPEED: f32 = 5.0;
}

pub mod ground {
    pub const COLOR: crate::Color = crate::Color::srgb(0.0, 0.5, 0.0);
    pub const SIZE: f32 = 30.0;
}

pub mod light {
    pub const BRIGHTNESS: f32 = 300.0;
    pub const ILLUMINANCE: f32 = 3000.0;
}

pub mod obstacle {
    pub const COLOR: crate::Color = crate::Color::srgb(0.3, 0.0, 0.3);
    pub const LENGTH: f32 = 2.0;
    pub const SPEED: f32 = 15.0;
}

pub mod player {
    pub mod default {
        pub const COLOR: crate::Color = crate::Color::srgb(0.5, 0.5, 0.5);
        pub const RADIUS: f32 = 1.0;
        pub const SPEED: f32 = 10.0;
    }
    pub mod shield {
        pub const COLOR: crate::Color = crate::Color::srgb(0.0, 0.0, 1.0);
    }
    pub mod gravity {
        pub const START: f32 = 30.0;
        pub const FALL: f32 = 2.0;
    }
    pub mod gear {
        pub const COLOR: crate::Color = crate::Color::BLACK;
        pub const POS: (f32, f32, f32) = (0.0, 0.4, 0.8);
        pub const SIZE: (f32, f32, f32) = (1.6, 0.8, 0.1);
    }
}

pub mod wall {
    pub const COLOR: crate::Color = crate::Color::srgba(0.0, 0.0, 1.0, 0.5);
    pub const SPEED: f32 = 10.0;
}
