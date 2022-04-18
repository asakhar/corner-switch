use anyhow::Result;
use gdk::Screen;
use xdo::Xdo;

fn main() -> Result<()> {
  let mut xdo = Xdo::new(&std::env::var("DISPLAY").expect("Display not set"))
    .expect("Failed to initialize xdo");

  let mut prev = (0, 0, 0);

  gdk::init();
  let width = Screen::screen_width() - 1;
  #[allow(unused_variables)]
  let height = Screen::screen_height() - 1;
  loop {
    let mouse = xdo
      .get_mouse_location()
      .expect("Cannot retrieve mouse location");
    if mouse.0 == prev.0 {
      continue;
    }

    if mouse.0 == 0 {
      let cur_desk = xdo
        .get_current_desktop()
        .expect("Cannot retrieve current desktop");
      xdo
        .set_current_desktop(cur_desk - 1)
        .expect("Cannot set current desktop");
    }
    if mouse.0 == width {
      let cur_desk = xdo
        .get_current_desktop()
        .expect("Cannot retrieve current desktop");
      let num_desks = xdo
        .get_number_of_desktops()
        .expect("Cannot retrieve number of desktops");
      if cur_desk + 2 == num_desks {
        continue;
      }
      xdo
        .set_current_desktop(cur_desk + 1)
        .expect("Cannot set current desktop");
    }
    prev = mouse;
  }
  // Ok(())
}
