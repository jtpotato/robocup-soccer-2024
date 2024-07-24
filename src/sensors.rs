use ev3dev_lang_rust::{
    sensors::{ColorSensor, CompassSensor, IrSeekerSensor, SensorPort},
    Ev3Result,
};

pub fn get_sensors() -> Ev3Result<(ColorSensor, CompassSensor, IrSeekerSensor)> {
    let col_sensor = ColorSensor::get(SensorPort::In3)?;
    let mut compass_sensor = CompassSensor::get(SensorPort::In4)?;
    let ir_sensor = IrSeekerSensor::get(SensorPort::In1)?;

    col_sensor.set_mode_col_reflect()?;

    compass_sensor.set_zero()?;
    ir_sensor.set_mode_ac()?; // super important

    Ok((col_sensor, compass_sensor, ir_sensor))
}

pub fn read_sensors(
    col_sensor: &ColorSensor,
    prev_avg_col_value: &mut i32,
    compass_sensor: &CompassSensor,
    ir_sensor: &IrSeekerSensor,
) -> Ev3Result<(bool, i32, i32)> {
    let mut compass_dir = compass_sensor.get_relative_rotation()?;
    if compass_dir > 180 {
        compass_dir -= 360;
    }

    let col_value = col_sensor.get_color()?;

    // calculate average colour value
    let avg_col_value = (0.999 * (*prev_avg_col_value as f32) + 0.001 * col_value as f32) as i32;
    *prev_avg_col_value = avg_col_value;

    let ball_sector = 5 - ir_sensor.get_ir_direction()?;
    let has_ball = avg_col_value > 10;

    return Ok((has_ball, compass_dir, ball_sector));
}
