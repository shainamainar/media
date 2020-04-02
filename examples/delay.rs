//make sure that the directory is set up for cargo nightly by running
//$ rustup override set nightly
//run with command
//$cargo ex delay


//currently just plays a sound using oscillator node, need to add using the 
//delay node
extern crate servo_media;
extern crate servo_media_auto;

use servo_media::audio::node::{AudioNodeInit, AudioNodeMessage, AudioScheduledSourceNodeMessage};
use servo_media::{ClientContextId, ServoMedia};
use std::sync::Arc;
use std::{thread, time};

fn run_example(servo_media: Arc<ServoMedia>) {
    let context =
        servo_media.create_audio_context(&ClientContextId::build(1, 1), Default::default());
    let context = context.lock().unwrap();
    let dest = context.dest_node();
    let listener = context.listener();
    let osc = context.create_node(
        AudioNodeInit::OscillatorNode(Default::default()),
        Default::default(),
    );
    
    // in panner they take the output from oscillator put it through panner
    //then take the output from panner and put it through dest
    //context.connect_ports(osc.output(0), panner.input(0));
    //context.connect_ports(panner.output(0), dest.input(0));
    context.connect_ports(osc.output(0), dest.input(0));
    let _ = context.resume();

    context.message_node(
        osc,
        AudioNodeMessage::AudioScheduledSourceNode(AudioScheduledSourceNodeMessage::Start(0.)),
    );

    thread::sleep(time::Duration::from_millis(3000));
    let _ = context.close();
}

fn main() {
    ServoMedia::init::<servo_media_auto::Backend>();
    if let Ok(servo_media) = ServoMedia::get() {
        run_example(servo_media);
    } else {
        unreachable!();
    }
}
