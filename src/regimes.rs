use std::{thread, time::Duration};

use ev3dev_lang_rust::{motors::LargeMotor, Ev3Error};

pub fn exit_stall(motor_left: &LargeMotor, motor_right: &LargeMotor) -> Result<(), Ev3Error> {
    motor_left.set_duty_cycle_sp(-100)?;
    motor_right.set_duty_cycle_sp(-100)?;

    thread::sleep(Duration::from_millis(500));

    Ok(())
}

pub fn search_for_ball(motor_left: &LargeMotor, motor_right: &LargeMotor) -> Result<(), Ev3Error> {
    motor_left.set_duty_cycle_sp(-50)?;
    motor_right.set_duty_cycle_sp(50)?;

    Ok(())
}

pub fn correction(
    error_value: i32,
    motor_diff: i32,
    motor_left: &LargeMotor,
    motor_right: &LargeMotor,
) -> Result<(), Ev3Error> {
    if error_value < 0 {
        motor_left.set_duty_cycle_sp(100 - motor_diff)?;
        motor_right.set_duty_cycle_sp(100)?;
    }
    if error_value > 0 {
        motor_left.set_duty_cycle_sp(100)?;
        motor_right.set_duty_cycle_sp(100 - motor_diff)?;
    }

    Ok(())
}
