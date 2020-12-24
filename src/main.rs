// Prevents a spare console from being created attached to our program on
// windows, but only if we're running in release mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Enabling macro_use allows the `colorpair!` macro, though you can also use
// `ColorPair::new(Color,Color)` if you don't want the macro.
extern crate easycurses;

use easycurses::Color::*;
use easycurses::*;
use std::{thread, time::Duration};

fn main() {
  // Initialize the system
  let mut easy = EasyCurses::initialize_system().unwrap();

  // don't show the cursor
  easy.set_cursor_visibility(CursorVisibility::Invisible);

  // don't echo the user's input
  easy.set_echo(false);

  // we'll print this in green text.
  easy.set_color_pair(colorpair!(Green on Black));

  let mut n: u32 = 0;

  loop {
    easy.move_rc(1, 2);
    easy.print(format!("{}   ", n));

    n += 1;

    easy.refresh();

    thread::sleep(Duration::from_millis(1));
  }

  // Ensure that the user has the latest view of things.

  // Get one input from the user. This is just so that they have a chance to
  // see the message and press a key, otherwise the program would end faster
  // than they could read it.
  //easy.get_input();
}
