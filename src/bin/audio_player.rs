use std::env;
use std::fs::File;
use std::path::Path;
use lofty::{AudioFile};
use rodio::{Decoder, OutputStream, Sink};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: gojo <music.mp3>");
        std::process::exit(0);
    }

    let file_path = &args[1];

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let audio_file = File::open(file_path).unwrap();

    let tagged_file = lofty::read_from_path(file_path).unwrap();

    let file_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();
    let duration = tagged_file.properties().duration();

    println!("Playing - {}", file_name);
    println!("Duration - {}s", duration.as_secs_f64());

    let source = Decoder::new(audio_file).unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);

    sink.sleep_until_end();

}