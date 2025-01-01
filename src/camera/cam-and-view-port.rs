/// This system will then change the title during execution
fn change_title(mut window: Single<&mut Window>, time: Res<Time>) {
  window.title = format!(
    "Seconds since startup: {}",
    time.elapsed().as_secs_f32() // .round()
  );
}
fn switch_level(input: Res<ButtonInput<KeyCode>>, mut window: Single<&mut Window>) {
  // window.resolution = WindowResolution::new(1280.0 / 2.0, 720.0 / 2.0);
  // window.resolution.set_scale_factor_override(Some(2.0));

  if input.just_pressed(KeyCode::KeyT) {
    let w = window.resolution.width();
    let h = window.resolution.height();
    dbg!("w: {}, h: {}", w, h);

    window.window_level = match window.window_level {
      WindowLevel::AlwaysOnBottom => WindowLevel::Normal,
      WindowLevel::Normal => WindowLevel::AlwaysOnTop,
      WindowLevel::AlwaysOnTop => WindowLevel::AlwaysOnBottom,
    };
    info!("WINDOW_LEVEL: {:?}", window.window_level);
  }
}

fn toggle_window_controls(input: Res<ButtonInput<KeyCode>>, mut window: Single<&mut Window>) {
  let toggle_minimize = input.just_pressed(KeyCode::Digit1);
  let toggle_maximize = input.just_pressed(KeyCode::Digit2);
  let toggle_close = input.just_pressed(KeyCode::Digit3);

  if toggle_minimize || toggle_maximize || toggle_close {
    if toggle_minimize {
      window.enabled_buttons.minimize = !window.enabled_buttons.minimize;
      dbg!("1: {:?}", window.enabled_buttons.minimize);
    }
    if toggle_maximize {
      window.enabled_buttons.maximize = !window.enabled_buttons.maximize;
      dbg!("2: {:?}", window.enabled_buttons.minimize);
    }
    if toggle_close {
      window.enabled_buttons.close = !window.enabled_buttons.close;
      dbg!("3: {:?}", window.enabled_buttons.minimize);
    }
  }
}

fn handle_right_click(mut evr: EventReader<MouseButtonInput>) {
  for ev in evr.read() {
    if ev.button == MouseButton::Right {
      dbg!("Right mouse button pressed");
    }
  }
}

fn handle_left_click(mut evr: EventReader<MouseButtonInput>) {
  for ev in evr.read() {
    if ev.button == MouseButton::Right {
      dbg!("Right mouse button pressed");
    }
  }
}

fn handle_drag(
  mut evr: EventReader<MouseMotion>,
  mut query_camera: Query<&mut Transform, With<CameraMarker>>
) {
  let mut transform = query_camera.single_mut();
  for ev in evr.read() {
    dbg!("Mouse drag: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    transform.rotate_local_y(ev.delta.x / 100.0);
    transform.rotate_local_x(ev.delta.y / 100.0);
  }
}

fn update_scroll_position(
  mut mw_evt: EventReader<MouseWheel>,
  mut query_camera: Query<&mut Transform, With<CameraMarker>>
) {
  let mut transform = query_camera.single_mut();

  for mouse_wheel_event in mw_evt.read() {
    let (dx, dy) = match mouse_wheel_event.unit {
      // MouseScrollUnit::Line => (mouse_wheel_event.x, mouse_wheel_event.y),
      // MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
      MouseScrollUnit::Line => (mouse_wheel_event.x, mouse_wheel_event.y),
      MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
    };

    dbg!("Mouse wheel: X: {}, Y: {}", dx, dy);

    transform.translation -= Vec3::new(0.0, dy / 1.0, 0.0);

    // if kb_evt.pressed(KeyCode::ControlLeft) || kb_evt.pressed(KeyCode::ControlRight) {
    //     std::mem::swap(&mut dx, &mut dy);
    // }
  }
}

// const LINE_HEIGHT: f32 = 1.0;
// fn update_scroll_position(
//     mut mouse_wheel_events: EventReader<MouseWheel>,
//     hover_map: Res<HoverMap>,
//     mut scrolled_node_query: Query<&mut ScrollPosition>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
// ) {
//     for mouse_wheel_event in mouse_wheel_events.read() {
//         let (mut dx, mut dy) = match mouse_wheel_event.unit {
//             MouseScrollUnit::Line => (
//                 mouse_wheel_event.x * LINE_HEIGHT,
//                 mouse_wheel_event.y * LINE_HEIGHT,
//             ),
//             MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
//         };

//         if keyboard_input.pressed(KeyCode::ControlLeft)
//             || keyboard_input.pressed(KeyCode::ControlRight)
//         {
//             std::mem::swap(&mut dx, &mut dy);
//         }

//         for (_pointer, pointer_map) in hover_map.iter() {
//             for (entity, _hit) in pointer_map.iter() {
//                 if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
//                     scroll_position.offset_x -= dx;
//                     scroll_position.offset_y -= dy;
//                 }
//             }
//         }
//     }
// }

// https://bevy-cheatbook.github.io/input/mouse.html
fn scroll_events(mut evr_scroll: EventReader<MouseWheel>) {
  for ev in evr_scroll.read() {
    match ev.unit {
      MouseScrollUnit::Line => {
        dbg!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
      }
      MouseScrollUnit::Pixel => {
        dbg!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
      }
    }
  }
}

fn mouse_button_events(mut mousebtn_evr: EventReader<MouseButtonInput>) {
  use bevy::input::ButtonState;

  for ev in mousebtn_evr.read() {
    match ev.state {
      ButtonState::Pressed => {
        dbg!("Mouse button press: {:?}", ev.button);
      }
      ButtonState::Released => {
        dbg!("Mouse button release: {:?}", ev.button);
      }
    }
  }
}
