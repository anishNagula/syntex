use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PatchNode {

    Sine {
        frequency: f32,
    },

    Square {
        frequency: f32,
    },

    LowPass {
        input: Box<PatchNode>,
        alpha: f32,
    },

    Mixer {
        inputs: Vec<PatchNode>,
    },

    Gain {
        input: Box<PatchNode>,
        gain: f32,
    },
}
