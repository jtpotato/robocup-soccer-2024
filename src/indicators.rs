use ev3dev_lang_rust::{Ev3Error, Led};

/// The `SensingRegime` enum represents the different states of the robot's sensing regime.
#[derive(PartialEq)]
pub enum SensingRegime {
    NoBall,
    MovingToBall,
    HasBall,
}

/// The `Indicator` struct represents the LED indicator of the robot.
pub struct Indicator {
    current_regime: SensingRegime,
    led: Led,
}

impl Indicator {
    /// Update the LED indicator according to the current sensing regime.
    pub fn update_indicators(&self, regime: SensingRegime) -> Result<(), Ev3Error> {
        if self.current_regime == regime {
            return Ok(());
        }

        match regime {
            SensingRegime::NoBall => {
                self.led.set_color(Led::COLOR_RED)?;
            }
            SensingRegime::MovingToBall => {
                self.led.set_color(Led::COLOR_AMBER)?;
            }
            SensingRegime::HasBall => {
                self.led.set_color(Led::COLOR_GREEN)?;
            }
        }

        return Ok(());
    }
}

/// Create a new `Indicator` with the default sensing regime set to `SensingRegime::NoBall`.
pub fn default_indicator(led: Led) -> Indicator {
    Indicator {
        current_regime: SensingRegime::NoBall,
        led,
    }
}
