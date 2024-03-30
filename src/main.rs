use std::fs::File;
use std::sync::{Arc, Mutex};
use rodio::{Decoder, OutputStream, Sink};
use lofty::{AudioFile};


fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // open file
    let mut audio_file = File::open("audio.mp3").unwrap();

    // get duration
    let tagged_file = lofty::read_from(&mut audio_file).unwrap();
    let duration = tagged_file.properties().duration().as_secs_f64();



    // let source_original = Decoder::new(audio_file).unwrap();
    // let source = Arc::new(source_original);

    let source = Decoder::new(audio_file).unwrap();

    // let file = lofty::Probe::open("audio.mp3").unwrap();
    // let source = Decoder::new(audio_file).unwrap();

    // let tagged_file = file.read().unwrap();
    // let duration = tagged_file.properties().duration().as_secs_f64();

    // let reader: BufReader<File> = BufReader::new(audio_file);


    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    source.skip_duration(std::time::Duration::from_secs(5));


    sink.sleep_until_end();

    // std::thread::sleep(std::time::Duration::from_secs(duration as u64 + 1 - 7));
}