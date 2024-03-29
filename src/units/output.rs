/* units/output.rs 
 *
 * defines common structure and functions for all output components, with
 * standard implementation for all functions.
 */

use gpiod;
use crate::counter;
use super::pulse::*;

// common structure with the most essential fields
pub struct Output<const COUNT: usize> {
    pub line:  gpiod::Lines<gpiod::Output>,
    pub sleep: [u32; COUNT],
    pub pulse: [Pulse; COUNT]
}

// trait defining functions essential for all output components.
// got implementation for default init()
pub trait OutBase<const COUNT: usize> {
    const PINS: [u32; COUNT];
    const ID: &'static str;

    fn init(chip: &gpiod::Chip) -> Output<COUNT> {
        let sleep_status: [u32; COUNT] = [0; COUNT]; 

        let blink_status: [Pulse; COUNT] = [Pulse::default(); COUNT];

        let options = gpiod::Options::output(Self::PINS)
            .consumer(Self::ID);

        let connection_line = chip
            .request_lines(options)
            .expect("Failed to initialize {id}");

        return Output {
            line:  connection_line,
            sleep: sleep_status,
            pulse: blink_status
        };
    }

    fn update(&mut self, counter: &u32);
    fn blink(&mut self, id: &str, duration: u8, pace: Pace);
    fn set(&mut self, id: &str, status: bool);
}

/* std_functions()
 * 
 * given `self` keyword in a trait redirects to the trait itself, default 
 * implementation could not be defined within the trait for functions that
 * relied on `self`.
 *
 * with this, option1 was to have functions that require itself as parameter,
 * like: `led.update(&mut led);` which is... just not very ideal,instead, with 
 * defining these functions outside the trait, `led.update()` is possible.
 *
 * furthermore, with this approach, custom, i.e. unit specific, implementations
 * could be built on top of the standard ones. they could be called with custom 
 * parameters specific to the need, instead of essentially rewriting the whole
 * code again with slight changes.
 */

// standard update: should be sufficient for most needs
pub fn std_update<const COUNT: usize>(
    unit: &mut Output<COUNT>, 
    counter: &u32
) {
    let mut values = [None; COUNT];

    for i in 0..COUNT {
        if *counter ==  unit.sleep[i] {

            // reset the sleep counter 
            unit.sleep[i] = {
                let next_val = counter::next(counter, &unit.pulse[i].pace);
                let to_reset = (unit.pulse[i].count != 0) as u32;
                next_val * to_reset 
            };

            // value = False when 'even numbered' blink, else True
            values[i] = Some(unit.pulse[i].count % 2 == 1);

            // update blink counter
            unit.pulse[i].count -= (unit.pulse[i].count != 0) as u8;
        }
    }
    unit.line.set_values(values)
        .expect("Failed to set values");
}

// standard blink: performs basic blink for given pace 
pub fn std_blink<const COUNT: usize>(
    unit: &mut Output<COUNT>, 
    id: usize, 
    duration: u8, 
    pace: Pace
) {
    unit.pulse[id].count = duration * 2;
    unit.pulse[id].pace  = pace as u32;
}

// stanard set: simplest way to switch a component's state (on/off)
pub fn std_set<const COUNT: usize>(
    unit: &mut Output<COUNT>, 
    id: usize, 
    value: bool
) {
    let mut selection = [None; COUNT];
    selection[id] = Some(value);
    unit.line.set_values(selection)
        .expect("Failed to set values");
}

