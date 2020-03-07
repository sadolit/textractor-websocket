use std::string::String;
use std::thread::spawn;
use std::time::Duration;

use crossbeam_channel::{bounded, Sender};
use once_cell::sync::OnceCell;

pub const ADDRESS: &str = "0.0.0.0:6677";
const BOUND: usize = 1000;
const SEND_TIMEOUT: Duration = Duration::from_secs(1);

fn run_once() -> Sender<String> {
    let (sender, receiver) = bounded(BOUND);
    let me = ws::WebSocket::new(|_| move |_| Ok(())).unwrap();
    let broacaster = me.broadcaster();
    spawn(move || me.listen(ADDRESS).unwrap());
    spawn(move || loop {
        let msg = receiver.recv().unwrap();
        broacaster.broadcast(msg).unwrap();
    });
    sender
}

fn get_sender() -> &'static Sender<String> {
    static INSTANCE: OnceCell<Sender<String>> = OnceCell::new();
    INSTANCE.get_or_init(run_once)
}

pub(crate) fn handle(s: String) {
    let sender = get_sender();
    sender.send_timeout(s, SEND_TIMEOUT).unwrap();
}
