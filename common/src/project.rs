use crate::color::Color;
use crate::effect::{hydrate_effect, Effect, EffectLayer, LedRange};
use crate::runner::Runner;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub framerate: usize,
    pub global_bpm: f32,
    pub effect_layers: Vec<EffectLayer>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            framerate: 50,
            global_bpm: 60.0,
            effect_layers: vec![],
        }
    }
}

impl Project {
    pub fn hydrate(&mut self, runner: &mut Runner) {
        runner.clear_leds();

        let base_effects = vec![EffectLayer {
            bpm_factor: 1.0,
            range: LedRange {
                min: 0,
                max: runner.project.led_count(),
            },
            effect: Effect::StaticColor {
                color: Color::black(),
            },
        }];

        let mut layers = base_effects;
        layers.append(&mut self.effect_layers);

        for layer in layers.iter() {
            let cycle_context = CycleContext::new(runner, layer);

            let mut effect_leds = Vec::new();
            runner.leds[layer.range.min..layer.range.max].clone_into(&mut effect_leds);
            hydrate_effect(&layer.effect, cycle_context, &mut effect_leds);

            for (i, color) in runner.leds[layer.range.min..layer.range.max]
                .iter_mut()
                .enumerate()
            {
                color.blend_with(&effect_leds[i]);
            }
        }
    }

    pub fn led_count(&self) -> usize {
        let mut max = 0;
        for effect in self.effect_layers.iter() {
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
    fn new(runner: &Runner, layer: &EffectLayer) -> Self {
        let current_millis = runner.start_time.elapsed().as_millis();

        let millis_per_measure = 60_000.0 / (runner.project.global_bpm * layer.bpm_factor);
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
