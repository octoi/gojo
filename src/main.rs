use std::fs::File;
use std::io::BufReader;
use rodio::{OutputStream};
use lofty::{AudioFile};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let audio_file = File::open("audio.mp3").unwrap();
    let file = lofty::Probe::open("audio.mp3").unwrap();

    let tagged_file = file.read().unwrap();
    let duration = tagged_file.properties().duration().as_secs_f64();

    let reader: BufReader<File> = BufReader::new(audio_file);


    let sink = stream_handle.play_once(reader).unwrap();


    println!("Duration - {}", duration);

    std::thread::sleep(std::time::Duration::from_secs(duration as u64 + 1));
}