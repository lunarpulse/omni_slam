#[derive(Debug)]
pub struct SonarModule{
    //temperature: f32,
    //humidity: f32,
    pub dist_to_core: f32,
    //this may go to the test conditions with temperature and humidity later
    pub velocity: f32,
    pub num_sensors: i8,
    pub effective_zone: f32, //TODO into a circle or hemispere after 3D
}
