// use bevy::image::Image;

pub enum EFontPaths {
  QuartzoMain,
  Bigpartyo2GreenMain,
  LoveYouBlackSeeTrough,
  LoveYouBlackSolid,
}

impl EFontPaths {
  pub fn as_str(&self) -> &'static str {
    match self {
      EFontPaths::QuartzoMain => "fonts/ui/quartzo/main.otf",
      EFontPaths::Bigpartyo2GreenMain => "fonts/ui/bigpartyo2-green/main.ttf",
      EFontPaths::LoveYouBlackSeeTrough => "fonts/ui/kg-someone-you-loved/black.see-trough.otf",
      EFontPaths::LoveYouBlackSolid => "fonts/ui/kg-someone-you-loved/black.solid.otf",
    }
  }
}
