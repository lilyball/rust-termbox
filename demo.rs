use std;
use termbox;

import tb = termbox;

fn main() {
    tb::init();
    let tc = {fg: tb::WHITE | tb::BOLD, bg: tb::BLACK};
    tb::print(1, 1, tc, "Hello, world!");
    tb::print(1, 3, tc, "Press 'q' to exit.");
    tb::present();
    loop {
        alt tb::poll_event() {
            tb::key_event(ev) {
                if ev.ch as char == 'q' {
                    break;
                }
            }
            _ { }
        }
    }
    tb::shutdown();
}

