use nix::sys::select::FdSet;
use polybar_iconography::bspc::{self, Node, WmState};
use polybar_iconography::settings::{get_settings, Settings};
use polybar_iconography::x11;
use std::hash::Hash;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;

const SUBSCRIPTS: [&str; 10] = ["₁", "₂", "₃", "₄", "₅", "₆", "₇", "₈", "₉", "₀"];
const SUPERSCRIPTS: [&str; 11] = ["⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹", "ⁿ"];

fn group_by<T, P, T2>(input: &[T], selector: P) -> Vec<Vec<&T>>
where
    T2: PartialOrd + Eq + Hash + Sized,
    P: Fn(&T) -> T2,
{
    let mut result: Vec<Vec<&T>> = vec![];
    for item in input.iter() {
        let key_1 = selector(item);
        let pos = result.iter().position(|p| {
            let key_2 = selector(p[0]);
            std::cmp::Ordering::Equal == key_1.partial_cmp(&key_2).unwrap()
        });
        let idx = if let Some(i) = pos {
            i
        } else {
            result.push(vec![]);
            result.len() - 1
        };
        result[idx].push(item);
    }
    result
}

fn get_client_nodes(root: &Node) -> Vec<&Node> {
    let mut result = vec![];
    if root.client.is_some() {
        result.push(root);
    }
    if let Some(ref first) = root.first_child {
        result.extend(get_client_nodes(first));
    }
    if let Some(ref second) = root.second_child {
        result.extend(get_client_nodes(second));
    }
    result
}

fn render(state: WmState, settings: &Settings) {
    let mut d = 0;
    let mut result = vec![];
    for (_m, monitor) in state.monitors.into_iter().enumerate() {
        if let Some(ref monitor_name) = settings.monitor {
            if !monitor.name.eq(monitor_name) {
                d += monitor.desktops.len();
                continue;
            }
        }

        let mut desktop_strings = vec![];
        let monitor_focused = state.focused_monitor_id == monitor.id;
        for desktop in monitor.desktops.iter() {
            let subscript = SUBSCRIPTS.get(d).cloned().unwrap_or("?");
            d += 1;
            let desktop_focused = monitor_focused && desktop.id == monitor.focused_desktop_id;
            let clients = desktop
                .root
                .as_ref()
                .map(get_client_nodes)
                .unwrap_or(vec![]);

            if clients.is_empty() && !desktop_focused {
                continue;
            }

            let mut desktop_string = String::new();
            desktop_string.push_str(subscript);
            desktop_string.push_str(settings.draw_settings.prefix.as_deref().unwrap_or(" "));
            let groups = group_by(&clients, |c| settings.icons.get_icon(c));
            for group in groups {
                let superscript = if group.len() > 1 {
                    SUPERSCRIPTS
                        .get(group.len())
                        .or(SUPERSCRIPTS.last())
                        .unwrap()
                } else {
                    " "
                };
                let group_focused =
                    desktop_focused && group.iter().any(|g| g.id == desktop.focused_node_id);
                let alert = group.iter().any(|g| g.client.as_ref().unwrap().urgent);
                let icon = settings.icons.get_icon(group[0]);

                let this_str = format!("{} {}", icon, superscript);
                let formatter = if alert {
                    &settings.draw_settings.urgent_node_draw_mode
                } else if group_focused {
                    &settings.draw_settings.focused_node_draw_mode
                } else {
                    &settings.draw_settings.node_draw_mode
                };
                let focus_link = format!("bspc node -f {}", group[0].id);
                desktop_string.push_str(&formatter.format(this_str, Some(&focus_link)));
            }
            if let Some(post) = settings.draw_settings.postfix.as_deref() {
                desktop_string.push_str(post);
            }
            let formatter = if desktop_focused {
                &settings.draw_settings.focused_workspace_draw_mode
            } else {
                &settings.draw_settings.workspace_draw_mode
            };
            desktop_strings.push(formatter.format(
                desktop_string,
                Some(&format!("bspc desktop -f '{}'", desktop.name)),
            ));
        }
        if !desktop_strings.is_empty() {
            let formatter = &settings.draw_settings.workspace_draw_mode;
            let separator = &settings.draw_settings.separator;
            let desktop = desktop_strings
                .join(&formatter.format(separator, None))
                .to_string();
            result.push(formatter.format(
                desktop,
                Some(&format!("bspc monitor -f '{}'", monitor.name)),
            ));
        }
    }
    println!("{}", result.join(""));
}

fn main() {
    let settings = get_settings();
    let mut socket = UnixStream::connect(x11::get_bspwm_socket_path()).unwrap();
    let mut msg = [
        "subscribe",
        "node_add",
        "node_remove",
        "node_focus",
        "node_flag",
        "desktop_focus",
    ]
    .join("\0");
    msg.push('\0');
    socket
        .write_all(msg.as_bytes())
        .expect("Failed to write to socket.");
    // socket .set_read_timeout(Some(Duration::from_millis( settings.icons.tick_rate.unwrap_or(1000),))) .expect("Failed to set socket read timeout???");
    socket
        .set_nonblocking(true)
        .expect("Failed to set socket to non-blocking mode");

    if let Ok(state) = bspc::dump_wm() {
        render(state, &settings);
    }

    let socket_fd = socket.as_raw_fd();
    let x11_fd = x11::get_raw_fd();

    let mut buf = [0; 100];
    loop {
        let mut fds = FdSet::new();
        fds.insert(socket_fd);
        fds.insert(x11_fd);

        // Block until either an event has ocurred in the X client, or until new data is available
        // in the socket. We don't care about the actual data, so we just discard everything
        // immediately on either connection.
        if nix::sys::select::select(None, &mut fds, None, None, None).is_ok() {
            while let Ok(Some(_)) = x11::poll_event() {}
            while socket.read(&mut buf).is_ok() {}
        }

        // Now that both connection should be blocked again, we can get the current wm state
        match bspc::dump_wm() {
            Ok(state) => {
                render(state, &settings);
            }
            e => eprintln!("{:?}", e),
        }
    }
}
