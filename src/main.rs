// A completely pointless ascii 'art' cpu percentage usage monitor that runs in the terminal.
// Just exploring the easycurses crate.

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
    let cpu: Vec<&str> = vec![
        "#####  #####  #   #  ##  #     ",
        "#      #   #  #   #  ## #   #  ",
        "#      #####  #   #    #       ",
        "#      #      #   #   # ##  #  ",
        "#####  #      #####  #  ##     ",
    ];

    let nums: Vec<[&str; 5]> = vec![
        ["#####  ", "#   #  ", "#   #  ", "#   #  ", "#####  "],
        ["    #  ", "    #  ", "    #  ", "    #  ", "    #  "],
        ["#####  ", "    #  ", "#####  ", "#      ", "#####  "],
        ["#####  ", "    #  ", "#####  ", "    #  ", "#####  "],
        ["#   #  ", "#   #  ", "#####  ", "    #  ", "    #  "],
        ["#####  ", "#      ", "#####  ", "    #  ", "#####  "],
        ["#      ", "#      ", "#####  ", "#   #  ", "#####  "],
        ["#####  ", "    #  ", "    #  ", "    #  ", "    #  "],
        ["#####  ", "#   #  ", "#####  ", "#   #  ", "#####  "],
        ["#####  ", "#   #  ", "#####  ", "    #  ", "    #  "],
        ["       ", "       ", "       ", "       ", "       "],
    ];

    let mut easy = EasyCurses::initialize_system().unwrap();

    easy.clear();
    easy.set_cursor_visibility(CursorVisibility::Invisible);
    easy.set_echo(false);
    easy.set_color_pair(colorpair!(Blue on Black));

    let mut system = sysinfo::System::new_all();
    let mut num_cores: u8 = 0;
    let mut aves: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0, 0.0];
    let mut i: usize = 0;

    // Count virtual cores.
    for _processor in system.get_processors() {
        num_cores += 1;
    }

    loop {
        easy.move_rc(10, 2);
        easy.refresh();

        system.refresh_all();

        let mut total: f32 = 0.0;

        for processor in system.get_processors() {
            total += processor.get_cpu_usage();
        }

        aves[i] = total / num_cores as f32;

        let ave = mov_ave(aves.clone());

        i += 1;
        if i == 5 {
            i = 0;
        }

        let mut digits: Vec<usize> = vec![10, 10, 10];

        if ave as u8 == 100 {
            digits = vec![1, 0, 0];
        } else {
            if ave >= 10.0 {
                let tens = (ave / 10.0).floor();
                let units = (ave - tens * 10.0).floor();
                digits[0] = tens as usize;
                digits[1] = units as usize;
            } else {
                digits[0] = ave as usize;
            }
        }

        easy.move_rc(1, 2);
        easy.print(format!(
            "{}{}{}{}",
            cpu[0], nums[digits[0]][0], nums[digits[1]][0], nums[digits[2]][0]
        ));
        easy.move_rc(2, 2);
        easy.print(format!(
            "{}{}{}{}",
            cpu[1], nums[digits[0]][1], nums[digits[1]][1], nums[digits[2]][1]
        ));
        easy.move_rc(3, 2);
        easy.print(format!(
            "{}{}{}{}",
            cpu[2], nums[digits[0]][2], nums[digits[1]][2], nums[digits[2]][2]
        ));
        easy.move_rc(4, 2);
        easy.print(format!(
            "{}{}{}{}",
            cpu[3], nums[digits[0]][3], nums[digits[1]][3], nums[digits[2]][3]
        ));
        easy.move_rc(5, 2);
        easy.print(format!(
            "{}{}{}{}",
            cpu[4], nums[digits[0]][4], nums[digits[1]][4], nums[digits[2]][4]
        ));

        thread::sleep(Duration::from_millis(500));
    }
}

fn mov_ave(aves: Vec<f32>) -> f32 {
    let mut total: f32 = 0.0;

    for i in 0..aves.iter().count() {
        total += aves[i];
    }

    total / 5.0
}
