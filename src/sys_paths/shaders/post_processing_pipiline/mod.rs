// use bevy::image::Image;

pub enum EPostprocessingPipeline {
  CustomWaterExample,
}

// prettier-ignore
impl EPostprocessingPipeline {
  pub fn as_str(&self) -> &'static str {
    match self {
      EPostprocessingPipeline::CustomWaterExample => "shaders/post-processing/custom-water-example.wgsl",
    }
  }
}
