// use bevy::image::Image;

pub enum EFont {
  QuartzoMain,
  Bigpartyo2GreenMain,
  LoveYouBlackSeeTrough,
  LoveYouBlackSolid,
}

impl EFont {
  pub fn as_str(&self) -> &'static str {
    match self {
      EFont::QuartzoMain => "fonts/ui/quartzo/main.otf",
      EFont::Bigpartyo2GreenMain => "fonts/ui/bigpartyo2-green/main.ttf",
      EFont::LoveYouBlackSeeTrough => "fonts/ui/kg-someone-you-loved/black.see-trough.otf",
      EFont::LoveYouBlackSolid => "fonts/ui/kg-someone-you-loved/black.solid.otf",
    }
  }
}
