use rodio::*;
use rodio;

use futures::*;
use std::sync::{Arc,Mutex};

use std::io::{Read,Seek,BufReader,Cursor,SeekFrom};
use std::io::prelude::*;
//use errors::*;


pub struct AudioContext {
    endpoint:Endpoint,
    destination:AudioDestinationNode,
}


impl AudioContext {

    pub fn new() -> AudioContext {
        
        let endpoint = rodio::default_endpoint().unwrap();
        AudioContext { endpoint,destination:AudioDestinationNode }

    }

    pub fn destination(&self) -> &AudioDestinationNode {
        &self.destination
    }

    pub fn create_buffer_source(&self) -> AudioBufferSourceNode {
        let sink = rodio::Sink::new(&self.endpoint);
        AudioBufferSourceNode{sink,buffer:None,endpoint:rodio::default_endpoint().unwrap()}
    }

    pub fn decode_audio_data<T:Into<Vec<u8>>>(&self,data:T) -> AudioBufferFuture
    {
        let buffer = Cursor::new(data.into());
        let (future,inner) = AudioBufferFuture::new();
        *inner.lock().unwrap() = Some(Ok(Async::Ready(AudioBuffer(buffer))));
        future
    }

}



pub enum Error {
    AudioDecodeFailed,
}


pub struct AudioBufferSourceNode {
    endpoint:Endpoint,
    buffer:Option<AudioBuffer>,
    sink:rodio::Sink
}


impl AudioBufferSourceNode {

    pub fn set_buffer(&mut self,buffer:AudioBuffer){
        self.buffer = Some(buffer);
        //let sub = buffer.0.ta
    }

    pub fn connect(&self,dest:&AudioDestinationNode) {
        //js!{ @{&self.0}.connect(@{&dest.0}) };
    }

    pub fn start(&mut self,position:u64){
        //self.buffer.expect("set_buffer first").0
        // .seek(SeekFrom::Start(position));
        //self.sink.sleep_until_end();
        //self.sink.stop();
        if let &Some(ref buffer) = &self.buffer {
            let mut cursor = buffer.0.clone();
            //cursor.seek(SeekFrom::Start(position));
            if let Ok(decoder) = Decoder::new(cursor) {

                self.sink = rodio::Sink::new(&self.endpoint);
                self.sink.append(decoder);
                self.sink.play();
                //    self.sink.sleep_until_end();

            }else{
                println!("Decoding audio failed {}",1 );
            }
            
        }
    }

}

pub struct AudioDestinationNode;

#[derive(Debug,Clone)]
pub struct AudioBuffer(Cursor<Vec<u8>>);



pub struct AudioBufferFuture {
    inner:Arc<Mutex<Option<Poll<AudioBuffer,Error>>>>,
}

impl AudioBufferFuture {
    pub fn new() -> (AudioBufferFuture,Arc<Mutex<Option<Poll<AudioBuffer,Error>>>>) {
        let inner = Arc::new(Mutex::new(None));
        (AudioBufferFuture{inner:inner.clone()},inner)
    }

    pub fn with(buffer:AudioBuffer) -> AudioBufferFuture {
        let inner = Arc::new(Mutex::new(None));
        AudioBufferFuture{inner}
    }

    pub fn take(&mut self) -> Option<AudioBuffer> {
        match self.poll() {
            Ok(Async::Ready(buffer)) => Some(buffer),
            _ => None
        }
    }
}

impl Future for AudioBufferFuture {

    type Item = AudioBuffer;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        self.inner.lock().unwrap().take().unwrap_or(Ok(Async::NotReady))
    }
}


impl AudioBuffer {
/*
    pub fn sample_rate(&self) -> f32 {
        let res = js! { return @{&self.reference}.sampleRate };
    }

    pub fn length(&self) -> u32 {
        (js! { return @{&self.reference}.length }).try_into().unwrap()
    }

    pub fn duration(&self) -> u32 {
        js!{ @{&self.reference}.duration }
    }

    pub fn number_of_channels() -> u32 {
        js!{ @{&self.reference}.numberOfChannels }
    }

    */

}

fn log(msg:&str){
   // js!{ console.log(msg)};
}