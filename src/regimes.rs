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

pub fn large_correction(
    ball_sector: i32,
    motor_left: &LargeMotor,
    motor_right: &LargeMotor,
) -> Result<(), Ev3Error> {
    if ball_sector < 0 {
        motor_left.set_duty_cycle_sp(0)?;
        motor_right.set_duty_cycle_sp(100)?;
    }
    if ball_sector > 0 {
        motor_left.set_duty_cycle_sp(100)?;
        motor_right.set_duty_cycle_sp(0)?;
    }

    Ok(())
}

pub fn follow_compass(
    compass_angle: i32,
    motor_left: &LargeMotor,
    motor_right: &LargeMotor,
) -> Result<(), Ev3Error> {
    if compass_angle < -10 {
        motor_left.set_duty_cycle_sp(70)?;
        motor_right.set_duty_cycle_sp(100)?;
        return Ok(());
    }
    if compass_angle > 10 {
        motor_left.set_duty_cycle_sp(100)?;
        motor_right.set_duty_cycle_sp(70)?;
        return Ok(());
    }
    motor_left.set_duty_cycle_sp(100)?;
    motor_right.set_duty_cycle_sp(100)?;

    Ok(())
}

pub fn approach_ball(
    ball_sector: i32,
    motor_left: &LargeMotor,
    motor_right: &LargeMotor,
) -> Result<(), Ev3Error> {
    if ball_sector < 0 {
        motor_left.set_duty_cycle_sp(70)?;
        motor_right.set_duty_cycle_sp(100)?;
        return Ok(()); // early returns.
    }
    if ball_sector > 0 {
        motor_left.set_duty_cycle_sp(100)?;
        motor_right.set_duty_cycle_sp(70)?;
        return Ok(());
    }

    motor_left.set_duty_cycle_sp(100)?;
    motor_right.set_duty_cycle_sp(100)?;

    Ok(())
}
