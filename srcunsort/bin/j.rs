//extern crate xcb;
//#![allow(dead_code)]
#![allow(unused_imports)]
//#![allow(unused_variables)]


mod rw {

    use anyhow::{anyhow, Context, Result};
    use std::rc::Rc;
    use xcb;
    use xcb_util::ewmh;
    use xcb::xproto::{PropertyNotifyEvent, PROPERTY_NOTIFY};

    #[derive(Debug)]
    pub enum BarError {
        
    }

    #[derive(Clone, Debug)]
    pub enum Position {
        Top,
        Bottom
    }

    pub struct Bar {
        position: Position,
        pub conn: Rc<ewmh::Connection>,
        screen_idx: usize,
        window_id: u32,
    }

    impl Bar {
        pub fn new(position: Position) -> Result<Bar> {

            /* XCB */

            let (xcb_conn, screen_idx) =
                xcb::Connection::connect(None).context("Failed to connect to X server")?;

            println!("xcb connected");

            let screen_idx = screen_idx as usize;

            let conn = ewmh::Connection::connect(xcb_conn)
                .map_err(|(e, _)| e)
                .context("Failed to wrap xcb::Connection in ewmh::Connection")?;
            
            println!("xcb wrapped in ewmh");

            let window_id = conn.generate_id();

            let height = 1;

            let screen = conn
                .get_setup()
                .roots()
                .nth(screen_idx)
                .ok_or_else(|| anyhow!("Invalid screen"))?;

            let width = screen.width_in_pixels();

            xcb::create_window(
                &conn,
                xcb::COPY_FROM_PARENT as u8,
                window_id,
                screen.root(),
                0,
                0,
                width,
                height,
                0,
                xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                screen.root_visual(),
                &[
                    (xcb::CW_BACK_PIXEL, screen.white_pixel()),
                    (xcb::CW_EVENT_MASK,
                     xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
                ]
            );

            xcb::map_window(&conn, window_id);
            println!("xcb window mapped");

            conn.flush(); // draws the window
            println!("xcb connection flushed");

            let y = screen.height_in_pixels() - 35;

            xcb::configure_window(&conn, window_id, &[
                (xcb::CONFIG_WINDOW_Y as u16, u32::from(y)),
                (xcb::CONFIG_WINDOW_HEIGHT as u16, 35),
                (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE),
            ]);

            xcb::map_window(&conn, window_id);
            conn.flush();


            let bar = Bar {
                /* XCB */
                conn: Rc::new(conn),
                screen_idx,
                window_id,

                /* Bar */
                position,
            };

            Ok(bar)
        }

        pub fn poll_for_event(&self) {
            println!("polling for events");
            self.conn.poll_for_event();
        }

        pub fn wait_for_event(&self) {
            println!("waiting for events");
            self.wait_for_event();
        }

        pub fn flush(&self) {
            self.conn.flush();
            println!("xcb connection flushed");
        }

        pub fn map_window(&self) {
            xcb::map_window(&self.conn, self.window_id);
            println!("xcb window mapped");
        }

    }
}


fn main() {
    let bar = match rw::Bar::new(rw::Position::Bottom) {
        Ok(bar) => {
            println!("bar created");
            loop {
                let event = match bar.conn.wait_for_event() {
                    Some(event) => {
                        let r = event.response_type() & !0x80;
                        match r {
                            xcb::EXPOSE => {
                                println!("xcb:EXPOSE {}", r);
                            },
                            xcb::KEY_PRESS => {
                                let key_press : &xcb::KeyPressEvent = unsafe {
                                    xcb::cast_event(&event)
                                };
                                match key_press.detail() {
                                    24 => {
                                        println!("event: quit");
                                        break;
                                    }
                                    _ => {
                                        println!("event: xcb::KEY_PRESS {}", key_press.detail());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    None => {
                        break;
                    }
                };
            }
        }
        Err(e) => {
            println!("bar error: {}", e);
        }
    };
}
