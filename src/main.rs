extern crate ev3dev_lang_rust;

use std::thread;
use std::time::Duration;

use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::{Ev3Result, Led};
use indicators::{default_indicator, SensingRegime};
use sensors::get_sensors;

mod indicators;
mod regimes;
mod sensors;

fn main() -> Ev3Result<()> {
    let motor_left = LargeMotor::get(MotorPort::OutA)?;
    let motor_right = LargeMotor::get(MotorPort::OutD)?;

    let (col_sensor, compass, seeker) = get_sensors()?;

    // Set command "run-direct".
    motor_left.run_direct()?;
    motor_right.run_direct()?;
    // force library to open file handler to write instructions to motor.
    motor_left.set_duty_cycle_sp(0)?;
    motor_right.set_duty_cycle_sp(0)?;

    let indicator = default_indicator(Led::new()?);
    let mut prev_avg_col = 0;

    // Event Loop
    loop {
        thread::sleep(Duration::from_millis(1)); // apparently, this is necessary to allow linux to do background stuff.

        let (has_ball, compass_dir, ball_sector) =
            sensors::read_sensors(&col_sensor, &mut prev_avg_col, &compass, &seeker)?;

        if ball_sector == 5 {
            indicator.update_indicators(SensingRegime::NoBall)?;
            regimes::search_for_ball(&motor_left, &motor_right)?;
            continue;
        }

        if has_ball {
            indicator.update_indicators(SensingRegime::HasBall)?;
            if ball_sector == 0 {
                regimes::correction(compass_dir, 20, 90, &motor_left, &motor_right)?;
                continue;
            }
            regimes::correction(ball_sector, 10, 100, &motor_left, &motor_right)?;
            continue;
        }

        indicator.update_indicators(SensingRegime::MovingToBall)?;

        if ball_sector.abs() > 2 {
            regimes::correction(ball_sector, 200, 100, &motor_left, &motor_right)?;
            continue;
        }

        if ball_sector.abs() > 1 {
            regimes::correction(ball_sector, 60, 80, &motor_left, &motor_right)?;
            continue;
        }

        regimes::correction(ball_sector, 5, 80, &motor_left, &motor_right)?;
    }
}
