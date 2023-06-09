use serde::{Deserialize, Serialize};

use crate::color::Color;
use crate::project::CycleContext;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WaveType {
    Sine,
    Square { pulse_width: f32 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Effect {
    StaticColor {
        color: Color,
    },
    Strobe {
        color: Color,
        pulse_width: f32,
    },
    Wave {
        color: Color,
        wave_type: WaveType,
        repeats: f32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LedRange {
    pub min: usize,
    pub max: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectLayer {
    pub bpm_factor: f32,
    pub range: LedRange,
    pub effect: Effect,
}

impl EffectLayer {
    pub fn new(bpm_factor: f32, range: LedRange, effect: Effect) -> Self {
        Self {
            bpm_factor,
            range,
            effect,
        }
    }
}

pub(crate) fn hydrate_effect(effect: &Effect, cycle_context: CycleContext, leds: &mut [Color]) {
    match effect {
        Effect::StaticColor { color } => {
            for led in leds.iter_mut() {
                *led = *color;
            }
        }
        Effect::Strobe { color, pulse_width } => {
            for led in leds.iter_mut() {
                if cycle_context.is_first_frame_of_measure {
                    *led = *color;
                    continue;
                }

                if cycle_context.measure_progress < *pulse_width {
                    *led = *color;
                    continue;
                }
            }
        }
        Effect::Wave {
            color,
            wave_type,
            repeats,
        } => {
            let led_count = leds.len();
            for (i, led) in leds.iter_mut().enumerate() {
                let t = cycle_context.measure_progress + i as f32 / led_count as f32;
                let mut factor = ((t * std::f32::consts::PI * 2.0 * *repeats).cos() + 1.0) / 2.0;

                if let WaveType::Square { pulse_width } = *wave_type {
                    factor = match factor >= pulse_width {
                        true => 1.0,
                        false => 0.0,
                    }
                };

                let mut new_color = *color;
                new_color.alpha -= factor;
                *led = new_color;
            }
        }
    }
}
