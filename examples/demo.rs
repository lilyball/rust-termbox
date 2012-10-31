extern mod termbox;

use tb = termbox;

fn print(x: uint, y: uint, s: &str) {
    tb::print(x, y, tb::bold, tb::white, tb::black, s);
}

fn main() {
    tb::init();
    print(1, 1, "Hello, world!");
    print(1, 3, "Press 'q' to quit.");
    tb::present();
    loop {
        match tb::poll_event() {
            tb::key_event(ev) => {
                if ev.ch as char == 'q' {
                    break;
                }
            },
            _ => { }
        }
    }
    tb::shutdown();
}

