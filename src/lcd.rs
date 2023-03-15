use cortex_m::delay::Delay;
use stm32_hal2::{
    adc::{Adc, AdcConfig, AdcDevice},
    clocks::Clocks,
    gpio::{Pin, PinMode, PinState, Port},
    pac::ADC2,
};

pub struct LiquidCrystal {
    d7: Pin,
    d6: Pin,
    d5: Pin,
    d4: Pin,
    rs: Pin,
    en: Pin,
}

impl LiquidCrystal {
    pub fn new(d7: Pin, d6: Pin, d5: Pin, d4: Pin, rs: Pin, en: Pin) -> Self {
        Self {
            d7,
            d6,
            d5,
            d4,
            rs,
            en,
        }
    }

    fn send_to_lcd(&mut self, data: u8, rs: PinState, delay: &mut Delay) {
        self.rs.set_state(rs);

        // Sending Data
        let pins = [&mut self.d7, &mut self.d6, &mut self.d5, &mut self.d4];
        for i in 0..=3 {
            let state = match (data >> (3 - i)) & 0x01 {
                1 => PinState::High,
                0 => PinState::Low,
                _ => PinState::High,
            };
            pins[i].set_state(state);
        }

        // Toggle EN pin
        self.en.set_high();
        delay.delay_us(20);
        self.en.set_low();
        delay.delay_us(20);
    }

    fn send_data(&mut self, data: u8, delay: &mut Delay) {
        let data_sent = (data >> 4) & 0x0f;
        self.send_to_lcd(data_sent, PinState::High, delay);
        let data_sent = data & 0x0f;
        self.send_to_lcd(data_sent, PinState::High, delay);
    }

    fn send_cmd(&mut self, cmd: u8, delay: &mut Delay) {
        let data = (cmd >> 4) & 0x0f;
        self.send_to_lcd(data, PinState::Low, delay);
        let data = cmd & 0x0f;
        self.send_to_lcd(data, PinState::Low, delay);
    }

    pub fn init(&mut self, delay: &mut Delay) {
        delay.delay_ms(50);
        self.send_cmd(0x03, delay);
        delay.delay_ms(5);
        self.send_cmd(0x03, delay);
        delay.delay_ms(1);
        self.send_cmd(0x03, delay);
        delay.delay_ms(10);
        self.send_cmd(0x02, delay);
        delay.delay_ms(10);

        self.send_cmd(0x28, delay);
        delay.delay_ms(1);
        self.send_cmd(0x08, delay);
        delay.delay_ms(1);
        self.send_cmd(0x01, delay);
        delay.delay_ms(1);
        self.send_cmd(0x06, delay);
        delay.delay_ms(1);
        self.send_cmd(0x0c, delay);
    }

    pub fn clear(&mut self, delay: &mut Delay) {
        self.send_cmd(0x01, delay);
        delay.delay_ms(2);
    }

    pub fn send_string(&mut self, str: &str, delay: &mut Delay) {
        for data in str.as_bytes() {
            self.send_data(*data, delay);
        }
    }

    pub fn put_cur(&mut self, row: u8, mut col: u8, delay: &mut Delay) {
        match row {
            0 => col |= 0x80,
            1 => col |= 0xc0,
            _ => col |= 0xc0,
        }

        self.send_cmd(col, delay);
    }
}

pub fn shield_button_init(adc: ADC2, clock_cfg: &Clocks) {
    let mut _adc_pin = Pin::new(Port::A, 0, PinMode::Analog);
    let adc_config = AdcConfig {
        prescaler: stm32_hal2::adc::Prescaler::D128,
        operation_mode: stm32_hal2::adc::OperationMode::OneShot,
        cal_single_ended: None,
        cal_differential: None,
        ..Default::default()
    };

    let mut button_adc = Adc::new_adc2(adc, AdcDevice::Two, adc_config, &clock_cfg);
    button_adc.set_input_type(5, stm32_hal2::adc::InputType::SingleEnded);
    button_adc.set_align(stm32_hal2::adc::Align::Right);
    button_adc.set_sample_time(5, stm32_hal2::adc::SampleTime::T2);
    button_adc.set_sequence(5, 1);
    button_adc.set_sequence_len(1);
}
