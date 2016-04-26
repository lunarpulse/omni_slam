use std::cell::RefCell;
use std::sync::Arc;

#[derive(Debug)]
pub struct TestCondition  {
   //radious of the set up`
    rad_incre: f32,
    //radious of the inner test ring
    rad_start:  f32,
    //radious of the outer test ring
    rad_end: f32,
    //increment of the speed each time
    ang_speed_incre: f32,
    //initial angle speed
    ang_speed: f32,
    //initial angle starting point
    ang_start: f32,
    //initial diameters, due to this container the entire struct need to be set up with new function
    pub diameters: Arc<RefCell<Vec<f32>>>,
    //arcs: Arc<RefCell<Vec<f32>>>, //TODO implement the ang_start fraction part
}

impl TestCondition {
    pub fn new(rad_incre: f32, rad_start: f32,rad_end: f32, ang_speed_incre: f32, ang_speed: f32, ang_start: f32) -> TestCondition{

        let mut result_vector = vec![rad_start,];
        result_vector.clear();
        let mut i = rad_start;

        while i<rad_end {
            i += rad_incre;
            result_vector.push(i);
        }
        let ref_result = RefCell::new(result_vector);
        let arc_ref_result = Arc::new(ref_result);

        TestCondition{
            rad_incre: rad_incre,
            rad_start:  rad_start,
            rad_end: rad_end,
            ang_speed_incre: ang_speed_incre,
            ang_speed: ang_speed,
            ang_start: ang_start,
            diameters: arc_ref_result,
            //arcs: Arc<RefCell<Vec<f32>>>,
        }
    }

    pub fn set_up_diameters(&self, rad_incre: f32, rad_start: f32,rad_end: f32) {
        //let mut result_vector = vec![self.rad_start,];
        let mut i = rad_start;
        self.diameters.borrow_mut().clear();
        while i<rad_end {
            i += rad_incre;
            self.diameters.borrow_mut().push(i);
        }
    }
}
