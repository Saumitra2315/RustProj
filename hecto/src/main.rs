use std::io::{self,stdout};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;


fn die(e: std::io::Error) {
    panic!("{}", e);
}
fn main() {

    let _stdout = stdout().into_raw_mode().unwrap();
    for key in io::stdin().keys(){
        match key {
            Ok(Key::Ctrl('q')) => break,
            Ok(key) => match key {
                Key::Ctrl(c) => {
                    if c.is_control() {
                        println!("{:?}\r", c as u8);
                    }
                    else {
                        println!("{:?} ({})\r",c as u8, c );
                    }
                }
                _ => println!("{:?}\r",key),
            },
            Err(err) =>die(err)
            
        }
    }
}
