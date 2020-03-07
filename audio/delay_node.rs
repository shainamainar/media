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
      // At tick 0, add terh block to the vecdequeue

      // At tick 1, 

      let mut iter = inputs.blocks[0].iter();
      // While the tick != to tick in the vecdequeue
      

      

   }

   
}
