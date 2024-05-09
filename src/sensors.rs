use ev3dev_lang_rust::{
    sensors::{CompassSensor, IrSeekerSensor, TouchSensor},
    Ev3Result,
};

pub fn read_sensors(
    touch_sensor: &TouchSensor,
    compass_sensor: &CompassSensor,
    ir_sensor: &IrSeekerSensor,
) -> Ev3Result<(bool, i32, i32)> {
    // Change the return type to Ev3Result<(bool, i32, i32)>
    let has_ball = touch_sensor.get_pressed_state()?;
    let compass_dir = compass_sensor.get_relative_rotation()?;
    let ball_sector = 5 - ir_sensor.get_ir_direction()?;
    return Ok((has_ball, compass_dir, ball_sector));
}
