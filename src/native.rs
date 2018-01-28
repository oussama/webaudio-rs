use rodio::*;
use rodio;

use futures::*;
use std::rc::Rc;
use std::cell::RefCell;

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
        AudioBufferSourceNode{sink,buffer:None}
    }

    pub fn decode_audio_data<T:Into<Vec<u8>>>(&self,data:T) -> AudioBufferFuture
    {
        let buffer = BufReader::new(Cursor::new(data.into()));
        let (future,inner) = AudioBufferFuture::new();
        *inner.borrow_mut() = if let Ok(decoder) = Decoder::new(buffer) {
            Some(Ok(Async::Ready(AudioBuffer(decoder))))
        }else{
            Some(Err(Error::AudioDecodeFailed))
        };
        future
    }

}



pub enum Error {
    AudioDecodeFailed,
}


pub struct AudioBufferSourceNode {
    buffer:Option<AudioBuffer>,
    sink:rodio::Sink
}


impl AudioBufferSourceNode {

    pub fn set_buffer(&mut self,buffer:AudioBuffer){
        //self.buffer = Some(buffer);
        self.sink.append(buffer.0);
        self.sink.pause();
    }

    pub fn connect(&self,dest:&AudioDestinationNode) {
        //js!{ @{&self.0}.connect(@{&dest.0}) };
    }

    pub fn start(&self,position:u64){
        //self.buffer.expect("set_buffer first").0
        // .seek(SeekFrom::Start(position));
        //self.sink.sleep_until_end();
        self.sink.play();
    }

}

pub struct AudioDestinationNode;


pub struct AudioBuffer(Decoder<BufReader<Cursor<Vec<u8>>>>);



pub struct AudioBufferFuture {
    inner:Rc<RefCell<Option<Poll<AudioBuffer,Error>>>>,
}

impl AudioBufferFuture {
    pub fn new() -> (AudioBufferFuture,Rc<RefCell<Option<Poll<AudioBuffer,Error>>>>) {
        let inner = Rc::new(RefCell::new(None));
        (AudioBufferFuture{inner:inner.clone()},inner)
    }
}

impl Future for AudioBufferFuture {

    type Item = AudioBuffer;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item,Self::Error> {
        self.inner.borrow_mut().take().unwrap_or(Ok(Async::NotReady))
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