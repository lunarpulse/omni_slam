extern crate csv;

mod measurement_simulation;
use measurement_simulation::sonar_module::*;
use measurement_simulation::test_condition::*;
use measurement_simulation::test_results::*;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
//TODO implement the getopts in this to control from command line

fn main() {
    let test_condiiton = TestCondition::new( 0.5, 0.5, 7.5, 5.0, 5.0, 0.0);
    println!("test_condiiton: {:?}", &test_condiiton);

    test_condiiton.set_up_diameters( 0.5, 0.5, 7.5,);
    println!("test_condiiton set_up_diameters: {:?}", test_condiiton);

    let check_stuct = SonarModule{dist_to_core: 1.5, velocity: 340.0,num_sensors: 1,effective_zone: 0.5} ;
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
    println!("data_acquition is empty? {:?}\n", data_acquition.is_empty());

    //TODO getopts to implment the command line options
    let data_to_csv = test_results.to_string();
    
    //println!("data_to_csv: {:?}", data_to_csv);
    //let mut rdr = csv::Reader::from_string(data_to_csv.clone()).has_headers(false);

    //
    // type Row = (f32, f32, bool);
    // let rows = rdr.decode().collect::<csv::Result<Vec<Row>>>().unwrap();
    // assert_eq!(rows.len(), 1430);

    // for row in rdr.decode() {
    //     let (radius, angle_speed, accquired): (f32, f32, bool) = row.unwrap();
    //     println!("{}, {}: {}", angle_speed, radius, accquired);
    // }
    // while !rdr.done() {
    //     while let Some(r) = rdr.next_bytes().into_iter_result() {
    //         print!("{:?} ", r.unwrap());
    //     }
    //     println!("");
    // }
    //TODO filename from the cmd args
    let file_name = "waveAckCheck.csv".to_string();
    write_csv(data_to_csv,file_name);
}
/// writing it to a csv
/// use std::error::Error;
/// use std::io::prelude::*;
/// use std::fs::File;
/// use std::path::Path;
fn write_csv(data_to_csv:String, file_name:String) {
    //File write down
    let path = Path::new(&file_name);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(data_to_csv.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               Error::description(&why))
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
