use bevy::math::Vec3;

/// "ENU coordinate system" means the LV95 reference East-North-Up system

/// Center of the H of the Heli-Pad at Wichlen
pub static ENGINE_ORIGIN_IN_ENU: Vec3 = Vec3::new(2_728_507.0, 1_194_530.0, 1_315.0);

pub static CAMERA_START_IN_ENU: Vec3 = Vec3::new(2_729_464.0, 1_195_442.0, 3_500.0);

pub static CAMERA_START_TARGET_POS: Vec3 = Vec3::new(2_727_184.0, 1_194_512.0, 1_443.0);

pub fn enu_to_engine(mut coords: Vec3) -> Vec3 {
    coords -= ENGINE_ORIGIN_IN_ENU;

    Vec3::new(coords.y, coords.z, coords.x)
}

pub fn ned_to_engine(coords: Vec3) -> Vec3 {
    // NED reference == Engine Origin

    Vec3::new(coords.y, -coords.z, -coords.x)
}
