use crate::color::Color;
use crate::effect::{hydrate_effect, Effect, EffectWrapper, LedRange};
use crate::runner::Runner;

#[derive(Debug, Clone)]
pub struct Project {
    pub framerate: usize,
    pub global_bpm: f32,
    pub effects: Vec<EffectWrapper>,
}

impl Project {
    pub fn hydrate(&mut self, runner: &mut Runner) {
        let base_effects = vec![EffectWrapper {
            bpm_factor: 1.0,
            range: LedRange {
                min: 0,
                max: runner.project.led_count(),
            },
            effect: Effect::StaticColor {
                color: Color::black(),
            },
        }];

        let mut effect_wrappers = base_effects;
        effect_wrappers.append(&mut self.effects);

        for effect_wrapper in effect_wrappers.iter() {
            let cycle_context = CycleContext::new(runner, effect_wrapper);

            let mut effect_leds = Vec::new();
            runner.leds[effect_wrapper.range.min..effect_wrapper.range.max]
                .clone_into(&mut effect_leds);
            hydrate_effect(&effect_wrapper.effect, cycle_context, &mut effect_leds);

            for (i, color) in runner.leds[effect_wrapper.range.min..effect_wrapper.range.max]
                .iter_mut()
                .enumerate()
            {
                color.blend_with(&effect_leds[i]);
            }
        }
    }

    pub fn led_count(&self) -> usize {
        let mut max = 0;
        for effect in self.effects.iter() {
            if max < effect.range.max {
                max = effect.range.max
            }
        }
        max
    }
}

#[derive(Debug, Clone)]
pub struct CycleContext {
    pub frame: usize,
    pub measure_progress: f32,
    pub is_first_frame_of_measure: bool,
}

impl CycleContext {
    fn new(runner: &Runner, effect_wrapper: &EffectWrapper) -> Self {
        let current_millis = runner.start_time.elapsed().as_millis();

        let millis_per_measure = 60_000.0 / (runner.project.global_bpm * effect_wrapper.bpm_factor);
        let millis_per_frame = 1000.0 / runner.project.framerate as f32;
        let measure_progress = (current_millis as f64 / millis_per_measure as f64) % 1.0;
        let is_first_frame_of_measure =
            measure_progress * millis_per_measure as f64 <= millis_per_frame as f64;

        Self {
            frame: runner.frame,
            measure_progress: measure_progress as f32,
            is_first_frame_of_measure,
        }
    }
}
