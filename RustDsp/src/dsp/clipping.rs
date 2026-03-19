// Soft Clipping - Emulação do recorte simétrico de diodo suave do TS9
// Usa aproximação polinomial para simular comportamento de diodo

pub struct SoftClipping {
    /// Resistência série do op-amp (Ohms) - afeta a suavidade do clipping
    series_resistance: f32,
    /// Corrente de saturação do diodo (aproximado)
    saturation_current: f32,
    /// Voltage thermal (kT/q) em temperatura ambiente ~26mV
    thermal_voltage: f32,
}

impl SoftClipping {
    pub fn new() -> Self {
        SoftClipping {
            series_resistance: 10000.0, // 10k ohms típico
            saturation_current: 1e-14,  // Silício típico
            thermal_voltage: 0.026,     // ~26mV a 25°C
        }
    }

    /// Processa um sample aplicando soft clipping
    /// Usa aproximação polinomial cúbica para o comportamento de diodo
    pub fn process(&self, input: f32) -> f32 {
        // Hard clipping em valores extremos para evitar overflow
        if input.abs() > 10.0 {
            return input.signum() * 10.0;
        }

        // Usar função de transferência suave que emula dois diodos em série
        // Comportamento simétrico
        self.soft_clip_cubic(input)
    }

    /// Approximação cúbica polinomial do comportamento de diodo suave
    /// f(x) = a*x + b*x^3
    /// Suave e contínua com derivada contínua (importante para estabilidade)
    fn soft_clip_cubic(&self, x: f32) -> f32 {
        // Ganho aproximado baseado na resistência série
        let a = 1.0 / (1.0 + self.series_resistance / 1000.0);
        
        // Região linear - para valores pequenos, comporta-se quase linearmente
        if x.abs() < 0.5 {
            return a * x;
        }

        // Região de saturação - polinômio cúbico suave
        let x_normalized = x / 2.0; // Normaliza para intervalo [-0.25, 0.25]
        let cubic_term = x_normalized.powi(3) * 0.1;
        
        a * (x + cubic_term * x.abs()) / (1.0 + cubic_term.abs())
    }

    /// Alternativa: Aproximação da função de diodo Shockley (mais precisa)
    /// V = Vt * ln(I/Is + 1)
    /// Inverted: I = Is * (exp(V/Vt) - 1)
    #[allow(dead_code)]
    fn soft_clip_diode_shockley(&self, input: f32) -> f32 {
        let vt = self.thermal_voltage;
        let is = self.saturation_current;

        // Aproximação rápida da função exponencial para valores seguros
        let clipped_input = input.clamp(-0.5, 0.5);
        let exp_arg = clipped_input / vt;

        // Para evitar overflow, usar aproximação de série para exp
        if exp_arg > 20.0 {
            vt * (clipped_input / vt).ln()
        } else if exp_arg < -20.0 {
            -vt * 40.0 // Aproximação para exp negativa grande
        } else {
            vt * ((exp_arg).exp() - 1.0).ln()
        }
    }

    /// Soft clip com tanh - smooth mas computacionalmente eficiente
    /// Menos preciso fisicamente, mas bom para processamento em tempo real
    #[allow(dead_code)]
    fn soft_clip_tanh(&self, input: f32) -> f32 {
        // tanh(x) ≈ x para |x| < 0.1, e satura em ±1.0
        // Aplicar ganho de entrada para controlar ponto de saturação
        let gain = 3.0;
        (input * gain).tanh() / gain
    }
}

impl Default for SoftClipping {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soft_clip_symmetry() {
        let sc = SoftClipping::new();
        let input_pos = 0.5;
        let input_neg = -0.5;

        let output_pos = sc.process(input_pos);
        let output_neg = sc.process(input_neg);

        // Verifica simetria
        assert!((output_pos + output_neg).abs() < 1e-6);
    }

    #[test]
    fn test_soft_clip_prevents_overflow() {
        let sc = SoftClipping::new();
        let input = 100.0;
        let output = sc.process(input);

        assert!(output.abs() <= 10.0);
    }

    #[test]
    fn test_soft_clip_linear_region() {
        let sc = SoftClipping::new();
        let input = 0.1; // Região linear
        let output = sc.process(input);

        // Na região linear, saída deve ser bem próxima à entrada
        assert!((output - input).abs() < 0.01);
    }
}
