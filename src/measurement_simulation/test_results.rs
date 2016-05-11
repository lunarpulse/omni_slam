use std::cell::RefCell;
use std::sync::Arc;
<<<<<<< HEAD
use measurement_simulation::sonar_module::SonarModule;
use measurement_simulation::test_condition::TestCondition;
extern crate std;

#[derive(Debug)]
pub struct TestResults  {
    results_distance: Arc<RefCell<Vec<(f32,f32,f32)>>>,
    results_accquisition: Arc<RefCell<Vec<(f32,f32,bool)>>>,
    elasped_time: Arc<RefCell<Vec<(f32,f32)>>>,
    elasped_frequency: Arc<RefCell<Vec<(f32,f32)>>>, //TODO implement the angle fraction part
}

impl TestResults {

    pub fn new() -> TestResults{
        let result_vector0 = vec![(0.0,0.0,0.0);1];
        let ref_result0 = RefCell::new(result_vector0);
        let arc_ref_result0 = Arc::new(ref_result0);

        let result_vector1 = vec![(0.0,0.0,true);1];
        let ref_result1 = RefCell::new(result_vector1);
        let arc_ref_result1 = Arc::new(ref_result1);

        let result_vector2 = vec![(0.0,0.0);1];
        let ref_result2 = RefCell::new(result_vector2);
        let arc_ref_result2 = Arc::new(ref_result2);

        let result_vector3  = vec![(0.0,0.0);1];
        let ref_result3 = RefCell::new(result_vector3);
        let arc_ref_result3 = Arc::new(ref_result3);

        TestResults{
            results_distance: arc_ref_result0,
            results_accquisition:arc_ref_result1,
            elasped_time:arc_ref_result2,
            elasped_frequency:arc_ref_result3,
        }
    }
    pub fn return_time(&self, sonar: &SonarModule, test_condition: &TestCondition) -> Vec<(f32,f32)>{

        self.elasped_time.borrow_mut().clear();
        for diameter in test_condition.diameters.as_ref().borrow_mut().iter() {
            self.elasped_time.as_ref().borrow_mut().push((*diameter, diameter/sonar.velocity))
        }
        return self.elasped_time.borrow_mut().clone();
    }

    pub fn return_hz(&self, sonar: &SonarModule, test_condition: &TestCondition) -> Vec<(f32,f32)>{
        self.elasped_frequency.borrow_mut().clear();
        for diameter in test_condition.diameters.as_ref().borrow_mut().iter() {
            let freq: f32 = 1.0/(diameter/sonar.velocity);
            self.elasped_frequency.borrow_mut().push((*diameter,freq));
        }
        return self.elasped_frequency.borrow_mut().clone();
    }

    ///Calculating the arc made by the sweeping action and the store away in the results. distance in a tuple form
    ///#Examples
    ///'''
    ///test_condiiton.set_up_diameters( 0.25, 1.0, 3.5,)
    /// let check_stuct = SonarModule{dist_to_core: 0.5,velocity: 340.0,num_sensors: 1,effective_zone: 0.5} ;
    ///initialising the test struct to store test results.
    /// let test_results = TestResults::new();
    ///
    /// let return_time : Vec<f32>= test_results.return_time(&check_stuct, &test_condiiton);
    /// let return_frequency : Vec<f32>= test_results.return_hz(&check_stuct, &test_condiiton);
    /// let cal_arc = test_results.calculate_arc(&check_stuct, &test_condiiton);
    ///'''
    pub fn calculate_arc(&self,sonar:&SonarModule, test_condition:&TestCondition) -> Vec<(f32,f32,f32)>{
        self.results_distance.borrow_mut().clear();
        let mut sensor_angle = test_condition.ang_start;
        while sensor_angle * std::f32::consts::PI/180.0 < std::f32::consts::PI  {
            for r in test_condition.diameters.as_ref().borrow().iter(){
                let arc: f32 = (r + sonar.dist_to_core) * sensor_angle as f32*std::f32::consts::PI;
                self.results_distance.borrow_mut().push((*r,sensor_angle,arc));
            }
            sensor_angle+=1.0;
        }

        return self.results_distance.borrow_mut().clone();
    }

    pub fn decide_acquisition(&self,sonar:&SonarModule, test_condition:&TestCondition) -> Vec<(f32,f32,bool)>{
        //---------------------------------------------------------------------------
        let mut accquired: bool;

        let mut radius : f32;
        //each iteration of diameter increase the angular speed by test_condition.ang_speed_incre
        let mut angle_speed = test_condition.ang_speed * std::f32::consts::PI /180.0;
        self.results_accquisition.borrow_mut().clear();
        //anglular speed level when no more acquiring the signal due to the angular speed is too fast then stop
        //restricting the search within 4 pi radian per seconds which is 2 rotations per second
        while angle_speed< std::f32::consts::PI * 4.0 {
            for diameter in test_condition.diameters.as_ref().borrow().iter(){
                radius = *diameter;
                let prev_sensor_position : f32 = 0.0;
                let flight_time : f32 = diameter/sonar.velocity;
                //angle in radian : a product of angular speed and time return. shows where is the sensor angle.
                //r times thetha is arc length alias the sensor position made by the angular velocity of the sensor
                //degree to radian
                let sensor_angle = test_condition.ang_start  + angle_speed * flight_time;
                //sensor_position = arc, dist_to_core = r, sensor_angle = theta
                let sensor_position = sonar.dist_to_core * sensor_angle;
                // accuired when new sensor position is within 50
                if (sensor_position<prev_sensor_position+sonar.effective_zone/4.0) && (prev_sensor_position-sonar.effective_zone/4.0 <sensor_position) {
                    accquired = true;
                }
                else{
                    accquired = false;
                    //angle_speed to degree again
                    //self.results_accquisition.borrow_mut().push((radius,angle_speed/std::f32::consts::PI /2.0,accquired));
                }

                //recording diameter speed and acquired or not
                //convert the radian to degree
                self.results_accquisition.borrow_mut().push((radius,angle_speed/std::f32::consts::PI *180.0,accquired));
            }
            //converting the test_condition.ang_speed_incre
            angle_speed += test_condition.ang_speed_incre * std::f32::consts::PI /180.0;
        }
        self.results_accquisition.borrow().clone()
    }
///a thing to string which array?
    pub fn to_string(&self) -> String {
        //or String::from("")
        let mut return_string = String::from("");

        for items in self.results_accquisition.as_ref().borrow().iter(){
            let radius:f32 =items.0;
            let angle_speed:f32 = items.1;
            let accquired:bool = items.2;
            return_string = return_string + &radius.to_string()
                    +&String::from(",")+ &angle_speed.to_string()
                        +&String::from(",")+ &accquired.to_string() + &String::from("\n");
        }
        return_string
    }



}

//TODO implement the Trait if possible

//TODO chaining method here
// pub fn l1_penalty(&mut self, l1_penalty: f32) -> &mut Hyperparameters {
//      self.l1_penalty = l1_penalty;
//      self
//  }
=======

#[derive(Debug)]
pub struct TestResults  {
    rad_incre: f32,
    start:  f32,
    end:f32,
    ang_incre: f32,
    angle:f32,
    diameters: Arc<RefCell<Vec<f32>>>,
    //arcs: Arc<RefCell<Vec<f32>>>, //TODO implement the angle fraction part
}

impl TestResults {
    pub fn new(rad_incre: f32, start: f32,end: f32, ang_incre: f32, angle: f32) -> TestResults{

        let mut result_vector = vec![start,];
        let mut i = start;

        while i<end {
            i += rad_incre;
            result_vector.push(i);
        }
        let ref_result = RefCell::new(result_vector);
        let arc_ref_result = Arc::new(ref_result);

        TestResults{
            rad_incre: rad_incre,
            start:  start,
            end: end,
            ang_incre: ang_incre,
            angle: angle,
            diameters: arc_ref_result,
            //arcs: Arc<RefCell<Vec<f32>>>,
        }
    }

    pub fn set_up_diameters(&self, rad_incre: f32, start: f32,end: f32) {
        //let mut result_vector = vec![self.start,];
        let mut i = start;
        self.diameters.borrow_mut().clear();
        while i<end {
            i += rad_incre;
            self.diameters.borrow_mut().push(i);
        }
    }
}

//TODO implement the Trait if possible
>>>>>>> origin/master
