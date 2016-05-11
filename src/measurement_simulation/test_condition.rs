use std::cell::RefCell;
use std::sync::Arc;

#[derive(Debug)]
pub struct TestCondition  {
    rad_incre: f32,
    start:  f32,
    end:f32,
    ang_incre: f32,
    angle:f32,
    diameters: Arc<RefCell<Vec<f32>>>,
    //arcs: Arc<RefCell<Vec<f32>>>, //TODO implement the angle fraction part
}

impl TestCondition {
    pub fn new(rad_incre: f32, start: f32,end: f32, ang_incre: f32, angle: f32) -> TestCondition{

        let mut result_vector = vec![start,];
        let mut i = start;

        while i<end {
            i += rad_incre;
            result_vector.push(i);
        }
        let ref_result = RefCell::new(result_vector);
        let arc_ref_result = Arc::new(ref_result);

        TestCondition{
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
