// #[macro_export]
// macro_rules! dbgln {
//   ($($arg:tt)*) => {
//     #[cfg(debug_assertions)]
//     println!($($arg)*);
//   };
// }

#[macro_export]
macro_rules! dbgln {
  ($($arg:tt)*) => {
    #[cfg(debug_assertions)]
    {
      use chrono::Local;
      let now = Local::now();
      // let timestamp = now.format("[%Y-%m-%d %H:%M:%S%.3f]").to_string();
      let timestamp = now.format("[%H:%M:%S%.3f]").to_string();
      println!("{} {}", timestamp, format!($($arg)*));
    }
  };
}
