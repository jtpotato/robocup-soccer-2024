extern crate ev3dev_lang_rust;

use std::thread;
use std::time::Duration;

use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::Ev3Result;
use sensors::get_sensors;

mod regimes;
mod sensors;

fn main() -> Ev3Result<()> {
    let motor_left = LargeMotor::get(MotorPort::OutA)?;
    let motor_right = LargeMotor::get(MotorPort::OutD)?;

    let (touch, compass, seeker) = get_sensors()?;

    // Set command "run-direct".
    motor_left.run_direct()?;
    motor_right.run_direct()?;
    // force library to open file handler to write instructions to motor.
    motor_left.set_duty_cycle_sp(0)?;
    motor_right.set_duty_cycle_sp(0)?;

    // Event Loop
    loop {
        thread::sleep(Duration::from_millis(1)); // apparently, this is necessary to allow linux to do background stuff.

        if motor_left.is_stalled()? || motor_right.is_stalled()? {
            regimes::exit_stall(&motor_left, &motor_right)?;
        }

        let (has_ball, compass_dir, ball_sector) =
            sensors::read_sensors(&touch, &compass, &seeker)?;

        if ball_sector == 5 {
            regimes::search_for_ball(&motor_left, &motor_right)?;
            continue;
        }

        if has_ball {
            regimes::correction(compass_dir, 30, &motor_left, &motor_right)?;
            continue;
        }

        if ball_sector.abs() > 1 {
            regimes::correction(ball_sector, 100, &motor_left, &motor_right)?;
            continue;
        }

        regimes::correction(ball_sector, 30, &motor_left, &motor_right)?;
    }
}
