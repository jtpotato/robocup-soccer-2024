use ev3dev_lang_rust::{
    sensors::{CompassSensor, IrSeekerSensor, SensorPort, TouchSensor},
    Ev3Result,
};

pub fn get_sensors() -> Ev3Result<(TouchSensor, CompassSensor, IrSeekerSensor)> {
    let touch_sensor = TouchSensor::get(SensorPort::In2)?;
    let mut compass_sensor = CompassSensor::get(SensorPort::In4)?;
    let ir_sensor = IrSeekerSensor::get(SensorPort::In1)?;

    compass_sensor.set_zero()?;
    ir_sensor.set_mode_ac()?; // super important

    Ok((touch_sensor, compass_sensor, ir_sensor))
}

pub fn read_sensors(
    touch_sensor: &TouchSensor,
    compass_sensor: &CompassSensor,
    ir_sensor: &IrSeekerSensor,
) -> Ev3Result<(bool, i32, i32)> {
    // Change the return type to Ev3Result<(bool, i32, i32)>
    let has_ball = touch_sensor.get_pressed_state()?;

    let mut compass_dir = compass_sensor.get_relative_rotation()?;
    if compass_dir > 180 {
        compass_dir -= 360;
    }

    let ball_sector = 5 - ir_sensor.get_ir_direction()?;

    return Ok((has_ball, compass_dir, ball_sector));
}
