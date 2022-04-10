use anyhow::Result;
use xdo::Xdo;

fn main() -> Result<()> {
    let mut xdo = Xdo::new(&std::env::var("DISPLAY").expect("Display not set"))
        .expect("Failed to initialize xdo");

    let mut mouse;

    let mut prev = (0, 0, 0);
    loop {
        mouse = xdo
            .get_mouse_location()
            .expect("Cannot retrieve mouse location");
        if mouse.0 == prev.0 {
            continue;
        }
        if mouse.0 == 0 {
            let cur_desk = xdo
                .get_current_desktop()
                .expect("Cannot retrieve current desktop");
            xdo.set_current_desktop(cur_desk - 1)
                .expect("Cannot set current desktop");
        }
        if mouse.0 == 3199 {
            let cur_desk = xdo
                .get_current_desktop()
                .expect("Cannot retrieve current desktop");
            let num_desks = xdo
                .get_number_of_desktops()
                .expect("Cannot retrieve number of desktops");
            println!("Num desks: {}, cur: {}", num_desks, cur_desk);
            if cur_desk + 1 == num_desks {
                xdo.set_number_of_desktops(num_desks + 1)
                    .expect("Cannot set number of desktops");
            }
            xdo.set_current_desktop(cur_desk + 1)
                .expect("Cannot set current desktop");
        }
        prev = mouse;
    }
    // Ok(())
}
