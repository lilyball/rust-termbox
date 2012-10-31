#[link(name = "termbox", vers = "0.1.0")];
#[crate_type = "lib"];

/*!
 *
 * A lightweight curses alternative wrapping the termbox library.
 *
 * # SYNOPSIS
 *
 * A hello world for the terminal:
 *
 *     extern mod std;
 *     extern mod termbox;
 *
 *     use tb = termbox;
 *
 *     fn main() {
 *         tb::init();
 *         tb::print(1, 1, tb::bold, tb::white, tb::black, "Hello, world!");
 *         tb::present();
 *         std::timer::sleep(std::uv_global_loop::get(), 1000);
 *         tb::shutdown();
 *     }
 *
 * # DESCRIPTION
 *
 * Output is double-buffered.
 *
 * TODO
 *
 * # EXAMPLES
 *
 * TODO
 *
 */

use libc::{c_int,c_uint};
use ff = foreign;

/*
 * Foreign functions from termbox.
 */
#[link_name="termbox"]
extern mod foreign {
    fn tb_init() -> c_int;
    fn tb_shutdown();

    fn tb_width() -> c_uint;
    fn tb_height() -> c_uint;

    fn tb_clear();
    fn tb_present();

    fn tb_set_cursor(cx: c_int, cy: c_int);

    fn tb_change_cell(x: c_uint, y: c_uint, ch: u32, fg: u16, bg: u16);

    fn tb_select_input_mode(mode: c_int) -> c_int;
    fn tb_set_clear_attributes(fg: u16, bg: u16);

    fn tb_peek_event(ev: *raw_event, timeout: c_uint) -> c_int;
    fn tb_poll_event(ev: *raw_event) -> c_int;
}


pub fn init() -> int { 
    ff::tb_init() as int
}

pub fn shutdown() { 
    ff::tb_shutdown(); 
}

pub fn width() -> uint {
    ff::tb_width() as uint
}

pub fn height() -> uint {
    ff::tb_height() as uint
}

/**
 * Clear buffer.
 */
pub fn clear() { 
    ff::tb_clear(); 
}

/**
 * Write buffer to terminal.
 */
pub fn present() { 
    ff::tb_present(); 
}

pub fn set_cursor(cx: int, cy: int) { 
    ff::tb_set_cursor(cx as c_int, cy as c_int); 
}

// low-level wrapper
fn change_cell(x: uint, y: uint, ch: u32, fg: u16, bg: u16) { 
    ff::tb_change_cell(x as c_uint, y as c_uint, ch, fg, bg); 
}

// Convert from enums to u16
fn convert_color(c: color) -> u16 {
    match c {
        black   => 0x00,
        red     => 0x01,
        green   => 0x02,
        yellow  => 0x03,
        blue    => 0x04,
        magenta => 0x05,
        cyan    => 0x06,
        white   => 0x07
    }
}

fn convert_style(sty: style) -> u16 {
    match sty {
        normal         => 0x00,
        bold           => 0x10,
        underline      => 0x20,
        bold_underline => 0x30
    }
}

/**
 * Print a string to the buffer.  Leftmost charater is at (x, y).
 */
pub fn print(x: uint, y: uint, sty: style, fg: color, bg: color, s: &str) {
    let fg: u16 = convert_color(fg) | convert_style(sty);
    let bg: u16 = convert_color(bg);
    for s.each_chari |i, ch| {
        ff::tb_change_cell((x + i) as c_uint, y as c_uint, ch as u32, fg, bg);
    }
}

/**
 * Print a charater to the buffer.
 */
pub fn print_ch(x: uint, y: uint, sty: style, fg: color, bg: color, ch: char) {
    let fg: u16 = convert_color(fg) | convert_style(sty);
    let bg: u16 = convert_color(bg);
    ff::tb_change_cell(x as c_uint, y as c_uint, ch as u32, fg, bg);
}

enum color {
    black,
    red,
    green,
    yellow,
    blue,
    magenta,
    cyan,
    white
}

enum style {
    normal,
    bold,
    underline,
    bold_underline
}

// Convenience functions
fn with_term(f: fn~()) {
    init();
    let res = task::try(f);
    shutdown();
    if result::is_err(&res) {
        error!("with_term: An error occured.");
    }
}



/*
 * The event type matches struct tb_event from termbox.h
 */
struct raw_event {
    mut e_type: u8,
    mut e_mod: u8,
    mut key: u16,
    mut ch: u32,
    mut w: i32,
    mut h: i32
}

fn nil_raw_event() -> raw_event { 
    raw_event{mut e_type: 0, mut e_mod: 0, mut key: 0, mut ch: 0, mut w: 0, mut h: 0}
}

enum event {
    key_event({md: u8, key: u16, ch: u32}),
    resize_event({w: i32, h: i32}),
    no_event
}

/**
 * Get an event if within timeout milliseconds, otherwise return no_event.
 */
pub fn peek_event(timeout: uint) -> event {
    let ev = nil_raw_event();
    let rc = ff::tb_peek_event(ptr::addr_of(&ev), timeout as c_uint);
    unpack_event(rc, &ev)
}

/**
 * Blocking function to return next event.
 */
pub fn poll_event() -> event {
    let ev = nil_raw_event();
    let rc = ff::tb_poll_event(ptr::addr_of(&ev));
    unpack_event(rc, &ev)
}

/* helper fn
 *
 * ev_type
 *   0 -> no event
 *   1 -> key
 *   2 -> resize
 *   -1 -> error
 */
fn unpack_event(ev_type: c_int, ev: &raw_event) -> event {
    match ev_type {
        0 => no_event,
        1 => key_event({md: ev.e_mod, key: ev.key, ch: ev.ch}),
        2 => resize_event({w: ev.w, h: ev.h}),
        _ => fail
    }
}

