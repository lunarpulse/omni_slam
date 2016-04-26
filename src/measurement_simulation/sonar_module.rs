use std::cell::RefCell;
use std::sync::Arc;

#[derive(Debug)]
pub struct SonarModule{
    //temperature: f32,
    //humidity: f32,
    velocity: f32,
    num_sensors: i8,
    results_distance: Arc<RefCell<Vec<u32>>>,
}

impl SonarModule {
    pub fn new(velocity: f32, num_sensors: i8) -> SonarModule{
        let result_vector : Vec<_> = (0..100u32).collect();
        let ref_result = RefCell::new(result_vector);
        let arc_ref_result = Arc::new(ref_result);
        SonarModule{
            velocity: velocity,
            num_sensors: num_sensors,
            results_distance: arc_ref_result,
        }
    }

    pub fn velocity(&self) -> f32 {
        self.velocity
    }

    pub fn num_sensors(&self) -> i8 {
        self.num_sensors
    }
}
