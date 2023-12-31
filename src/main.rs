use std::f64::consts;
const PI: f64 = consts::PI;

#[derive(Debug)]
enum Shape {
    Circle,
    Rectangle,
}

#[derive(Debug)]
struct Object {
    mass: Mass,
    shape: Shape,
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

impl SHMParams {
    fn calculate_angular_velocity(&mut self, obj_mass: Mass, spring_constant:f64){
        match obj_mass{
            Mass::Value(mass) => {
                self.angular_velocity = (spring_constant / mass).sqrt();
            }
            Mass::MassLess => {
                self.angular_velocity = 0.0;
            }
        }

    }
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
    fn shm(&self, time: f64) -> f64;
}

impl SimpleHarmonicMotion for Spring {
    // calculate kjr-positon given time T
    fn shm(&self, time: f64) -> f64 {
        let pos = self.motion.angular_velocity * time + self.motion.phase_angle;
        self.motion.amplitude * pos.sin()
    }
}

fn main() {
    let shm_params = SHMParams{angular_velocity: 1.0, amplitude: 5.0, phase_angle: 0.0};
    let obj = Object { mass: Mass::Value(2.0), shape:Shape::Rectangle}; 
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
