use std::{
    io::{BufRead, BufReader, Write},
    process::{ChildStdin, ChildStdout},
    sync::{Mutex, MutexGuard},
};

use gtk::glib::once_cell::sync::Lazy;

use crate::snapshot_object::SnapshotData;

#[derive(Debug, Default)]
pub struct Requester {
    reader: Option<BufReader<ChildStdout>>,
    writer: Option<ChildStdin>,
}

impl Requester {
    pub fn set_reader(&mut self, reader: BufReader<ChildStdout>) {
        self.reader = Some(reader);
    }

    pub fn set_writer(&mut self, writer: ChildStdin) {
        self.writer = Some(writer);
    }

    pub fn run_btrfs(&mut self, args: &[&str]) -> String {
        let req = serde_json::to_string(args).unwrap();
        writeln!(self.writer.as_ref().unwrap(), "{}", req).unwrap();
        let mut reply = String::new();
        self.reader.as_mut().unwrap().read_line(&mut reply).unwrap();
        reply
    }

    pub fn snapshots(&mut self) -> Vec<SnapshotData> {
        let reply_json = self.run_btrfs(&["list_snapshots"]);
        serde_json::from_str(&reply_json).unwrap()
    }

    pub fn rename_snapshot(&mut self, before: &str, after: &str) -> bool {
        let reply_json = self.run_btrfs(&["rename_snapshot", before, after]);
        serde_json::from_str(&reply_json).unwrap()
    }

    pub fn delete_snapshot(&mut self, path: &str) -> bool {
        let reply_json = self.run_btrfs(&["delete_snapshot", path]);
        serde_json::from_str(&reply_json).unwrap()
    }

    pub fn create_snapshot(&mut self, src: &str, dest: &str) -> bool {
        let reply_json = self.run_btrfs(&["create_snapshot", src, dest]);
        serde_json::from_str(&reply_json).unwrap()
    }
}

static DAEMON: Lazy<Mutex<Requester>> = Lazy::new(|| Mutex::default());

pub fn daemon() -> MutexGuard<'static, Requester> {
    DAEMON.lock().unwrap()
}
