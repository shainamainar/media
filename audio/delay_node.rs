use node::{AudioNode, AudioParam, AudioNodeEngine};
use block::{Block, Chunk, Tick};

//akldsjf
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

pub(crate) struct DelayNode{

}
impl DelayNode{

}

impl AudioNodeEngine for DelayNode {

   // Check that the incoming node type is valid
   fn node_type(&self) -> AudioNodeType {AudioNodeType::DelayNode}
   // Start processing the audio
   fn process(&mut self, inputs: Chunk, info: &BlockInfo) -> Chunk {
      
      //t=0;
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
