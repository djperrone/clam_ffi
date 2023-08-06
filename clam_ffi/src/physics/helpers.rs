pub fn get_magnitude(vector: glam::Vec3) -> f32 {
    (f32::powf(vector.x, 2.) + f32::powf(vector.y, 2.) + f32::powf(vector.z, 2.)).sqrt()
}

pub fn set_magnitude(mut vector: glam::Vec3, new_mag: f32) -> glam::Vec3 {
    let old_mag = get_magnitude(vector);
    let ratio: f32 = new_mag / old_mag.max(0.0001);
    vector *= ratio;
    return vector;
}
