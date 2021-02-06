mod power_down;
use power_down::PowerDown;

mod line_in;
use line_in::LineIn;

mod headphone_out;
use headphone_out::HeadphoneOut;

mod analog_audio_path;
use analog_audio_path::AnalogAudioPath;

mod digital_audio_path;
use digital_audio_path::DigitalAudioPath;

pub struct WM8731<F: FnMut(u8, u16)> {
    set_register: F,
}

impl<F: FnMut(u8, u16)> WM8731<F> {
    pub fn with_setter(setter: F) -> Self {
        Self {
            set_register: setter,
        }
    }

    pub fn left_line_in(&mut self, c: fn(&mut LineIn)) {
        let mut li = LineIn::new();
        c(&mut li);

        (self.set_register)(0, li.data)
    }

    pub fn right_line_in(&mut self, c: fn(&mut LineIn)) {
        let mut li = LineIn::new();
        c(&mut li);

        (self.set_register)(1, li.data)
    }

    pub fn left_headphone_out(&mut self, c: fn(&mut HeadphoneOut)) {
        let mut lho = HeadphoneOut::new();
        c(&mut lho);

        (self.set_register)(2, lho.data)
    }

    pub fn right_headphone_out(&mut self, c: fn(&mut HeadphoneOut)) {
        let mut rho = HeadphoneOut::new();
        c(&mut rho);

        (self.set_register)(3, rho.data)
    }

    pub fn analog_audio_path(&mut self, c: fn(&mut AnalogAudioPath)) {
        let mut aap = AnalogAudioPath::new();
        c(&mut aap);

        (self.set_register)(4, aap.data)
    }

    pub fn digital_audio_path(&mut self, c: fn(&mut DigitalAudioPath)) {
        let mut dap = DigitalAudioPath::new();
        c(&mut dap);

        (self.set_register)(5, dap.data)
    }

    pub fn power_down(&mut self, c: fn(&mut PowerDown)) {
        let mut pd = PowerDown::new();
        c(&mut pd);

        (self.set_register)(6, pd.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_down() {
        let mut data = None;
        let mut codec = WM8731::with_setter(|address, value| data = Some((address, value)));

        codec.power_down(|c| {
            c.line_input();
            c.adc();
            c.dac();
        });

        assert_eq!(data, Some((6, 0b0_0000_1101)));
    }
}

pub fn main() {}
