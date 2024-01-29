/* trigger.rs 
 * 
 * defines actions to be triggered; either automatically or through user call.
 * */

use super::system;
use crate::units::{Led, Button, Buzzer, OutBase, InBase}; 

// background processes
// cmds that are triggered every cycle; required by the program
pub fn auto(counter: &u32, leds: &mut Led, _buzzers: &mut Buzzer) {
    if 1 == *counter {
        leds.set("POWER", system::power_stability());
    }
}

// user requested actions
// actions that were triggered by user's explicit action 
pub fn call(shift: &mut bool, cmd: u16) {
    let status: u16 = 10 * (*shift as u16) + cmd;

    match status {
        10 => {
            *shift = false;
            // turn led off
        },

        11 => {
            *shift = true;
            // turn led on
        },

        // ========================= //
        _ => {}
    }
}

// confirm user hasn't requested to quit
pub fn keep_running(buttons: &Button) -> bool {
    unimplemented!();
}

