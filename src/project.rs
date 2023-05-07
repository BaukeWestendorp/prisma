use crate::effect::{hydrate_effect, EffectWrapper};
use crate::runner::Runner;

#[derive(Debug, Clone)]
pub struct Project {
    pub framerate: usize,
    pub global_bpm: f32,
    pub cues: Vec<Cue>,
}

impl Project {
    pub fn hydrate(&self, runner: &mut Runner) {
        for cue in self.cues.iter() {
            cue.hydrate(runner);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cue {
    pub effects: Vec<EffectWrapper>,
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

impl Cue {
    pub(crate) fn hydrate(&self, runner: &mut Runner) {
        for effect_wrapper in self.effects.iter() {
            let cycle_context = CycleContext::new(runner, effect_wrapper);

            let mut effect_leds = Vec::new();
            runner.leds.clone_into(&mut effect_leds);
            hydrate_effect(&effect_wrapper.effect, cycle_context, &mut effect_leds);

            for (i, color) in runner.leds.iter_mut().enumerate() {
                color.blend_with(&effect_leds[i]);
            }
        }
    }
}
