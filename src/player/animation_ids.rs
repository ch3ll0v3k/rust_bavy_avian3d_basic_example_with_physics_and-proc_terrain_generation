// use bevy::image::Image;

pub enum EAnimationIds {
  None, //  = 'None',
  Idle, //  = 'Standing-idle-01',
  WalkForward, //  = 'Standing-walk-forward',
  WalkLeft, //  = 'Standing-walk-left',
  WalkRight, //  = 'Standing-walk-right',
  WalkBack, //  = 'Standing-walk-back',
  RunForward, //  = 'Standing-run-forward',
  RunBack, //  = 'Standing-run-back',
  RunLeft, //  = 'Standing-run-left',
  RunRight, //  = 'Standing-run-right',
  RunForwardStop, //  = 'Standing-run-forward-stop',
  RunForwardJump, //  = 'Standing-jump-running-to-run-forward',
  StandingJump, //  = 'Standing-jump',
}

impl EAnimationIds {
  pub fn as_str(&self) -> &'static str {
    match self {
      EAnimationIds::None => "None",
      EAnimationIds::Idle => "Standing-idle-01",
      EAnimationIds::WalkForward => "Standing-walk-forward",
      EAnimationIds::WalkLeft => "Standing-walk-left",
      EAnimationIds::WalkRight => "Standing-walk-right",
      EAnimationIds::WalkBack => "Standing-walk-back",
      EAnimationIds::RunForward => "Standing-run-forward",
      EAnimationIds::RunBack => "Standing-run-back",
      EAnimationIds::RunLeft => "Standing-run-left",
      EAnimationIds::RunRight => "Standing-run-right",
      EAnimationIds::RunForwardStop => "Standing-run-forward-stop",
      EAnimationIds::RunForwardJump => "Standing-jump-running-to-run-forward",
      EAnimationIds::StandingJump => "Standing-jump",
    }
  }
}

// #animation: Crouch-to-standing
// #animation: Standing-aim-recoil
// #animation: Crouch-walk-forward
// #animation: Crouch-walk-left
// #animation: Crouch-walk-right
// #animation: Fall-a-land-to-run-forward
// #animation: Fall-a-loop
// #animation: Standing-run-forward-stop
// #animation: Fall-b-loop
// #animation: Standing-aim-walk-back
// #animation: Standing-aim-walk-forward
// #animation: Standing-aim-walk-left
// #animation: Standing-aim-walk-right
// #animation: Crouch-walk-back
// #animation: Standing-disarm-bow
// #animation: Standing-draw-arrow
// #animation: Standing-equip-bow
// #animation: Standing-to-crouch
// #animation: Fall-a-land-to-standing-idle-02
// #animation:
// #animation: Standing-melee-kick
// #animation: Standing-dive-forward
// #animation: Standing-aim-idle
// #animation: Standing-aim-overdraw
// #animation: Crouch-idle-01
// #animation: Crouch-idle-03-looking-over
// #animation: Crouch-idle-02-looking-around
