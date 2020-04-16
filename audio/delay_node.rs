//use node::{AudioNode, AudioParam, AudioNodeEngine};
use node::{AudioNodeEngine, AudioNodeMessage, BlockInfo};
use block::{Block, Chunk, Tick};
use param::{Param, ParamType};
use node::{AudioNodeType, ChannelInfo};

/* pub struct DelayNode{
   pub c: BaseAudioContext,
   pub option: DelayOptions
} */

pub struct DelayOptions{
   pub maxDelayTime: f32,
   pub delayTime: f32,
}
impl Default for DelayOptions{
   fn default() -> Self{
      DelayOptions{
         maxDelayTime: 1.0,
         delayTime: 0.0,
      }
   }
}
//Constructor (line 173 DelayNode.cpp)
//Note: find implementation in servo/servo
#[derive(AudioNodeCommon)]
pub(crate) struct DelayNode{
   delay_type: DelayType,
   delayTime: Option<Tick>,
   channel_info: ChannelInfo,

}
impl DelayNode{
   pub fn new(options: DelayOptions, channel_info: ChannelInfo) -> Self{
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
      let queue = VecDeque::<(Tick, Block)>;
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
	  }
	}
}

      impl AudioNodeEngineP2 for DelayNode {
            // Check that the incoming node type is valid
         fn node_type(&self) -> AudioNodeType {AudioNodeType::DelayNode}
            // Start processing the audio
         fn process(&mut self, inputs: Chunk, info: &BlockInfo) -> Chunk {
            let queue = VecDeque::<(Tick, Block)>;

            // Obtain the start and stop times of audio
            if BlockInfo.Tick.tickTime != 0 {
                while let Some(mut frame) = iter.next() {
                  let tick = frame.tick();
					if tick < start_at {
						continue;
					} else if tick > stop_at {
						break;
					} else {
						// Obtain location of delay
						inputs.blocks.push(Default::default());
						let (start_at, stop_at) = match self(info.frame) {
						DelayOptions::Check => {
                        return Block;
						}
						// Insert delay and adjust
						DelayNodeOptions::Between(start, end) => (start_at, stop_at) {
							while let Some(mut frame) = iter.next(){
								let tick = frame.tick();
								if queue.is_empty{
									queue.push(tick, frame)
								}
								else{
									let tickTime = tick - delayTime;
									if tickTime == tick{
										break;
									}
								};
							inputs 
							}
						}
						}
            
					}
				}					
			else {
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

				  }
   
			 }
			}
		}
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

/*

    fn process(&mut self, inputs: Chunk, _: &BlockInfo) -> Chunk {
        self.chunk = Some(inputs);
        Chunk::default()
    }

impl AudioNodeEngineP3 for DelayNode {
   // Check that the incoming node type is valid
   fn node_type(&self) -> AudioNodeType {AudioNodeType::DelayNode}
   // Start processing the audi
         if queue.is_empty{
            queue.push(tick, frame)
         }
         else{
            let tickTime = tick - delayTime;
            if tickTime == tick{
               break;
            }else{
               let buf: VecDeque<i32> = VecDeque::with_capacity(10);
               assert!(buf.capacity() >= 10);
            }
         inputs
      }

*/
