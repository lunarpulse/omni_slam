mod measurement_simulation;
use measurement_simulation::sonar_module::*;
use measurement_simulation::test_condition::*;
use measurement_simulation::test_results::*;
extern crate csv;

fn main() {
    let test_condiiton = TestCondition::new( 0.5, 0.5, 7.5, 5.0, 5.0, 0.0);
    println!("test_condiiton: {:?}", &test_condiiton);

    test_condiiton.set_up_diameters( 0.25, 1.0, 3.5,);
    println!("test_condiiton set_up_diameters: {:?}", test_condiiton);

    let check_stuct = SonarModule{dist_to_core: 1.5,velocity: 340.0,num_sensors: 1,effective_zone: 0.5} ;
    println!("check_stuct : {:?}", check_stuct);

    //initialising the test struct to store test results.
    let test_results = TestResults::new();

    let return_time : Vec<(f32,f32)>= test_results.return_time(&check_stuct, &test_condiiton);
    println!("return_time: {:?} s", return_time);

    let return_frequency : Vec<(f32,f32)>= test_results.return_hz(&check_stuct, &test_condiiton);
    //let return_vec: Vec<_>= return_frequency.iter().map(|x| println!("{} hz", x)).collect();
    println!("return_frequency: {:?} Hz\n", return_frequency);

    println!("TesResults: {:?}\n", test_results);

    let cal_arc = test_results.calculate_arc(&check_stuct, &test_condiiton);
    println!("cal_arc: {:?}\n", cal_arc.capacity());

    println!("test_condiiton: {:?}\n", test_condiiton);

    let data_acquition = test_results.decide_acquisition(&check_stuct, &test_condiiton);
    //data_acquition.iter().map(move|(_,_,x)|if x == false{println!("data_acquition: {:?}\n", data_acquition);});
    println!("data_acquition: {:?}\n", data_acquition.is_empty());

    let data_to_csv = test_results.toString();
    //println!("data_to_csv: {:?}", data_to_csv);
    let mut rdr = csv::Reader::from_string(data_to_csv).has_headers(false);
    for row in rdr.decode() {
        let (radius, angle_speed, accquired): (f32, f32, bool) = row.unwrap();
        println!("{}, {}: {}", angle_speed, radius, accquired);
    }

}
