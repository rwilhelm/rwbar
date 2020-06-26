extern crate xcb;

use std::rc::Rc;
use xcb_util::ewmh;

use std::collections::HashMap;

//fn screen(&conn) -> Result(xcb::Screen<'_>> {
//    let screen = conn.get_setup().roots().nth(
//}

fn main() {

    /* https://rtbo.github.io/rust-xcb/xcb/base/struct.Connection.html
     *
     * xcb::Connection handles communication with the X server. It wraps an xcb_connection_t object
     * and will call xcb_disconnect when the Connection goes out of scope
     */

    println!("Connecting to X server");

    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();

    dbg!(screen_num);



    //let ewmh_conn = ewmh::Connection::connect(conn);

    /*
     * Accessor for the data returned by the server when the Connection was initialized.
     */

    //let setup = conn.get_setup();

    /*
     * Get the number of screens.
     */

    let screen = conn
        .get_setup()
        .roots()
        .nth(screen_num as usize)
        .unwrap();

    /*
     * Allocates an XID for a new object. Typically used just prior to various object creation
     * functions, such as xcb::create_window.
     *
     * The ID with which you will refer to the new window, created by xcb_generate_id.
     */

    let wid = conn.generate_id();

    /*
        xcb_void_cookie_t xcb_create_window (xcb_connection_t *connection,    
            int8_t           depth,         /* Depth of the screen */
            cb_window_t      wid,           /* Id of the window */
            cb_window_t      parent,        /* Id of an existing window that should be the parent of the new window */
            nt16_t           x,             /* X position of the top-left corner of the window (in pixels) */
            nt16_t           y,             /* Y position of the top-left corner of the window (in pixels) */
            int16_t          width,         /* Width of the window (in pixels) */
            int16_t          height,        /* Height of the window (in pixels) */
            int16_t          border_width,  /* Width of the window's border (in pixels) */

        pub fn create_window(
            c: &Connection,
            depth: u8,
            wid: Window,
            parent: Window,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u16,
            visual: Visualid,
            value_list: &[(u32, u32)]) -> VoidCookie
    */
            
    /*
     * https://rtbo.github.io/rust-xcb/xcb/xproto/fn.create_window.html
     */
    xcb::create_window(
        &conn, /* The connection object to the server */
        xcb::COPY_FROM_PARENT as u8, // depth
        wid, // window_id
        screen.root(), // parent
        0, // x
        0, // y
        150, // width
        150, // height
        10, // border_width
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16, // class
        screen.root_visual(), // visual_id (from parent)
        &[ // value list
            (xcb::CW_BACK_PIXEL, screen.white_pixel()),
            (xcb::CW_EVENT_MASK,
             xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
        ]
    );

    //let y = match self.position {
    //    Position::Top => 0,
    //    Position::Bottom => self.screen()?.height_in_pixels() - self.height,
    //};

    let y = screen.height_in_pixels() - 35;

    // Update the height/position of the XCB window and the height of the Cairo surface.

    let values = [
        (xcb::CONFIG_WINDOW_Y as u16, u32::from(y)),
        (xcb::CONFIG_WINDOW_HEIGHT as u16, 35),
        (xcb::CONFIG_WINDOW_STACK_MODE as u16, xcb::STACK_MODE_ABOVE),
    ];

    xcb::configure_window(&conn, wid, &values);

    xcb::map_window(&conn, wid);

    conn.flush();

    loop {
        let event = conn.wait_for_event();
        match event {
            None => { break; }
            Some(event) => {
                let r = event.response_type() & !0x80;
                match r {
                    xcb::EXPOSE => {

                        /* We flush the request */
                        conn.flush();

                    },
                    xcb::KEY_PRESS => {
                        let key_press : &xcb::KeyPressEvent = unsafe {
                            xcb::cast_event(&event)
                        };
                        println!("Key '{}' pressed", key_press.detail());
                        break;
                    },
                    _ => {}
                }
            }
        }
    }
}
