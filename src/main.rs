use rodio::{source::Source, Decoder, OutputStream};
use std::fs::{self, File};
use std::i32;
use std::io::{BufReader, Write};
use std::thread::sleep;
use std::time::Duration;

// read the cached file to see what session we're on
// load the session into memory
// loop through the session
// - play sound corresponding to walk or run
// - for each loop, loop through the time and display timer
// final message

mod running_program;

fn main() {
    let home_dir = dirs::home_dir().expect("No $HOME var");
    let path = home_dir.join(".cache/runaway-session.txt");
    let session_number = match fs::read_to_string(path.clone()) {
        // new line value will cause the parse to fail
        Ok(val) => val.parse::<i32>().unwrap_or(1),
        Err(e) => {
            println!("Error reading file: {}", e);
            println!("Creating file: /home/cc/.cache/runaway-session.txt");
            // fs::write will create file if it doesn't exist
            // FIX: change these 0s to 1s for production
            fs::write(path.clone(), "0").expect("Unable to write to file");
            0
        }
    };

    // running_program::get_session(x)[y].z where
    // x = session
    // y = the part of the section you're on
    // z = 0 or 1 with 0 being "walk" or "run" and 1 being the duration
    // println!("{}", running_program::get_session(1)[1].1);
    let session = running_program::get_session(session_number);

    for i in 0..session.len() {
        let mut timer = session[i].1;

        let state = match session[i].0 {
            "w" => "walk",
            "r" => "run ",
            _ => "none",
        };

        let color_escape_code = match session[i].0 {
            "w" => "\x1b[0;36m",
            "r" => "\x1b[0;35m",
            _ => "none",
        };

        // get output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // load a sound from a file, path relative to Cargo.toml
        let file = match session[i].0 {
            "w" => BufReader::new(File::open("resources/audio/clave-walk.wav").unwrap()),
            "r" => BufReader::new(File::open("resources/audio/clave-run.wav").unwrap()),
            _ => return,
        };
        // decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        // play the sound directly on the device
        let _ = stream_handle.play_raw(source.convert_samples());

        while timer > 0 {
            // {:0>2} zero pads the numbers
            let time = format!("{:0>2}:{:0>2}", timer / 60, timer % 60);
            // carriage return magic to print over previous text
            print!("\r{color_escape_code}{state} {time}\x1b[0m");
            // if you don't put a newline at the end, it won't print from the buffer, hence flush
            let _ = std::io::stdout().flush();
            // sleep for 1 second
            sleep(Duration::new(1, 0));
            timer -= 1;
        }
    }

    // can't end without a newline character or zsh prints a highlighted percent symbol
    print!("\rcongrats on finishing the run!\n");

    let next_session_number = session_number + 1;
    fs::write(path, next_session_number.to_string()).expect("Unable to write to file");
}
