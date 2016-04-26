use std::cell::RefCell;
use std::sync::Arc;
use measurement_simulation::sonar_module::SonarModule;
use measurement_simulation::test_condition::TestCondition;

#[derive(Debug)]
pub struct TestResults  {
    results_distance: Arc<RefCell<Vec<f32>>>,
    results_accquisition: Arc<RefCell<Vec<f32>>>,
    elasped_time: Arc<RefCell<Vec<f32>>>,
    elasped_frequency: Arc<RefCell<Vec<f32>>>, //TODO implement the angle fraction part
}

impl TestResults {

    pub fn new() -> TestResults{
        let result_vector0 = vec![0.0;1];
        let ref_result0 = RefCell::new(result_vector0);
        let arc_ref_result0 = Arc::new(ref_result0);

        let result_vector1 = vec![0.0;1];
        let ref_result1 = RefCell::new(result_vector1);
        let arc_ref_result1 = Arc::new(ref_result1);

        let result_vector2 = vec![0.0;1];
        let ref_result2 = RefCell::new(result_vector2);
        let arc_ref_result2 = Arc::new(ref_result2);

        let result_vector3  = vec![0.0;1];
        let ref_result3 = RefCell::new(result_vector3);
        let arc_ref_result3 = Arc::new(ref_result3);

        TestResults{
            results_distance: arc_ref_result0,
            results_accquisition:arc_ref_result1,
            elasped_time:arc_ref_result2,
            elasped_frequency:arc_ref_result3,
        }
    }
    pub fn return_time(&self, sonar: &SonarModule, test_condition: &TestCondition) -> Vec<f32>{

        self.elasped_time.borrow_mut().clear();
        for diameter in test_condition.diameters.as_ref().borrow_mut().iter() {
            self.elasped_time.as_ref().borrow_mut().push(diameter/sonar.velocity)
        }
        return self.elasped_time.borrow_mut().clone();
    }

    pub fn return_hz(&self, sonar: &SonarModule, test_condition: &TestCondition) -> Vec<f32>{
        self.elasped_frequency.borrow_mut().clear();
        for diameter in test_condition.diameters.as_ref().borrow_mut().iter() {
            let freq: f32 = 1.0/(diameter/sonar.velocity);
            self.elasped_frequency.borrow_mut().push(freq);
        }
        return self.elasped_frequency.borrow_mut().clone();
    }
}

//TODO implement the Trait if possible
