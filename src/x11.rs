use std::{
    mem,
    os::fd::{AsRawFd, RawFd},
    sync::OnceLock,
};

use xcb::{
    x::{self, Atom, EventMask, Window},
    Connection,
};

const SOCKET_ENV_VAR: &str = "BSPWM_SOCKET";

fn x_connection() -> &'static (Connection, i32) {
    static INSTANCE: OnceLock<(Connection, i32)> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let display = std::env::var("DISPLAY").ok();
        xcb::Connection::connect(display.as_deref()).unwrap()
    })
}

pub fn get_bspwm_socket_path() -> &'static str {
    static INSTANCE: OnceLock<String> = OnceLock::new();
    let s = INSTANCE.get_or_init(|| {
        if let Ok(socket) = std::env::var(SOCKET_ENV_VAR) {
            return socket;
        }
        let display = std::env::var("DISPLAY").ok();
        let (_conn, screen_num) = x_connection();
        let display = display
            .map(|mut s| {
                s.remove(0);
                s
            })
            .unwrap_or("0".to_string());
        format!("/tmp/bspwm_{}_{}-socket", display, screen_num)
    });
    s
}

fn check_atom(atom: Atom, window: Window) -> anyhow::Result<()> {
    let (conn, _) = x_connection();
    let req = xcb::x::GetAtomName { atom };
    let res = conn.send_request(&req);
    let res = conn.wait_for_reply(res)?;
    let atom_name = res.name().to_string();
    let mut getprop = xcb::x::GetProperty {
        delete: false,
        window,
        property: atom,
        r#type: xcb::x::ATOM_ANY,
        long_offset: 0,
        long_length: 0,
    };
    let typecheck = conn.wait_for_reply(conn.send_request(&getprop))?;
    let typename = conn
        .wait_for_reply(conn.send_request(&{
            x::GetAtomName {
                atom: typecheck.r#type(),
            }
        }))?
        .name()
        .to_string();
    getprop.r#type = typecheck.r#type();
    getprop.long_length = typecheck.bytes_after();
    let t = conn.wait_for_reply(conn.send_request(&getprop))?;
    let value = if t.format() == 8 {
        String::from_utf8_lossy(t.value()).to_string()
    } else {
        match t.format() {
            16 => {
                let a: &[u16] = t.value();
                format!("{:?}", a.iter().take(10).collect::<Vec<_>>())
            }
            32 => {
                let a: &[u32] = t.value();
                format!("{:?}", a.iter().take(10).collect::<Vec<_>>())
            }
            _ => panic!("Unexpected format"),
        }
    };

    println!(
        "Atom {:?} atom name {} type {:?} type name {} value {:?} width {}",
        atom,
        atom_name,
        typecheck.r#type(),
        typename,
        value,
        t.format(),
    );
    Ok(())
}

#[allow(unused)]
fn check_properties(window: Window) {
    let (conn, _) = x_connection();
    let props = xcb::x::ListProperties { window };
    let cookie = conn.send_request(&props);
    let reply = conn.wait_for_reply(cookie);

    match reply {
        Ok(props) => {
            for atom in props.atoms().iter().cloned() {
                if let Err(e) = check_atom(atom, window) {
                    eprintln!("Error checking atom: {}", e);
                }
            }
        }
        Err(e) => {
            dbg!(e);
        }
    }
}
pub fn get_client_name(client: u32) -> Option<String> {
    let (conn, _) = x_connection();

    // There is no way to construct Window from a known ID to my knowledge. Unsafe to the rescue!
    let window: Window = unsafe { mem::transmute(client) };

    watch_properties(window);

    let mut prop = x::GetProperty {
        delete: false,
        window,
        property: x::ATOM_WM_NAME,
        r#type: x::ATOM_STRING,
        long_offset: 0,
        long_length: 40,
    };
    let cookie = conn.send_request(&prop);
    let mut reply = conn.wait_for_reply(cookie);
    match reply {
        Ok(mut v) => {
            if v.format() != 8 {
                eprintln!("Unexpected character width: {}", v.format());
                return None;
            }
            // if the response contains no bytes, but indicates there are more bytes
            // to get, that means the type it returned is the actual type of the atom
            // This happens because the WM_NAME atom can be either a STRING or a UTF8_STRING.
            // In this case, we should just make another request with the type the request
            // returned.
            if v.length() == 0 && v.bytes_after() > 0 {
                prop.r#type = v.r#type();
                reply = conn.wait_for_reply(conn.send_request(&prop));
                match reply {
                    Ok(new) => {
                        v = new;
                    }
                    Err(e) => {
                        dbg!(e);
                        return None;
                    }
                }
            }
            return Some(String::from_utf8_lossy(v.value()).to_string());
        }
        Err(e) => {
            dbg!(e);
            None
        }
    }
}

fn watch_properties(window: Window) {
    let (conn, _) = x_connection();
    let value_list = vec![x::Cw::EventMask(EventMask::PROPERTY_CHANGE)];

    let ch = x::ChangeWindowAttributes {
        window,
        value_list: &value_list,
    };
    let _ = conn.send_and_check_request(&ch);
}

pub fn get_raw_fd() -> RawFd {
    x_connection().0.as_raw_fd()
}
pub fn poll_event() -> Result<Option<xcb::Event>, xcb::Error> {
    x_connection().0.poll_for_event()
}
