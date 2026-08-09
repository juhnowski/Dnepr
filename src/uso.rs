pub const ADC_CHANNELS: usize = 8;
pub const DAC_CHANNELS: usize = 4;

/// Модуль УСО (Устройство связи с объектом)
pub struct PeripheralUso {
    pub adc_inputs: [f64; ADC_CHANNELS],
    pub dac_outputs: [f64; DAC_CHANNELS],
    pub selected_channel: usize,
}

impl PeripheralUso {
    pub fn new() -> Self {
        Self {
            adc_inputs: [0.0; ADC_CHANNELS],
            dac_outputs: [0.0; DAC_CHANNELS],
            selected_channel: 0,
        }
    }
}
