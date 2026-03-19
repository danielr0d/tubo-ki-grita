// Estágio de Ganho: Controla Drive (ganho de entrada) e Level (ganho de saída)

pub struct GainStage {
    /// Ganho de entrada mínimo (quando Drive = 0.0)
    min_input_gain: f32,
    /// Ganho de entrada máximo (quando Drive = 1.0)
    max_input_gain: f32,
    /// Ganho de saída mínimo (quando Level = 0.0)
    min_output_gain: f32,
    /// Ganho de saída máximo (quando Level = 1.0)
    max_output_gain: f32,
}

impl GainStage {
    pub fn new() -> Self {
        GainStage {
            // Drive: 0.5 (0dB) a 50 (34dB) - emula o ganho do op-amp com resistências
            min_input_gain: 0.5,   // ~-6dB
            max_input_gain: 50.0,  // ~34dB
            // Level: 0.1 (-20dB) a 1.0 (0dB) - controle de volume de saída
            min_output_gain: 0.1,
            max_output_gain: 1.0,
        }
    }

    /// Aplica ganho de entrada (Drive parameter)
    /// drive: 0.0 - 1.0
    pub fn apply_drive(&self, input: f32, drive: f32) -> f32 {
        // Mapear parâmetro drive (0.0 - 1.0) para ganho exponencial
        // Usar escala exponencial para sentir mais natural
        let drive_clamped = drive.clamp(0.0, 1.0);
        let gain = self.min_input_gain
            + (self.max_input_gain - self.min_input_gain) * drive_clamped.powi(2);

        (input * gain).clamp(-1.0, 1.0)
    }

    /// Aplica ganho de saída (Level parameter)
    /// level: 0.0 - 1.0
    pub fn apply_level(&self, input: f32, level: f32) -> f32 {
        let level_clamped = level.clamp(0.0, 1.0);
        let gain = self.min_output_gain
            + (self.max_output_gain - self.min_output_gain) * level_clamped;

        (input * gain).clamp(-1.0, 1.0)
    }

    /// Aplica ambos ganhos em sequência
    pub fn apply_both(&self, input: f32, drive: f32, level: f32) -> f32 {
        self.apply_level(self.apply_drive(input, drive), level)
    }
}

impl Default for GainStage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drive_increases_with_parameter() {
        let gs = GainStage::new();
        let input = 0.1;
        let output_0 = gs.apply_drive(input, 0.0);
        let output_1 = gs.apply_drive(input, 1.0);

        assert!(output_1 > output_0);
    }

    #[test]
    fn test_level_increases_with_parameter() {
        let gs = GainStage::new();
        let input = 0.1;
        let output_0 = gs.apply_level(input, 0.0);
        let output_1 = gs.apply_level(input, 1.0);

        assert!(output_1 > output_0);
    }

    #[test]
    fn test_no_clipping_on_extreme_drive() {
        let gs = GainStage::new();
        let input = 1.0;
        let output = gs.apply_drive(input, 1.0);

        assert!(output.abs() <= 1.0);
    }

    #[test]
    fn test_gain_range() {
        let gs = GainStage::new();
        
        // Verificar que apply_drive sempre retorna valores entre -1.0 e 1.0
        for i in 0..=10 {
            let drive = i as f32 / 10.0;
            let output = gs.apply_drive(0.5, drive);
            assert!(output >= -1.0 && output <= 1.0);
        }
    }
}
