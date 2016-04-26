mod measurement_simulation;

#[cfg(test)]
mod test {

    use measurement_simulation::sonar_module::*;
    use measurement_simulation::test_results::*;
    use measurement_simulation::test_condition::*;

    /// Constructs a new `SonarModule`.
    ///
    /// # Examples
    ///
    /// ```
    /// let check_stuct = SonarModule{dist_to_core: 0.5,velocity: 2.0,num_sensors: 1,effective_zone: 0.5};
    /// assert_eq!(0.5f32, check_stuct.dist_to_core);
    /// assert_eq!(2.0f32, check_stuct.velocity);
    /// assert_eq!(1i8, check_stuct.num_sensors);
    /// assert_eq!(0.5f32, check_stuct.effective_zone);
    /// ```
    #[test]
    fn stuct_input_check() {
        let check_stuct = SonarModule{dist_to_core: 0.5,velocity: 2.0,num_sensors: 1,effective_zone: 0.5};
        assert_eq!(0.5f32, check_stuct.dist_to_core);
        assert_eq!(2.0f32, check_stuct.velocity);
        assert_eq!(1i8, check_stuct.num_sensors);
        assert_eq!(0.5f32, check_stuct.effective_zone);
    }

    #[test]
    fn test_condition_test() {
        let test_condiiton = TestCondition::new(  0.5, 0.5,7.5, 5.0, 5.0, 0.0);
    }

    #[test]
    fn test_return_timeFn() {
        let check_stuct = SonarModule{dist_to_core: 0.5,velocity: 2.0,num_sensors: 1,effective_zone: 0.5};
        let test_condiiton = TestCondition::new(  0.5, 0.5,7.5, 5.0, 5.0, 0.0);
        let test_results = TestResults::new();
        let return_time : Vec<f32>= test_results.return_time(&check_stuct, &test_condiiton);
    }

}
