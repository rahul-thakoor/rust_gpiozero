use std::fmt;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::Button;
use rppal::gpio::{self, Level, Trigger};

/// Adds `.debounce()` method to [`Button`] for converting to a [`Debounced`] button
pub trait Debounce {
    fn debounce(self, duration: Duration) -> Debounced;
}

impl Debounce for Button {
    fn debounce(self, duration: Duration) -> Debounced {
        Debounced {
            inner: self,
            period: duration,
            last_trigger: Arc::new(Mutex::new(None)),
        }
    }
}

/// Wrapper type for [`Button`] to allow for software [debouncing](https://en.wikipedia.org/wiki/Switch#Contact%20Bounce). Will prevent
/// Subsequent triggers with the given debounce period (E.g. 50-100 milliseconds)
///
/// Can be used with blocking functions (E.g [`Button::wait_for_press`]):
/// ```
/// use rust_gpiozero::{Button, Debounce};
/// use std::time::Duration;
///
/// // Create a button which is attached to Pin 17
/// let mut button = Button::new(17)
///     // Add debouncing so that subsequent presses within 100ms don't trigger a press
///     .debounce(Duration::from_millis(100));
///
/// button.wait_for_press(None);
/// println!("button pressed");
/// ```
///
/// Or async interrupt functions (E.g. [`Button::when_pressed`]):
/// ```
/// use rust_gpiozero::{Button, Debounce};
/// use std::time::Duration;
///
/// // Create a button which is attached to Pin 17
/// let mut button = Button::new(17)
///     // Add debouncing so that subsequent presses within 100ms don't trigger a press
///     .debounce(Duration::from_millis(100));
///
/// // Add an async interrupt to trigger whenever the button is pressed
/// button.when_pressed(|_| {
///     println!("button pressed");
/// }).unwrap();
/// ```

pub struct Debounced {
    inner: Button,
    period: Duration,
    last_trigger: Arc<Mutex<Option<Instant>>>,
}

impl fmt::Debug for Debounced {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Debounced")
            .field("pin", &self.inner.pin())
            .field("period", &self.period)
            .finish()
    }
}

impl Debounced {
    /// Pause the program until the device is deactivated, or the timeout is reached.
    /// * `timeout` - Number of seconds to wait before proceeding. If this is None, then wait indefinitely until the device is inactive.
    pub fn wait_for_release(&mut self, timeout: Option<f32>) {
        Debounced::wait_for(self, timeout, false)
    }

    /// Pause the program until the device is activated, or the timeout is reached.
    /// * `timeout` - Number of seconds to wait before proceeding. If this is None, then wait indefinitely until the device is active.
    pub fn wait_for_press(&mut self, timeout: Option<f32>) {
        Debounced::wait_for(self, timeout, true)
    }

    fn wait_for(&mut self, timeout: Option<f32>, active: bool) {
        let trigger = match active {
            true => Trigger::RisingEdge,
            false => Trigger::FallingEdge,
        };
        let timeout = timeout.map(|seconds| Duration::from_millis((seconds * 1000.0) as u64));
        self.inner.pin.set_interrupt(trigger).unwrap();
        loop {
            self.inner.pin.poll_interrupt(true, timeout).unwrap();
            // Check that enough time has passed since the last press
            if let Some(last_trigger) = self.last_trigger.lock().unwrap().as_ref() {
                // If this press is within the debounce time, continue blocking until the next press
                if last_trigger.elapsed() < self.period {
                    continue;
                }
            }
            // if self.last_trigger is not set, there have been no previous presses so debounce time doesn't matter
            break;
        }
        self.last_trigger.lock().unwrap().replace(Instant::now());
    }

    /// Asynchronously invokes the passed closure everytime the button is pressed, if the debounce period has passed
    pub fn when_pressed<C>(&mut self, action: C) -> Result<(), gpio::Error>
    where
        C: FnMut(Level) + Send + 'static,
    {
        self.action_on(true, action)
    }

    /// Asynchronously invokes the passed closure everytime the button is released, if the debounce period has passed
    pub fn when_released<C>(&mut self, action: C) -> Result<(), gpio::Error>
    where
        C: FnMut(Level) + Send + 'static,
    {
        self.action_on(false, action)
    }

    pub(crate) fn action_on<C>(&mut self, active: bool, mut action: C) -> Result<(), gpio::Error>
    where
        C: FnMut(Level) + Send + 'static,
    {
        let period = self.period;
        let last_trigger = self.last_trigger.clone();
        self.inner.action_on(active, move |level| {
            let mut lt = last_trigger.lock().unwrap();
            if let Some(last) = lt.as_ref() {
                if last.elapsed() < period {
                    // Within debounce period, don't execute action
                    return;
                }
            }
            lt.replace(Instant::now());
            action(level);
        })
    }
}

impl Deref for Debounced {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Debounced {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
