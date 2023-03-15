use stm32_hal2::comp::Comp;

#[allow(unused)]
pub struct Comparator {
    hcomp: Comp,
    // htim: BasicTimer<>,
    timer_frequency: f32,
    current_comp_val: u8,
    ticks: u64,
    clock_ticks: u64,
}

impl Comparator {
    pub fn new(hcomp: Comp, timer_frequency: f32) -> Self {
        Self {
            hcomp,
            timer_frequency,
            current_comp_val: 0,
            ticks: 0,
            clock_ticks: 0,
        }
    }

    pub fn start(&mut self) {
        self.hcomp.start();
        // htim.start();
        self.current_comp_val = self.hcomp.get_output_level() as u8;
    }

    pub fn stop(&mut self) {
        self.hcomp.stop();
        // htim.stop();
    }

    pub fn handle_callback(&mut self) {
        if self.clock_ticks > 65535 {
            self.ticks = 0;
            self.clock_ticks = 0;
        }

        let current_comp_val = self.hcomp.get_output_level() as u8;
        if self.current_comp_val != current_comp_val {
            self.ticks += 1;
            self.current_comp_val = current_comp_val;
        }
        self.clock_ticks += 1;
    }

    pub fn calculate_frequency(&self) -> f32 {
        let freq = self.timer_frequency * self.ticks as f32 / self.clock_ticks as f32 / 2.0;
        freq
    }
}
