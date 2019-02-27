# Changelog

## 0.2.0
`rust_gpiozero` now uses [rppal](https://github.com/golemparts/rppal/) for gpio access

* **input_devices**
  - **InputDevice** changes:
    + `pin` is now `u8`
    + `new` configures device with the pin pulled to low by default 
    + Added `new_with_pullup`: create InputDevice with the pin number given with the pin pulled high with an internal resistor by default
    + `value` returns a `bool`

  - **DigitalInputDevice** changes:
    + `pin` is now `u8`
    + `new` configures device with the pin pulled to low by default 
    + Added `new_with_pullup`: create InputDevice with the pin number given with the pin pulled high with an internal resistor by default
    + `value` returns a `bool`
    + `wait_for_inactive` takes an argument `timeout` (Option<f32>)
    + `wait_for_active` takes an argument `timeout` (Option<f32>)
    
  - **Button** changes:
    + `pin` is now `u8`
    + `new` configures device with the pin pulled to `high` by default 
    + Added `new_with_pulldown`: create InputDevice with the pin number given with the pin pulled low with an internal resistor by default
    + `value` returns a `bool`
    + `wait_for_inactive` takes an argument `timeout` (Option<f32>)
    + `wait_for_active` takes an argument `timeout` (Option<f32>)
    
* **output_devices**
  - **PWMOutputDevice** (New)
    + A generic output device configured for software pulse-width modulation (PWM)
    + Values can be specified between 0.0 and 1.0 for varying levels of power in the device.
    
  - **PWMLED** (New)
    + Represents a light emitting diode (LED) with variable brightness.
    + Values can be specified between 0.0 and 1.0 for varying levels of brightness.
 
  - **Servo** (New)
    + Represents a PWM-controlled servo motor connected to a GPIO pin
    
  - **OutputDevice** changes:
    + `pin` is now `u8`
    + Added `active_high`: When True, the value property is True when the device's pin is high. When False the value property is True when the device's pin is low (i.e. the value is inverted).
    + Added `set_active_high` to set the state for `active_high`
    
  - **DigitalOutputDevice** changes:
    + `pin` is now `u8`
    + Added `active_high`: When True, the value property is True when the device's pin is high. When False the value property is True when the device's pin is low (i.e. the value is inverted).
    + Added `set_active_high` to set the state for `active_high`
    + Added `blink` to make the device turn on and off repeatedly in the background. 
    + Added `set_blink_count`to set the number of times to blink the device
    + Added `wait` which blocks until background blinking process is done
    
  - **LED** changes:
    + `pin` is now `u8`
    + Added `active_high`: When True, the value property is True when the device's pin is high. When False the value property is True when the device's pin is low (i.e. the value is inverted).
    + Added `set_active_high` to set the state for `active_high`
    + `blink` now takes `f32` for `on_time` and `off_time`. 
    + Added `set_blink_count`to set the number of times to blink the device
    + Added `wait` which blocks until background blinking process is done
    + Added `is_lit` which returns True if the device is currently active and False otherwise.
    
  - **Buzzer** changes:
    + `pin` is now `u8`
    + Added `active_high`: When True, the value property is True when the device's pin is high. When False the value property is True when the device's pin is low (i.e. the value is inverted).
    + Added `set_active_high` to set the state for `active_high`
    + Added `beep` to make the device turn on and off repeatedly in the background. 
    + Added `set_beep_count`to set the number of times to beep the device
    + Added `wait` which blocks until background beeping process is done
    + Removed `blink` method
    
  - **Motor** changes:
    + `formward_pin` and `backward_pin`are now `u8`
    + Added `set_speed` method:  Use `set_speed` to change the speed at which motors should turn. Can be any value between 0.0 and the default 1.0 (maximum speed)

* **device**
  - Renamed `GPIODevice` to `GpioDevice`
