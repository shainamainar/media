use node::{AudioNode, AudioParam};

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