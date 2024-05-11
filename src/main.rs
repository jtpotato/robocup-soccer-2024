extern crate ev3dev_lang_rust;

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

    // Event Loop
    loop {
        if motor_left.is_stalled()? || motor_right.is_stalled()? {
            regimes::exit_stall(&motor_left, &motor_right)?;
        }

        let (has_ball, compass_dir, ball_sector) =
            sensors::read_sensors(&touch, &compass, &seeker)?;

        // Search for ball if it cannot be seen
        if ball_sector == 5 {
            regimes::search_for_ball(&motor_left, &motor_right)?;
            continue;
        }

        if has_ball {
            regimes::follow_compass(compass_dir, &motor_left, &motor_right)?;
            continue;
        }

        if ball_sector.abs() > 1 {
            regimes::large_correction(ball_sector, &motor_left, &motor_right)?;
            continue;
        }

        regimes::approach_ball(ball_sector, &motor_left, &motor_right)?;
    }
}
