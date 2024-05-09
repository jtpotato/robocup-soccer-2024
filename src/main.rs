extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::Ev3Result;
use sensors::get_sensors;

mod sensors;

fn main() -> Ev3Result<()> {
    let motor_left = LargeMotor::get(MotorPort::OutA)?;
    let motor_right = LargeMotor::get(MotorPort::OutD)?;

    let (touch, compass, seeker) = get_sensors()?;

    // Set command "run-direct".
    motor_left.run_direct()?;
    motor_right.run_direct()?;

    // Event Loop
    loop {
        let (has_ball, compass_dir, ball_sector) =
            sensors::read_sensors(&touch, &compass, &seeker)?;

        // Search for ball if it cannot be seen
        if ball_sector == 5 {
            motor_left.set_duty_cycle_sp(-50)?;
            motor_right.set_duty_cycle_sp(50)?;
            continue;
        }

        // Ball is very left
        if ball_sector < -1 {
            motor_left.set_duty_cycle_sp(100)?;
            motor_right.set_duty_cycle_sp(0)?;
            continue;
        }
        // Ball is very right
        if ball_sector > 1 {
            motor_left.set_duty_cycle_sp(0)?;
            motor_right.set_duty_cycle_sp(100)?;
            continue;
        }

        if has_ball {
            if ball_sector == -1 {
                // left
                motor_left.set_duty_cycle_sp(100)?;
                motor_right.set_duty_cycle_sp(90)?;
                continue;
            }
            if ball_sector == 1 {
                // right
                motor_left.set_duty_cycle_sp(90)?;
                motor_right.set_duty_cycle_sp(100)?;
                continue;
            }
            if ball_sector == 0 {
                if compass_dir < -30 {
                    motor_left.set_duty_cycle_sp(100)?;
                    motor_right.set_duty_cycle_sp(95)?;
                    continue;
                }
                if compass_dir > 30 {
                    motor_left.set_duty_cycle_sp(95)?;
                    motor_right.set_duty_cycle_sp(100)?;
                    continue;
                }
                motor_left.set_duty_cycle_sp(100)?;
                motor_right.set_duty_cycle_sp(100)?;
                continue;
            }
        }

        // Approach ball
        if ball_sector == 0 {
            motor_left.set_duty_cycle_sp(100)?;
            motor_right.set_duty_cycle_sp(100)?;
            continue;
        }

        if ball_sector == -1 {
            motor_left.set_duty_cycle_sp(100)?;
            motor_right.set_duty_cycle_sp(50)?;
        }
        if ball_sector == 1 {
            motor_left.set_duty_cycle_sp(50)?;
            motor_right.set_duty_cycle_sp(100)?;
        }
    }
}
