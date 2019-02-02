//! Output device component interfaces for devices such as `LED`, `PWMLED`, etc
use rppal::gpio::{Gpio, IoPin, Level, Mode};
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

/// Represents a generic GPIO output device.
#[derive(Debug)]
pub struct OutputDeviceR {
    pin: IoPin,
    active_state: bool,
    inactive_state: bool,
}

macro_rules! impl_io_device {
    () => {
        fn value_to_state(&self, value: bool) -> bool {
            if value {
                self.active_state
            } else {
                self.inactive_state
            }
        }

        fn state_to_value(&self, state: bool) -> bool {
            state == self.active_state
        }

        /// Returns ``True`` if the device is currently active and ``False`` otherwise.
        pub fn value(&self) -> bool {
            match self.pin.read() {
            Level::Low => self.state_to_value(false),
            Level::High => self.state_to_value(true),
            }
        }




    }
}

macro_rules! impl_output_device {
    () => {
    /// Set the state for active_high
    pub fn set_active_high(&mut self, value: bool) {
        if value {
            self.active_state=true;
            self.inactive_state=false;
        } else {
            self.active_state=false;
            self.inactive_state=true;
        }
    }
    /// When ``True``, the `value` property is ``True`` when the device's
    /// `pin` is high. When ``False`` the `value` property is
    /// ``True`` when the device's pin is low (i.e. the value is inverted).
    /// Be warned that changing it will invert `value` (i.e. changing this property doesn't change
    /// the device's pin state - it just changes how that state is interpreted).
    pub fn active_high(&self) -> bool {
        self.active_state
    }

    /// Turns the device on.
    pub fn on(&mut self) {
        self.write_state(true)
    }

    /// Turns the device off.
    pub fn off(&mut self) {
        self.write_state(false)
    }
    /// Reverse the state of the device. If it's on, turn it off; if it's off, turn it on.
    pub fn toggle(&mut self) {
        if self.is_active(){
            self.off()
        }else{
            self.on()
        }
    }



    fn write_state(&mut self, value: bool) {
        if self.value_to_state(value) {
            self.pin.set_high()
        } else {
            self.pin.set_low()
        }
    }




    }
}

impl OutputDeviceR {
    /// Returns an OutputDevice with the pin number given
    /// # Arguments
    ///
    /// * `pin` - The GPIO pin which the device is attached to
    ///  
    pub fn new(pin: u8) -> OutputDeviceR {
        match Gpio::new() {
            Err(e) => panic!("{:?}", e),
            Ok(gpio) => match gpio.get(pin) {
                Err(e) => panic!("{:?}", e),
                Ok(pin) => OutputDeviceR {
                    pin: pin.into_io(Mode::Output),
                    active_state: true,
                    inactive_state: false,
                },
            },
        }
    }

    impl_device!();
    impl_gpio_device!();
    impl_io_device!();
    impl_output_device!();
}

/// Represents a generic output device with typical on/off behaviour.
/// Extends behaviour with a blink() method which uses a background
/// thread to handle toggling the device state without further interaction.
#[derive(Debug)]
pub struct DigitalOutputDeviceR {
    device: Arc<Mutex<OutputDeviceR>>,
    blinking: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    blink_count: Option<i32>
}

macro_rules! impl_digital_output_device {
    () => {

        fn blinker(&mut self,
                on_time: f32,
                off_time: f32,
                n: Option<i32>){
            self.stop();

            let device = Arc::clone(&self.device);
            let blinking = Arc::clone(&self.blinking);


            self.handle = Some(thread::spawn(move || {
                blinking.store(true, Ordering::SeqCst);
                match n {
                Some(end) => {
                    for _ in 0..end {
                            if !blinking.load(Ordering::SeqCst) {
                                device.lock().unwrap().off();
                                break;
                            }
                            device.lock().unwrap().on();
                            thread::sleep(Duration::from_millis((on_time * 1000.0) as u64));
                            device.lock().unwrap().off();
                            thread::sleep(Duration::from_millis((off_time * 1000.0) as u64));
                    }
                }
                None => loop {
                    if !blinking.load(Ordering::SeqCst) {
                        device.lock().unwrap().off();
                        break;
                    }
                    device.lock().unwrap().on();
                    thread::sleep(Duration::from_millis((on_time * 1000.0) as u64));
                    device.lock().unwrap().off();
                    thread::sleep(Duration::from_millis((off_time * 1000.0) as u64));
                },
            }
            }));

        }
        /// Returns ``True`` if the device is currently active and ``False`` otherwise.
        pub fn is_active(&self) -> bool{
            Arc::clone(&self.device).lock().unwrap().is_active()
        }
        /// Turns the device on.
        pub fn on(&self){
            self.stop();
            self.device.lock().unwrap().on()
        }
        /// Turns the device off.
        pub fn off(&self){
            self.stop();
            self.device.lock().unwrap().off()
        }

        /// Reverse the state of the device. If it's on, turn it off; if it's off, turn it on.
        pub fn toggle(&mut self) {
            self.device.lock().unwrap().toggle()
        }

        /// Returns ``True`` if the device is currently active and ``False`` otherwise.
        pub fn value(&self) -> bool {
            self.device.lock().unwrap().value()
        }

        fn stop(&self) {
        self.blinking.clone().store(false, Ordering::SeqCst);
        self.device.lock().unwrap().pin.set_low();
        }

        /// When ``True``, the `value` property is ``True`` when the device's
        /// `pin` is high. When ``False`` the `value` property is
        /// ``True`` when the device's pin is low (i.e. the value is inverted).
        /// Be warned that changing it will invert `value` (i.e. changing this property doesn't change
        /// the device's pin state - it just changes how that state is interpreted).
        pub fn active_high(&self) -> bool {
            self.device.lock().unwrap().active_high()
        }

        /// Set the state for active_high
        pub fn set_active_high(&mut self, value: bool) {
            self.device.lock().unwrap().set_active_high(value)
        }

        /// The `Pin` that the device is connected to.
        pub fn pin(&self) -> u8 {
           self.device.lock().unwrap().pin.pin()
        }

        /// Shut down the device and release all associated resources.
        pub fn close(self) {
            drop(self)
        }

        /// Block until background process is done
        pub fn wait(&mut self){
            self.handle
                .take().expect("Called stop on non-running thread")
                .join().expect("Could not join spawned thread");
        }

        
    }
}

impl DigitalOutputDeviceR {
    pub fn new(pin: u8) -> DigitalOutputDeviceR {
        DigitalOutputDeviceR {
            device: Arc::new(Mutex::new(OutputDeviceR::new(pin))),
            blinking: Arc::new(AtomicBool::new(false)),
            handle: None,
            blink_count: None
        }
    }

    impl_digital_output_device!();

    /// Make the device turn on and off repeatedly in the background. 
    /// Use `set_blink_count` to set the number of times to blink the device
    /// # Arguments
    /// * `on_time` - Number of seconds on
    /// * `off_time` - Number of seconds off
    ///
    pub fn blink(&mut self, on_time: f32, off_time: f32) {
        match self.blink_count{
            None => self.blinker(on_time, off_time, None),
            Some(n) => self.blinker(on_time, off_time, Some(n))
        }
        
    }
    /// Set the number of times to blink the device
    /// # Arguments
    /// * `n` - Number of times to blink
    pub fn set_blink_count(&mut self, n:i32){
        self.blink_count = Some(n)
    }
}

///  Represents a light emitting diode (LED)
///
/// # Example
///  Connect LED as shown below, with cathode(short leg) connected to GND
///
/// ```shell
///           Resistor     LED
///  Pin 14 o--/\/\/---->|------o GND
///  ```
///

#[derive(Debug)]
pub struct LEDR {
    device: Arc<Mutex<OutputDeviceR>>,
    blinking: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    blink_count: Option<i32>
}

impl LEDR {
    pub fn new(pin: u8) -> LEDR {
        LEDR {
            device: Arc::new(Mutex::new(OutputDeviceR::new(pin))),
            blinking: Arc::new(AtomicBool::new(false)),
            handle: None,
            blink_count: None
        }
    }

    impl_digital_output_device!();

    /// Returns True if the device is currently active and False otherwise.
    pub fn is_lit(&self) -> bool {
        self.is_active()
    }

    /// Make the device turn on and off repeatedly in the background. 
    /// Use `set_blink_count` to set the number of times to blink the device
    /// # Arguments
    /// * `on_time` - Number of seconds on
    /// * `off_time` - Number of seconds off
    ///
    pub fn blink(&mut self, on_time: f32, off_time: f32) {
        match self.blink_count{
            None => self.blinker(on_time, off_time, None),
            Some(n) => self.blinker(on_time, off_time, Some(n))
        }
        
    }
    /// Set the number of times to blink the device
    /// # Arguments
    /// * `n` - Number of times to blink
    pub fn set_blink_count(&mut self, n:i32){
        self.blink_count = Some(n)
    }
}

/// Represents a digital buzzer component.
///
/// Connect the cathode (negative pin) of the buzzer to a ground pin;
/// connect the other side to any GPIO pin.

#[derive(Debug)]
pub struct BuzzerR {
    device: Arc<Mutex<OutputDeviceR>>,
    blinking: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    blink_count: Option<i32>
}

impl BuzzerR {
    pub fn new(pin: u8) -> BuzzerR {
        BuzzerR {
            device: Arc::new(Mutex::new(OutputDeviceR::new(pin))),
            blinking: Arc::new(AtomicBool::new(false)),
            handle: None,
            blink_count: None
        }
    }

    impl_digital_output_device!();

    /// Make the device turn on and off repeatedly in the background. 
    /// Use `set_beep_count` to set the number of times to beep the device
    /// # Arguments
    /// * `on_time` - Number of seconds on
    /// * `off_time` - Number of seconds off
    ///
    pub fn beep(&mut self, on_time: f32, off_time: f32) {
        match self.blink_count{
            None => self.blinker(on_time, off_time, None),
            Some(n) => self.blinker(on_time, off_time, Some(n))
        }
        
    }
    /// Set the number of times to beep the device
    /// # Arguments
    /// * `n` - Number of times to beep
    pub fn set_beep_count(&mut self, n:i32){
        self.blink_count = Some(n)
    }
}
