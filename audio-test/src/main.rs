extern crate rodio;

use rodio::Sink;
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration; // Import the Duration type from std::time

fn play_mp3_file(file_path: &str) {
    // Open the MP3 file and create a buffer for it.
    let file = File::open(file_path).expect("Failed to open MP3 file");
    let source = rodio::Decoder::new(BufReader::new(file)).expect("Failed to decode MP3 file");

    // Create an audio sink (output) and play the audio.
    let device = rodio::default_output_device().expect("No output device found");
    let sink = Sink::new(&device);

    // Add the decoded audio to the sink.
    sink.append(source);

    // Sleep for a while to allow the audio to play.
    sleep(Duration::from_secs(10)); // Adjust the duration as needed

    // Stop the playback when you're done.
    sink.stop();
}

fn main() {
    let mp3_file_path = "/Users/Aniku/cs181g/walmart-text-based-adventure/audio-test/src/beep.mp3";
    play_mp3_file(mp3_file_path);
    println!("You can't do that!");
}
