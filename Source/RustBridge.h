#pragma once

#include <cstdint>
#include <cstddef>

// ============================================================================
// FFI Bridge para o módulo DSP Rust
// Declarações C-compatíveis das funções exportadas pelo Rust
// ============================================================================

#ifdef __cplusplus
extern "C" {
#endif

/// Opaque type para o contexto DSP
typedef struct Ts9DspContext Ts9DspContext;

/// Cria um novo contexto DSP
/// @param sample_rate - Taxa de amostragem em Hz (e.g., 44100, 48000)
/// @return Ponteiro para o contexto DSP alocado
Ts9DspContext* create_ts9_dsp(float sample_rate);

/// Destroi um contexto DSP e libera sua memória
/// @param dsp - Ponteiro para o contexto a ser destruído
void destroy_ts9_dsp(Ts9DspContext* dsp);

/// Processa um bloco de áudio através do pipeline DSP
/// @param dsp - Contexto DSP
/// @param inputs - Ponteiro para array de ponteiros de entrada (canais)
/// @param outputs - Ponteiro para array de ponteiros de saída (canais)
/// @param num_channels - Número de canais (1 = mono, 2 = estéreo, etc.)
/// @param num_samples - Número de amostras por canal
/// @param drive - Parâmetro Drive (0.0 - 1.0) - ganho de entrada
/// @param tone - Parâmetro Tone (0.0 - 1.0) - controle de tons
/// @param level - Parâmetro Level (0.0 - 1.0) - ganho de saída
/// @param is_bypassed - Se true, copia entrada para saída sem processamento
void process_ts9_block(
    Ts9DspContext* dsp,
    const float** inputs,
    float** outputs,
    int num_channels,
    int num_samples,
    float drive,
    float tone,
    float level,
    bool is_bypassed
);

/// Atualiza a taxa de amostragem (para reconexão ou mudanças de projeto)
/// @param dsp - Contexto DSP
/// @param sample_rate - Nova taxa de amostragem em Hz
void set_sample_rate(Ts9DspContext* dsp, float sample_rate);

#ifdef __cplusplus
}
#endif
