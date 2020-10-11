// use std::io::BufReader;
use std::fs;

fn main() {
    let paths = fs::read_dir("./audio_files").unwrap();

    for entry in paths {
        println!("Name: {}", entry.unwrap().path().display());
    }
       
    let paths = fs::read_dir("./audio_files").unwrap();
    
    for entry in paths {
        let entry2 = entry.unwrap();

        let entry_path = entry2.path();

        let file_name = entry_path.file_name().unwrap();

        let file_name_as_str = file_name.to_str().unwrap();

        let file_name_as_string = String::from(file_name_as_str);
        println!("Name: {}",file_name_as_string);
    }

    // let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    // let sink = rodio::Sink::try_new(&handle).unwrap();

    // let file = std::fs::File::open("examples/music.flac").unwrap();
    // sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    // sink.sleep_until_end();
}
