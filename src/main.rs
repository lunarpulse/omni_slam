mod measurement_simulation;
use measurement_simulation::data_bundle::*;

fn main() {
    let test_condiiton = TestCondition::new( 0.5,0.5,7.5, 30.0, 5.0);
    println!("{:?}", &test_condiiton);

    test_condiiton.set_up_diameters( 0.25,1.0, 3.5,);
    println!("{:?}", test_condiiton);

    let check_stuct = SonarModule::new(2.0,1);
    println!("{:?}", check_stuct);

}
