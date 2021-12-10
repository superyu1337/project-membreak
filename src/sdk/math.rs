use super::Vector3;

pub fn vector_normalise(vec: &mut Vector3) {
    // Normalize yaw to -180, 180
    vec.y = libm::fmodf(vec.y, 360.0);
    if vec.y > 180.0 { vec.y -= 360.0 }
    else if vec.y < -180.0 { vec.y += 360.0 };

    // Clamp pitch to -89, 89
    vec.x = libm::fminf(libm::fmaxf(vec.x, -89.0), 89.0);

    // Roll to zero
    vec.z = 0.0;
}

const PI_180: f32 = 57.2957795131;

pub fn calc_angles(src: Vector3, dst: Vector3) -> Vector3 {
    let mut angles = Vector3{x: 0f32, y: 0f32, z: 0f32};
    let delta = Vector3{x: src.x - dst.x, y: src.y - dst.y, z: src.z - dst.z };
    let hyp = libm::sqrtf(delta.x * delta.x + delta.y * delta.y);
    angles.x = PI_180 * libm::atanf(delta.z / hyp);
    angles.y = PI_180 * libm::atanf(delta.y / delta.x);
    angles.z = 0.0;

    if delta.x >= 0.0 {
        angles.y += 180.0;
    }

    return angles;
}