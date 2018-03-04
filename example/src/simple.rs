use utils::*;
use webaudio::*;
use application::*;
use futures::*;

pub fn main() {

    let size = (800, 600);
    let config = AppConfig::new("Test", size);
    let mut app = App::new(config);

    let file_data = include_bytes!("../assets/nocturne.ogx");

    let ctx = AudioContext::new();
    let mut future = ctx.decode_audio_data(&file_data[..]);

    let mut sources = Vec::new();

    app.run(move |_t:&mut App| {
        if let Some(buffer) = future.take() {
                    let mut source = ctx.create_buffer_source(); // creates a sound source
                    source.set_buffer(buffer);                    // tell the source which sound to play
                    source.connect(ctx.destination());       // connect the source to the context's destination (the speakers)
                    source.start(0);
                    sources.push(source);
        }
        //println!("{}",sources.len());
    });

}