use rw;

#[tokio::main]
async fn main() {
    let bar = match rw::Bar::new(rw::Position::Bottom) {
        Ok(bar) => {
            println!("bar created");
            loop {
                println!("loop");
                let event = match bar.conn.wait_for_event() {
                    Some(event) => {
                        let r = event.response_type() & !0x80;
                        match r {
                            xcb::EXPOSE => {
                                println!("xcb:EXPOSE {}", r);
                            }
                            xcb::KEY_PRESS => {
                                let key_press: &xcb::KeyPressEvent =
                                    unsafe { xcb::cast_event(&event) };
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
