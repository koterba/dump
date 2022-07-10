pub struct Model {
    pub vel: f64,
    pub angle: f64,
    pub grav: f64
}

////////
// EXAMPLE: Model
////////

// let model = Model {
//     vel: 60.0,
//     angle: 40.0,
//     grav: 9.8,
// }

pub fn get_y(x: f64, model: &Model) -> f64 { // x is height in meters
    let vel = model.vel;
    let angle = model.angle * (std::f64::consts::PI / 180.0f64); // this is to turn it into degrees
    let grav = model.grav;

    let y = (x * angle.tan()) - (
        (
            grav
                * 
            x.powf(2.0)
        )
            /
        (
            (vel.powf(2.0) * 2.0)
                *
            (angle.cos()).powf(2.0)
        )
    );

    y
}
