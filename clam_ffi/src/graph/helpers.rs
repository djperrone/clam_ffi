pub fn get_magnitude(vector: glam::Vec3) -> f32 {
    // (f32::powf(vector.x, 2.) + f32::powf(vector.y, 2.) + f32::powf(vector.z, 2.)).sqrt()
    vector.length()
}

// pub fn set_magnitude(mut vector: glam::Vec3, new_mag: f32) -> glam::Vec3 {
//     let old_mag = get_magnitude(vector);
//     let ratio: f32 = new_mag / old_mag.max(0.0001);
//     vector *= ratio;
//     return vector;
// }

pub fn set_magnitude(mut vector: glam::Vec3, new_mag: f32) -> glam::Vec3 {
    let old_mag = vector.length();

    if old_mag.abs() > std::f32::EPSILON {
        let ratio: f32 = new_mag / old_mag;
        vector *= ratio;
    } else {
        // Handle the case where the original magnitude is very close to zero
        // You can choose to return a default vector or handle it differently based on your requirements
        // Here, we return a zero vector as an example
        vector = glam::Vec3::ZERO;
    }

    return vector;
}
