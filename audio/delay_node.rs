use node::{AudioNode, AudioParam, AudioNodeEngine};
use block::{Block, Chunk, Tick};

/* pub struct DelayNode{
   pub c: BaseAudioContext,
   pub option: DelayOptions
} */

pub struct DelayOptions{
   pub maxDelayTime: f32;
   pub delayTime: f32;
}
impl Default for DelayOptions{
   fn default() -> Self{
      DelayOptions{
         maxDelayTime: 1,
         delayTime: 0,
      }
   }
}
//Constructor (line 173 DelayNode.cpp)
//Note: find implementation in servo/servo
pub(crate) struct DelayNode{
   delayTime: Option<Tick>;
   channel_info: ChannelInfo,

}
impl DelayNode{
   pub fn new(options: DelayNodeOptions, channel_info: ChannelInfo) -> Self{
      Self{
         channel_info,
         delay_type: options.delay_type,
         delayTime: None,
      }
   }

}

impl AudioNodeEngine for DelayNode {
   // Check that the incoming node type is valid
   fn node_type(&self) -> AudioNodeType {AudioNodeType::DelayNode}
   // Start processing the audio
   fn process(&mut self, inputs: Chunk, info: &BlockInfo) -> Chunk {
      inputs.blocks.push(Default::default());
      let queue = VecDeque<(Tick, Block)>;
      inputs.blocks[0].explicit_silence();
      let mut iter = input.blocks[0].iter();
      while let Some(mut frame) = iter.next(){
         let tick = frame.tick();
         if queue.is_empty{
            queue.push(tick, frame)
         }
         else{
            let tickTime = tick - delayTime;
            if tickTime == tick{
               break;
            }else{
               
            }
         inputs
      }
      //t may not always start at 0
      //use the tick from the frame in BlockInfo
      // if t= 0, add the block to the vecdequeue
      //t++
      // Output similar block on silence

      //input(t) = t - delaytime
      // While the input(t) != to tick in the vecdequeue, keep iterating through t
      
      //input(t) = tick - delaytime
      //t++
      //  return block of silence
         

      // Otherwise output the block in the vecdeq


      

   }

   
}
