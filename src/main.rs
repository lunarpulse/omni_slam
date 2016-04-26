mod measurement_simulation;
use measurement_simulation::sonar_module::*;
use measurement_simulation::test_condition::*;
use measurement_simulation::test_results::*;

fn main() {
    let test_condiiton = TestCondition::new( 0.5, 0.5,7.5, 5.0, 5.0, 0.0);
    println!("test_condiiton: {:?}", &test_condiiton);

    test_condiiton.set_up_diameters( 0.25, 1.0, 3.5,);
    println!("test_condiiton set_up_diameters: {:?}", test_condiiton);

    let check_stuct = SonarModule{dist_to_core: 0.5,velocity: 340.0,num_sensors: 1,effective_zone: 0.5} ;
    println!("check_stuct : {:?}", check_stuct);

    let test_results = TestResults::new();

    let return_time : Vec<f32>= test_results.return_time(&check_stuct, &test_condiiton);
    println!("return_time: {:?} s", return_time);

    let return_frequency : Vec<f32>= test_results.return_hz(&check_stuct, &test_condiiton);
    //let return_vec: Vec<_>= return_frequency.iter().map(|x| println!("{} hz", x)).collect();
    println!("return_frequency: {:?} Hz\n", return_frequency);

    println!("TesResults: {:?}\n", test_results);
    println!("test_condiiton: {:?}", test_condiiton);


}
