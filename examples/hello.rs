extern mod std;
extern mod termbox;

use tb = termbox;

fn main() {
    tb::init();
    tb::print(1, 1, tb::bold, tb::white, tb::black, "Hello, world!");
    tb::present();
    std::timer::sleep(std::uv_global_loop::get(), 1000);
    tb::shutdown();
}
