mod measurement_simulation;

#[cfg(test)]
mod test {

    use measurement_simulation::sonar_module::*;
    use measurement_simulation::test_results::*;
    use measurement_simulation::test_condition::*;

    #[test]
    fn stuct_input_check() {
        let check_stuct = SonarModule::new(2.0,1);
        assert_eq!(2.0f32, check_stuct.velocity());
        assert_eq!(1i8, check_stuct.num_sensors());
    }

    #[test]
    fn test_condition_test() {
        let test_condiiton = TestCondition::new( 0.5,0.5,7.5, 30.0, 5.0);
    }

}
