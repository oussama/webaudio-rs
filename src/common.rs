use stdweb::*;
use stdweb::unstable::TryInto;

use futures::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct AudioContext {
    reference:Reference,
    destination:AudioDestinationNode,
}


impl AudioContext {

    fn new() {
        
    }

    pub fn create_buffer_source(&self) -> AudioBufferSourceNode {
        let res = js!{ @{&self.reference}.createBufferSource(); };
        AudioBufferSourceNode(res.into_reference().unwrap())
    }

    pub fn decode_audio_data(&self,data:&[u8]) -> AudioBufferFuture {
        let (future,inner) = AudioBufferFuture::new();
        js!{
            @{&self.reference}.decodeAudioData(@{data},@{move|buffer:Value|{
                *inner.borrow_mut() = Some(buffer.into_reference()
                .map(|reference|Async::Ready(AudioBuffer{reference}))
                .ok_or(Error::AudioDecodeFailed));
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
    fn new() {
        
    }

    pub fn connect(dest:&AudioDestinationNode) {

    }
}

pub struct AudioDestinationNode {

}


pub struct AudioBuffer {
    reference:Reference,
}



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

