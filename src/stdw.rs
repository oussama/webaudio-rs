use stdweb::*;
use stdweb::unstable::TryInto;

use futures::*;
use std::rc::Rc;
use std::cell::RefCell;

use stdweb::web::*;

pub struct AudioContext {
    reference:Reference,
    destination:AudioDestinationNode,
}


impl AudioContext {

    pub fn new() -> AudioContext {
        let res = js!{
            window.AudioContext = window.AudioContext || window.webkitAudioContext;
            return new AudioContext();
        };
        let reference = res.into_reference().unwrap();
        let destination_res = js!{ return @{&reference}.destination };
        AudioContext {
            reference,
            destination:AudioDestinationNode(destination_res.into_reference().unwrap())
        }
    }

    pub fn destination(&self) -> &AudioDestinationNode {
        &self.destination
    }

    pub fn create_buffer_source(&self) -> AudioBufferSourceNode {
        let res = js!{ return @{&self.reference}.createBufferSource(); };
        AudioBufferSourceNode(res.into_reference().unwrap())
    }

    pub fn decode_audio_data(&self,data:&[u8]) -> AudioBufferFuture {
        let (future,inner) = AudioBufferFuture::new();
        js!{
            @{&self.reference}.decodeAudioData(@{ unsafe { TypedArray::from(data) } }.buffer,@{move|buffer:Value|{
                js!{ console.log("callback",@{&buffer})};
                *inner.borrow_mut() = Some(buffer.into_reference()
                .map(|reference|Async::Ready(AudioBuffer(reference)))
                .ok_or(Error::AudioDecodeFailed));
                js!{ console.log("loaded")};
            }})
        }
        future
    }

}



pub enum Error {
    AudioDecodeFailed,
}


pub struct AudioBufferSourceNode (Reference);


impl AudioBufferSourceNode {

    pub fn set_buffer(&self,buffer:&AudioBuffer){
        js!{ @{&self.0}.buffer = @{&buffer.0} };
    }

    pub fn connect(&self,dest:&AudioDestinationNode) {
        js!{ @{&self.0}.connect(@{&dest.0}) };
    }

    pub fn start(&self,position:u32){
        js!{ @{&self.0}.start(@{position}); };
    }

}

pub struct AudioDestinationNode(Reference);


pub struct AudioBuffer(Reference);



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
    js!{ console.log(msg)};
}