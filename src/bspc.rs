#![allow(unused)]

use std::{
    io::{BufRead, BufReader, Read, Write},
    os::unix::net::UnixStream,
    process::{ChildStdout, Command, Stdio},
    rc::Rc,
    time::{Duration, SystemTime},
};

use serde::Deserialize;

use crate::x11::{self, get_client_name};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WmState {
    pub focused_monitor_id: i32,
    pub clients_count: i32,
    pub monitors: Vec<Monitor>,
    // pub focus_history: Vec<FocusHistoryItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FocusHistoryItem {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub name: String,
    pub id: i32,
    pub randr_id: i32,
    pub focused_desktop_id: i32,
    pub desktops: Vec<Desktop>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Desktop {
    pub name: String,
    pub id: i32,
    pub focused_node_id: i32,
    pub root: Option<Node>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: i32,
    pub client: Option<Client>,
    pub first_child: Option<Box<Node>>,
    pub second_child: Option<Box<Node>>,
    pub hidden: bool,
    pub sticky: bool,
    pub private: bool,
    pub locked: bool,
    pub marked: bool,
    pub split_type: String,
    pub split_ratio: f64,
}

impl Node {
    pub fn get_wm_name(&self) -> Option<String> {
        x11::get_client_name(self.id as u32)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub class_name: String,
    pub urgent: bool,
    pub shown: bool,
}

pub fn dump_wm() -> anyhow::Result<WmState> {
    let start = std::time::Instant::now();
    let mut socket = UnixStream::connect(x11::get_bspwm_socket_path())?;
    socket.write_all(b"wm\0-d\0");
    let res: WmState = serde_yaml::from_reader(socket)?;
    let end = SystemTime::now();
    Ok(res)
}
