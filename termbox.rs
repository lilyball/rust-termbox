#[link(name = "termbox",
       vers = "0.1.0")];
#[crate_type = "lib"];

/*!
 * A wrapper for the termbox library.
 */

use std;

import libc::{c_int,c_uint};
import uint::range;

// Foreign functions from termbox.
#[doc(hidden)]
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


/** 
 * Initialize an environment.
 */
fn init() -> int { 
    ret foreign::tb_init() as int; 
}
/**
 * Shutdown an environment.
 */
fn shutdown() { 
    foreign::tb_shutdown(); 
}

fn width() -> uint { 
    ret foreign::tb_width() as uint; 
}
fn height() -> uint { 
    ret foreign::tb_height() as uint; 
}

/**
 * Clear buffer.
 */
fn clear() { 
    foreign::tb_clear(); 
}

/**
 * Write buffer to terminal.
 */
fn present() { 
    foreign::tb_present(); 
}

fn set_cursor(cx: int, cy: int) { 
    foreign::tb_set_cursor(cx as c_int, cy as c_int); 
}
fn change_cell(x: uint, y: uint, ch: u32, fg: u16, bg: u16) { 
    foreign::tb_change_cell(x as c_uint, y as c_uint, ch, fg, bg); 
}


type coloring = { fg: u16, bg: u16 };

// Constants
const BOLD:      u16 = 0x10;
const UNDERLINE: u16 = 0x20;
const BLACK:     u16 = 0x00;
const RED:       u16 = 0x01;
const GREEN:     u16 = 0x02;
const YELLOW:    u16 = 0x03;
const BLUE:      u16 = 0x04;
const MAGENTA:   u16 = 0x05;
const CYAN:      u16 = 0x06;
const WHITE:     u16 = 0x07;

// Convenience functions

/**
 * Print a string to the buffer.  Leftmost charater is at (x, y).
 */
fn print(x: uint, y: uint, c: coloring, s: str) {
    let {fg: fg, bg: bg} = c;
    for s.each_chari |i, ch| {
        change_cell(x + i, y, ch as u32, fg, bg);
    }
}

/**
 * Print a charater to the buffer.
 */
fn print_ch(x: uint, y: uint, c: coloring, ch: char) {
    let {fg: fg, bg: bg} = c;
    change_cell(x, y, ch as u32, fg, bg);
}

fn with_term(-f: fn~()) {
    init();
    let res = task::try(f);
    shutdown();
    if result::is_err(res) {
        #error("with_term: An error occured.");
    }
}



/*
 * The event type matches struct tb_event from termbox.h
 */
#[doc(hidden)]
type raw_event = {
    mut type: u8,
    mut mod: u8,
    mut key: u16,
    mut ch: u32,
    mut w: i32,
    mut h: i32
};

#[doc(hidden)]
fn nil_raw_event() -> raw_event { 
    {mut type: 0, mut mod: 0, mut key: 0, mut ch: 0, mut w: 0, mut h: 0}
}

enum event {
    key_event({md: u8, key: u16, ch: u32}),
    resize_event({w: i32, h: i32}),
    no_event
}

fn peek_event(timeout: uint) -> event {
    let ev = nil_raw_event();
    let rc = foreign::tb_peek_event(ptr::addr_of(ev), timeout as c_uint);
    ret unpack_event(rc, &ev);
}

fn poll_event() -> event {
    let ev = nil_raw_event();
    let rc = foreign::tb_poll_event(ptr::addr_of(ev));
    ret unpack_event(rc, &ev);
}

/* helper fn
 *
 * ev_type
 *   0 -> no event
 *   1 -> key
 *   2 -> resize
 *   -1 -> error
 */
#[doc(hidden)]
fn unpack_event(ev_type: c_int, ev: &raw_event) -> event {
    alt ev_type {
        0 { no_event }
        1 { key_event({md: ev.mod, key: ev.key, ch: ev.ch}) }
        2 { resize_event({w: ev.w, h: ev.h}) }
        _ { fail; }
    }
}
