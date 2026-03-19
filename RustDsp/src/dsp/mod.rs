// Módulo DSP - Exporta todos os submódulos
pub mod oversampler;
pub mod clipping;
pub mod filters;
pub mod gain_stage;

pub use oversampler::Oversampler2x;
pub use clipping::SoftClipping;
pub use filters::{MidHumpFilter, ToneControlFilter};
pub use gain_stage::GainStage;
