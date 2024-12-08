// Some colors I really liked from a Freya Holmer presentation:
// https://www.youtube.com/watch?v=kfM-yu0iQBk
pub const FREYA_ORANGE: [u8; 4] = [0xeb, 0x79, 0x07, 0xff];
pub const FREYA_GOLD: [u8; 4] = [0xea, 0x9e, 0x36, 0xff];
pub const FREYA_RED: [u8; 4] = [0xf8, 0x10, 0x53, 0xff];
pub const FREYA_BLUE: [u8; 4] = [0x30, 0xb5, 0xf7, 0xff];
pub const FREYA_GREEN: [u8; 4] = [0x0a, 0xeb, 0x9f, 0xff];
pub const FREYA_LIGHT_BLUE: [u8; 4] = [0x72, 0xc5, 0xdd, 0xff];
pub const FREYA_GRAY: [u8; 4] = [0xb2, 0xc5, 0xc5, 0xff];
pub const FREYA_PINK: [u8; 4] = [0xea, 0xa4, 0x83, 0xff];
pub const FREYA_LIGHT_GRAY: [u8; 4] = [0xf4, 0xf5, 0xf8, 0xff];
pub const FREYA_DARK_BLUE: [u8; 4] = [0x4d, 0xa7, 0xc2, 0xff];
pub const FREYA_DARK_GREEN: [u8; 4] = [0x37, 0xbd, 0xa9, 0xff];
pub const FREYA_DARK_RED: [u8; 4] = [0xae, 0x20, 0x44, 0xff];
pub const FREYA_VIOLET: [u8; 4] = [0xa4, 0x01, 0xed, 0xff];
pub const FREYA_WHITE: [u8; 4] = [0xfa, 0xf8, 0xfb, 0xff];
pub const FREYA_YELLOW: [u8; 4] = [0xf7, 0xd4, 0x54, 0xff];
pub const FREYA_LIGHT_YELLOW: [u8; 4] = [0xea, 0xd8, 0xad, 0xff];
pub const FREYA_LIGHT_GREEN: [u8; 4] = [0x6e, 0xc2, 0x9c, 0xff];

// Returns the expected size of units depending on their type
pub fn get_unit_sized_color(unit_name: &str, user_id: i64) -> (f32, [u8; 4]) {
    let mut unit_size = 0.45;
    let color = match unit_name {
        "VespeneGeyser" => FREYA_LIGHT_GREEN,
        "SpacePlatformGeyser" => FREYA_LIGHT_GREEN,
        "LabMineralField" => {
            unit_size = 0.24;
            FREYA_LIGHT_BLUE
        }
        "LabMineralField750" => {
            unit_size = 0.36;
            FREYA_LIGHT_BLUE
        }
        "MineralField" => {
            unit_size = 0.48;
            FREYA_LIGHT_BLUE
        }
        "MineralField450" => {
            unit_size = 0.6;
            FREYA_LIGHT_BLUE
        }
        "MineralField750" => {
            unit_size = 0.72;
            FREYA_LIGHT_BLUE
        }
        "XelNagaTower" => {
            // This should be super transparent
            unit_size = 0.72;
            FREYA_WHITE
        }
        "RichMineralField" => FREYA_GOLD,
        "RichMineralField750" => FREYA_ORANGE,
        "DestructibleRockEx1DiagonalHugeBLUR" => {
            unit_size = 2.0;
            FREYA_GRAY
        }
        "DestructibleDebris6x6" => {
            unit_size = 1.8;
            FREYA_GRAY
        }
        "UnbuildablePlatesDestructible" => {
            unit_size = 0.6;
            FREYA_LIGHT_GRAY
        }
        "Overlord" => {
            unit_size = 0.6;
            FREYA_YELLOW
        }
        "SCV" | "Drone" | "Probe" | "Larva" => {
            unit_size = 0.3;
            FREYA_LIGHT_GRAY
        }
        "Hatchery" | "CommandCenter" | "Nexus" => {
            unit_size = 1.2;
            FREYA_PINK
        }
        "Broodling" => {
            unit_size = 0.06;
            FREYA_LIGHT_GRAY
        }
        _ => {
            // Ignore the Beacons for now.
            if !unit_name.starts_with("Beacon") {
                tracing::warn!("Unknown unit name: '{}'", unit_name);
            }
            // Fallback to user color
            user_color(user_id)
        }
    };
    (unit_size, color)
}

pub fn user_color(user_id: i64) -> [u8; 4] {
    match user_id {
        0 => FREYA_LIGHT_GREEN,
        1 => FREYA_LIGHT_BLUE,
        2 => FREYA_LIGHT_GRAY,
        3 => FREYA_ORANGE,
        _ => FREYA_WHITE,
    }
}
