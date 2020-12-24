// Prevents a spare console from being created attached to our program on
// windows, but only if we're running in release mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Enabling macro_use allows the `colorpair!` macro, though you can also use
// `ColorPair::new(Color,Color)` if you don't want the macro.


use easycurses::Color::*;
use easycurses::*;
use std::{thread, time::Duration};
use sysinfo::{ProcessorExt, SystemExt};

fn main() {
  // Initialize the system
  let mut easy = EasyCurses::initialize_system().unwrap();

  //easy.set_scrolling(false);

  easy.clear();

  // don't show the cursor
  easy.set_cursor_visibility(CursorVisibility::Invisible);

  // don't echo the user's input
  easy.set_echo(false);

  // we'll print this in green text.
  easy.set_color_pair(colorpair!(Blue on Black));

  let mut system = sysinfo::System::new_all();

  let mut num_cores: u8 = 0;

  // Count virtual cores.
  for _processor in system.get_processors() {
    num_cores += 1;
  }



  loop {
    easy.move_rc(1, 2);

    easy.refresh();

    system.refresh_all();

    let mut total: f32 = 0.0;

    for processor in system.get_processors() {
        total += processor.get_cpu_usage();
    }

    let ave = total / num_cores as f32;

    easy.print(format!("{}", ave as u8));
    easy.print_char('%');
    easy.print("  ");

    thread::sleep(Duration::from_millis(1000));
  }
}
