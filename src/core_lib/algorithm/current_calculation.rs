pub fn single_phase_current_calculation(pe: f32, kx: f32, cos: f32) -> f32 {
    const U: f32 = 0.22;
    let ijs: f32 = pe * kx / U / cos;
    ijs
    //(ijs * 100.0).round() / 100.0
}

pub fn three_phase_current_calculation(pe: f32, kx: f32, cos: f32) -> f32 {
    const U: f32 = 0.38;
    let sqrt3: f32 = 3.0_f32.sqrt();
    let ijs: f32 = pe * kx / U / cos / sqrt3;
    ijs
    //(ijs * 100.0).round() / 100.0
}
