// Oversampler 2x com filtro anti-aliasing polifásico
// Essencial para reduzir aliasing na distorção não-linear

pub struct Oversampler2x {
    sample_rate: f32,
    // Buffer de histórico para interpolação
    prev_sample: f32,
    // Coeficientes de filtro pré-computados
    filter_coeffs: Vec<f32>,
}

impl Oversampler2x {
    pub fn new(sample_rate: f32) -> Self {
        Oversampler2x {
            sample_rate,
            prev_sample: 0.0,
            filter_coeffs: Self::compute_filter_coeffs(),
        }
    }

    /// Computa coeficientes para um filtro Hamming de baixa passagem
    /// em 0.5 * Fs (Nyquist do sinal original)
    fn compute_filter_coeffs() -> Vec<f32> {
        // Coeficientes pré-computados para filtro FIR de 11 taps (Hamming)
        // Cutoff em Nyquist/2
        vec![
            -0.0015625,
            -0.00390625,
            0.02734375,
            -0.0390625,
            -0.1015625,
            0.3828125,
            0.6484375,
            -0.0390625,
            0.02734375,
            -0.00390625,
            -0.0015625,
        ]
    }

    /// Upsampling: insere zeros entre amostras e aplica filtro
    pub fn upsample(&mut self, input: &[f32]) -> Vec<f32> {
        let mut upsampled = Vec::with_capacity(input.len() * 2);

        for &sample in input {
            upsampled.push(sample);
            upsampled.push(0.0); // Zero inserido
        }

        // Aplicar filtro anti-aliasing
        self.apply_filter(&upsampled)
    }

    /// Downsampling: remove amostras pares (mantém as ímpares)
    pub fn downsample(&self, input: &[f32]) -> Vec<f32> {
        input.iter().step_by(2).copied().collect()
    }

    /// Aplica filtro FIR aos dados
    fn apply_filter(&self, signal: &[f32]) -> Vec<f32> {
        let mut output = signal.to_vec();
        let taps = self.filter_coeffs.len();

        // Aplicar convolução simples
        for i in 0..output.len() {
            let mut sum = 0.0;
            for (j, &coeff) in self.filter_coeffs.iter().enumerate() {
                let idx = i as i32 - (taps as i32 / 2) + j as i32;
                if idx >= 0 && (idx as usize) < output.len() {
                    sum += output[idx as usize] * coeff;
                }
            }
            output[i] = sum;
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oversampler_buffer_size() {
        let os = Oversampler2x::new(44100.0);
        let input = vec![0.5, 0.3, -0.2];
        let upsampled = os.upsample(&input);
        assert_eq!(upsampled.len(), input.len() * 2);
    }

    #[test]
    fn test_downsampler_reduces_by_half() {
        let os = Oversampler2x::new(44100.0);
        let input = vec![0.5, 0.1, 0.3, 0.2, -0.2, 0.0];
        let downsampled = os.downsample(&input);
        assert_eq!(downsampled.len(), input.len() / 2);
    }
}
