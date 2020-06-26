use anyhow::{anyhow, Context, Result};
use std::rc::Rc;
use xcb;
use xcb::xproto::{PropertyNotifyEvent, PROPERTY_NOTIFY};
use xcb_util::ewmh;

fn get_root_visual_type(conn: &xcb::Connection, screen: &xcb::Screen<'_>) -> xcb::Visualtype {
    for root in conn.get_setup().roots() {
        for allowed_depth in root.allowed_depths() {
            for visual in allowed_depth.visuals() {
                if visual.visual_id() == screen.root_visual() {
                    return visual;
                }
            }
        }
    }
    panic!("No visual type found");
}


//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

//fn xcb_connect() -> xcb::Connection {
//    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
//
//    let screen = conn
//        .get_setup()
//        .roots()
//        .nth(screen_num as usize)
//        .unwrap();
//
//    let wid = conn.generate_id();
//
//    // https://rtbo.github.io/rust-xcb/xcb/xproto/fn.create_window.html
//    xcb::create_window(
//        &conn,
//        xcb::COPY_FROM_PARENT as u8, // depth
//        wid, // window_id
//        screen.root(), // parent
//        0, // x
//        0, // y
//        150, // width
//        150, // height
//        10, // border_width
//        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16, // class
//        screen.root_visual(), // visual_id (from parent)
//        &[
//            (xcb::CW_BACK_PIXEL, screen.white_pixel()),
//            (xcb::CW_EVENT_MASK,
//             xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
//        ]
//    );
//}

#[derive(Clone, Debug)]
pub enum Position {
    Top,
    Bottom
}

pub struct Bar {
    position: Position,

    conn: Rc<ewmh::Connection>,
    screen_idx: usize,
    window_id: u32,

    //surface: cairo::XCBSurface,
    width: u16,
    height: u16,
}

impl Bar {
    pub fn new(position: Position) -> Result<Bar> {

        let (conn, screen_idx) =
            xcb::Connection::connect(None).context("Failed to connect to X server")?;

        let screen_idx = screen_idx as usize;
        let window_id = conn.generate_id();

        let height = 1;

        let screen = conn
            .get_setup()
            .roots()
            .nth(screen_idx)
            .ok_or_else(|| anyhow!("Invalid screen"))?;

        let values = [
            (xcb::CW_BACK_PIXEL, screen.black_pixel()),
            (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE),
        ];

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
            &values,
        );

        let cairo_conn = unsafe {
            cairo::XCBConnection::from_raw_none(conn.get_raw_conn() as *mut cairo_sys::xcb_connection_t)
        };

        let visual = unsafe {
            cairo::XCBVisualType::from_raw_none(
                &mut get_root_visual_type(&conn, &screen).base as *mut xcb::ffi::xcb_visualtype_t
                    as *mut cairo_sys::xcb_visualtype_t,
            )
        };

        let drawable = cairo::XCBDrawable(window_id);

        let surface = cairo::XCBSurface::create(&cairo_conn, &drawable, &visual, i32::from(width), i32::from(height))
            .map_err(|status| anyhow!("XCBSurface::create: {}", status))?;

        let ewmh_conn = ewmh::Connection::connect(conn)
            .map_err(|(e, _)| e)
            .context("Failed to wrap xcb::Connection in ewmh::Connection")?;

        let bar = Bar {
            conn: Rc::new(ewmh_conn),
            screen_idx,
            window_id,
            position,
            height,
            width
        };

        Ok(bar)
    }
}

fn main() {

    // https://rtbo.github.io/rust-xcb/xcb/base/struct.Connection.html

    //let conn = xcb_connect();

    //let y = screen.height_in_pixels() - 35;

    //let values = [
    //    (xcb::CONFIG_WINDOW_Y as u16, u32::from(y)),
    //    (xcb::CONFIG_WINDOW_HEIGHT as u16, 35),
    //    (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE),
    //];

    //xcb::configure_window(&conn, wid, &values);

    //xcb::map_window(&conn, wid);

    //conn.flush();

    //loop {
    //    let event = conn.wait_for_event();
    //    match event {
    //        None => { break; }
    //        Some(event) => {
    //            let r = event.response_type() & !0x80;
    //            dbg!(r);
    //            match r {
    //                xcb::EXPOSE => {

    //                    /* We flush the request */
    //                    conn.flush();

    //                },
    //                xcb::KEY_PRESS => {
    //                    let key_press : &xcb::KeyPressEvent = unsafe {
    //                        xcb::cast_event(&event)
    //                    };
    //                    println!("Key '{}' pressed", key_press.detail());
    //                    break;
    //                },
    //                _ => {}
    //            }
    //        }
    //    }
    //}
}
