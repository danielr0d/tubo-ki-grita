// TS9 Tube Screamer Emulation - DSP Core
// FFI Interface para integração com C++ (JUCE)

pub mod dsp;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use dsp::oversampler::Oversampler2x;
use dsp::clipping::SoftClipping;
use dsp::filters::{MidHumpFilter, ToneControlFilter};
use dsp::gain_stage::GainStage;

/// Contexto DSP principal do TS9
/// Mantém o estado de todos os estágios de processamento
#[repr(C)]
pub struct Ts9DspContext {
    sample_rate: f32,
    /// Oversampler para o estágio de clipping
    oversampler: Oversampler2x,
    /// Soft clipping com modelagem de diodo suave
    clipping: SoftClipping,
    /// Filtro que cria o característico "mid-hump" do TS9
    mid_hump_filter: MidHumpFilter,
    /// Filtro de controle de tom (tone control)
    tone_filter: ToneControlFilter,
    /// Gerenciador de ganho (Drive e Level)
    gain_stage: GainStage,
    /// Flag para bypass do efeito
    is_bypassed: Arc<AtomicBool>,
}

impl Ts9DspContext {
    fn new(sample_rate: f32) -> Self {
        Ts9DspContext {
            sample_rate,
            oversampler: Oversampler2x::new(sample_rate),
            clipping: SoftClipping::new(),
            mid_hump_filter: MidHumpFilter::new(sample_rate),
            tone_filter: ToneControlFilter::new(sample_rate),
            gain_stage: GainStage::new(),
            is_bypassed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Processa um bloco de áudio mono através de todo o pipeline DSP
    fn process_block(
        &mut self,
        input: &[f32],
        output: &mut [f32],
        drive: f32,
        tone: f32,
        level: f32,
        is_bypassed: bool,
    ) {
        if is_bypassed {
            output.copy_from_slice(input);
            return;
        }

        // Estágio 1: Ganho de entrada (Drive)
        let mut stage1: Vec<f32> = input
            .iter()
            .map(|&s| self.gain_stage.apply_drive(s, drive))
            .collect();

        // Estágio 2: Filtro Mid-Hump (simula o pré-filtro do TS9)
        self.mid_hump_filter.process(&mut stage1);

        // Estágio 3: Soft Clipping com Oversampling
        // Upsample -> Clip -> Downsample
        let oversampled = self.oversampler.upsample(&stage1);
        let mut clipped: Vec<f32> = oversampled
            .iter()
            .map(|&s| self.clipping.process(s))
            .collect();
        let mut downsampled = self.oversampler.downsample(&clipped);

        // Estágio 4: Filtro de Tone Control
        self.tone_filter.process(&mut downsampled, tone);

        // Estágio 5: Ganho de saída (Level)
        for (i, &s) in downsampled.iter().enumerate() {
            output[i] = self.gain_stage.apply_level(s, level);
        }
    }
}

// ============================================================================
// FFI - Foreign Function Interface para C++
// ============================================================================

/// Cria um novo contexto DSP
/// # Arguments
/// * `sample_rate` - Taxa de amostragem em Hz (e.g., 44100, 48000)
/// # Returns
/// Ponteiro para o contexto DSP alocado
#[no_mangle]
pub extern "C" fn create_ts9_dsp(sample_rate: f32) -> *mut Ts9DspContext {
    let ctx = Box::new(Ts9DspContext::new(sample_rate));
    Box::into_raw(ctx)
}

/// Destroi um contexto DSP e libera sua memória
/// # Arguments
/// * `dsp` - Ponteiro para o contexto a ser destruído
#[no_mangle]
pub extern "C" fn destroy_ts9_dsp(dsp: *mut Ts9DspContext) {
    if !dsp.is_null() {
        unsafe {
            let _ = Box::from_raw(dsp);
        }
    }
}

/// Processa um bloco de áudio
/// # Arguments
/// * `dsp` - Contexto DSP
/// * `inputs` - Ponteiro para array de ponteiros de entrada (canais)
/// * `outputs` - Ponteiro para array de ponteiros de saída (canais)
/// * `num_channels` - Número de canais (1 = mono, 2 = estéreo, etc.)
/// * `num_samples` - Número de amostras por canal
/// * `drive` - Parâmetro Drive (0.0 - 1.0)
/// * `tone` - Parâmetro Tone (0.0 - 1.0)
/// * `level` - Parâmetro Level (0.0 - 1.0)
/// * `is_bypassed` - Se true, copia entrada para saída sem processamento
#[no_mangle]
pub extern "C" fn process_ts9_block(
    dsp: *mut Ts9DspContext,
    inputs: *const *const f32,
    outputs: *const *mut f32,
    num_channels: i32,
    num_samples: i32,
    drive: f32,
    tone: f32,
    level: f32,
    is_bypassed: bool,
) {
    if dsp.is_null() || inputs.is_null() || outputs.is_null() {
        return;
    }

    unsafe {
        let dsp = &mut *dsp;
        let inputs_slice = std::slice::from_raw_parts(inputs, num_channels as usize);
        let outputs_slice = std::slice::from_raw_parts_mut(outputs, num_channels as usize);
        let num_samples = num_samples as usize;

        // Processar cada canal
        for ch in 0..num_channels as usize {
            let input = std::slice::from_raw_parts(inputs_slice[ch], num_samples);
            let output = std::slice::from_raw_parts_mut(outputs_slice[ch], num_samples);

            dsp.process_block(input, output, drive, tone, level, is_bypassed);
        }
    }
}

/// Atualiza a taxa de amostragem (para reconexão ou mudanças de projeto)
#[no_mangle]
pub extern "C" fn set_sample_rate(dsp: *mut Ts9DspContext, sample_rate: f32) {
    if !dsp.is_null() {
        unsafe {
            let dsp = &mut *dsp;
            dsp.sample_rate = sample_rate;
            dsp.mid_hump_filter = dsp::filters::MidHumpFilter::new(sample_rate);
            dsp.tone_filter = dsp::filters::ToneControlFilter::new(sample_rate);
        }
    }
}
