use ev3dev_lang_rust::{motors::LargeMotor, Ev3Error};

/// The `search_for_ball` regime is responsible for searching for the HiTechnic infrared ball.
///
/// The robot rotates counter-clockwise in this regime.
pub fn search_for_ball(motor_left: &LargeMotor, motor_right: &LargeMotor) -> Result<(), Ev3Error> {
    motor_left.set_duty_cycle_sp(-50)?;
    motor_right.set_duty_cycle_sp(50)?;

    Ok(())
}

/// The `correction` regime is responsible for correcting the robot's trajectory according to a target `error_value` which tries to reach `0`.
///
/// The robot turns left if the error is negative and right if the error is positive.
///
/// The degree of correction is determined by the `motor_diff` parameter, which is the difference between the duty cycle of the left and right motors.
///
/// Example:
/// ```rs
/// let sensor_value = ... // get sensor value here
/// regimes::correction(sensor_value, 30, &motor_left, &motor_right)?;
/// ```
pub fn correction(
    error_value: i32,
    motor_diff: i32,
    speed: i32,
    motor_left: &LargeMotor,
    motor_right: &LargeMotor,
) -> Result<(), Ev3Error> {
    if error_value < 0 {
        motor_left.set_duty_cycle_sp(speed - motor_diff)?;
        motor_right.set_duty_cycle_sp(speed)?;
    }
    if error_value > 0 {
        motor_left.set_duty_cycle_sp(speed)?;
        motor_right.set_duty_cycle_sp(speed - motor_diff)?;
    }

    Ok(())
}
