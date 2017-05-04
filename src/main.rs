/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern crate termios;

use std::env;
use std::io::{self, Read, Write};
use std::process::exit;

use termios::{Termios, tcsetattr, ECHO, ICANON, TCSANOW};

fn main() {
    let mut args = env::args();
    args.next();
    let current_pos: usize = args.next().expect("initial cursor position is expected")
        .parse().unwrap();
    let command = args.next().expect("command parameter is expected");
    let backward = args.next() == Some("--backward".to_string());
    let target = read_char();
    let mut new_pos = current_pos;
    if backward {
        let data: Vec<_> = command.chars().enumerate().collect();
        let skip_count = command.len().checked_sub(current_pos).unwrap_or(0);
        for &(i, character) in data.iter().rev().skip(skip_count) {
            if character == target {
                new_pos = i;
                break;
            }
        }
    }
    else {
        for (i, character) in command.chars().enumerate().skip(current_pos + 1) {
            if character == target {
                new_pos = i;
                break;
            }
        }
    }
    exit(new_pos as i32);
}

fn read_char() -> char {
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
    buffer[0] as char
}
