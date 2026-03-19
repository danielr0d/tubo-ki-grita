// Filtros para emulação TS9
// 1. Mid-Hump: Cria o pico característico dos médios (~720 Hz)
// 2. Tone Control: Filtro passa-baixas ajustável para controlar os agudos

/// Filtro biquad genérico IIR para uso em ambos os filtros
#[repr(C)]
pub struct BiquadFilter {
    // Coeficientes feedforward
    b0: f32,
    b1: f32,
    b2: f32,
    // Coeficientes feedback
    a1: f32,
    a2: f32,
    // Estados passados para processamento direto II
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BiquadFilter {
    /// Cria um novo filtro biquad com coeficientes inicializados
    fn new(b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) -> Self {
        BiquadFilter {
            b0,
            b1,
            b2,
            a1,
            a2,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Processa um sample usando estrutura Direct Form II
    fn process(&mut self, input: f32) -> f32 {
        // Variável intermediária
        let w = input - self.a1 * self.y1 - self.a2 * self.y2;

        // Saída
        let output = self.b0 * w + self.b1 * self.x1 + self.b2 * self.x2;

        // Atualizar estados
        self.x2 = self.x1;
        self.x1 = w;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }

    /// Reset dos estados
    fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }

    /// Cria um filtro passa-banda (peaking)
    /// Usado para o mid-hump
    fn peaking(sample_rate: f32, center_freq: f32, q: f32, gain_db: f32) -> Self {
        let w0 = 2.0 * std::f32::consts::PI * center_freq / sample_rate;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        let alpha = sin_w0 / (2.0 * q);
        let a_gain = 10_f32.powf(gain_db / 40.0);

        let b0 = 1.0 + alpha * a_gain;
        let b1 = -2.0 * cos_w0;
        let b2 = 1.0 - alpha * a_gain;
        let a0 = 1.0 + alpha / a_gain;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha / a_gain;

        BiquadFilter::new(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
    }

    /// Cria um filtro passa-baixas (lowpass)
    /// Usado para tone control
    fn lowpass(sample_rate: f32, cutoff_freq: f32, q: f32) -> Self {
        let w0 = 2.0 * std::f32::consts::PI * cutoff_freq / sample_rate;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        let alpha = sin_w0 / (2.0 * q);

        let b0 = (1.0 - cos_w0) / 2.0;
        let b1 = 1.0 - cos_w0;
        let b2 = (1.0 - cos_w0) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha;

        BiquadFilter::new(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
    }
}

// ============================================================================

/// Filtro Mid-Hump: Cria o pico característico do TS9 em ~720 Hz
/// Este é um dos segredos do timbre do TS9 - um pico de ressonância nos médios
#[repr(C)]
pub struct MidHumpFilter {
    // Dois filtros: um passa-alta seguido de um passa-baixa resulta em passa-banda
    high_pass: BiquadFilter,
    low_pass: BiquadFilter,
    peaking_filter: BiquadFilter,
}

impl MidHumpFilter {
    pub fn new(sample_rate: f32) -> Self {
        // High-pass em ~200 Hz para remover muito baixo
        let high_pass = BiquadFilter::lowpass(sample_rate, 200.0, 0.707);

        // Low-pass em ~4kHz para limpar muito agudo
        let low_pass = BiquadFilter::lowpass(sample_rate, 4000.0, 0.707);

        // Peaking em ~720 Hz com Q=2.0 para criar o "mid-hump" característico
        // Ganho em torno de 6-8dB para ressonância audível
        let peaking_filter = BiquadFilter::peaking(sample_rate, 720.0, 2.0, 7.0);

        MidHumpFilter {
            high_pass,
            low_pass,
            peaking_filter,
        }
    }

    /// Processa um bloco de áudio através do filtro
    pub fn process(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample = self.high_pass.process(*sample);
            *sample = self.low_pass.process(*sample);
            *sample = self.peaking_filter.process(*sample);
        }
    }
}

// ============================================================================

/// Filtro de Tone Control: Controle ajustável dos agudos (passa-baixas)
/// Baseado no circuito do TS9 - quando girado para o lado "dark", reduz agudos
#[repr(C)]
pub struct ToneControlFilter {
    sample_rate: f32,
    // Vamos usar múltiplos filtros para simular a resposta não-linear do circuito
    lowpass_stage1: BiquadFilter,
    lowpass_stage2: BiquadFilter,
}

impl ToneControlFilter {
    pub fn new(sample_rate: f32) -> Self {
        // Padrão: cutoff em ~4kHz (tone = 0.5)
        let cutoff = 4000.0;
        let q = 0.707; // Butterworth

        ToneControlFilter {
            sample_rate,
            lowpass_stage1: BiquadFilter::lowpass(sample_rate, cutoff, q),
            lowpass_stage2: BiquadFilter::lowpass(sample_rate, cutoff, q),
        }
    }

    /// Processa o buffer com tone control ajustável
    /// tone: 0.0 = muito escuro (cutoff baixo), 1.0 = muito brilhante (cutoff alto)
    pub fn process(&mut self, buffer: &mut [f32], tone: f32) {
        // Mapear parâmetro tone (0.0 - 1.0) para frequência de corte
        // Dark (tone=0.0) -> ~1.5kHz, Bright (tone=1.0) -> ~7kHz
        let min_freq = 1500.0;
        let max_freq = 7000.0;
        let cutoff_freq = min_freq + (max_freq - min_freq) * tone;

        // Recriar filtros com nova frequência de corte
        // (Em produção, seria melhor cache ou interpolação de coeficientes)
        let mut lp1 = BiquadFilter::lowpass(self.sample_rate, cutoff_freq, 0.707);
        let mut lp2 = BiquadFilter::lowpass(self.sample_rate, cutoff_freq, 0.707);

        // Processar com cascata de dois filtros para maior atenuação
        for sample in buffer.iter_mut() {
            *sample = lp1.process(*sample);
            *sample = lp2.process(*sample);
        }

        // Atualizar estado do filtro para próxima chamada
        self.lowpass_stage1 = lp1;
        self.lowpass_stage2 = lp2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biquad_stability() {
        // Criar um filtro é deve manter valores em range
        let mut lp = BiquadFilter::lowpass(44100.0, 1000.0, 0.707);
        let output = lp.process(1.0);
        assert!(output.is_finite());
        assert!(output.abs() < 10.0);
    }

    #[test]
    fn test_mid_hump_filter() {
        let mut mh = MidHumpFilter::new(44100.0);
        let mut buffer = vec![0.5; 100];
        mh.process(&mut buffer);

        // Verificar que nada explodiu
        for &sample in buffer.iter() {
            assert!(sample.is_finite());
        }
    }

    #[test]
    fn test_tone_control_tone_parameter() {
        let mut tc = ToneControlFilter::new(44100.0);
        let mut buffer_dark = vec![0.5; 100];
        let mut buffer_bright = vec![0.5; 100];

        tc.process(&mut buffer_dark, 0.0);  // Escuro
        tc.process(&mut buffer_bright, 1.0); // Brilhante

        // Buffer brilhante deve ter mais amplitude (menos atenuação)
        let amp_dark: f32 = buffer_dark.iter().map(|x| x.abs()).sum();
        let amp_bright: f32 = buffer_bright.iter().map(|x| x.abs()).sum();

        assert!(amp_bright > amp_dark);
    }
}
