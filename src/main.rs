use std::f64::consts;
const PI: f64 = consts::PI;

#[derive(Debug)]
struct Object {
    mass: Mass,
}

#[derive(Debug)]
pub enum Mass {
    Value(f64),
    MassLess,
}

#[derive(Debug)]
pub struct SHMParams{
    angular_velocity: f64,
    phase_angle: f64,
    amplitude: f64,
}

#[derive(Debug)]
pub struct Spring {
    spring_constant: f64, // spring constant 
    lenght: f64,
    mass: Mass,
    motion: SHMParams,
    attached_object: Option<Object>,
}

impl Spring {
    fn new(spring_constant:f64, lenght:f64, mass:Mass, motion: SHMParams, attached_object: Option<Object>) -> Self {
        Spring {spring_constant, lenght, mass, motion, attached_object} 
        }
}

trait SimpleHarmonicMotion {
    // add code here
    fn shm(&self, time: f64) -> f64;
}

impl SimpleHarmonicMotion for Spring {
    // calculate x-positon given time T
    fn shm(&self, time: f64) -> f64 {
        let pos = self.motion.angular_velocity * time + self.motion.phase_angle;
        self.motion.amplitude * pos.sin()
    }
}

fn main() {
    let shm_params = SHMParams{angular_velocity: 1.0, amplitude: 5.0, phase_angle: 0.0};
    let obj = Object { mass: Mass::Value(2.0)}; 
    let test = Spring::new(1.9, 
                           2.9, 
                           Mass::MassLess, 
                           shm_params, 
                           Some(obj)
                           );
    let time = 0.0;
    println!("soo thee result: {}", test.shm(time));
    let x : f64 = 2.0;
    let sinx = PI * x.sin();
    println!("{}",sinx);

}
