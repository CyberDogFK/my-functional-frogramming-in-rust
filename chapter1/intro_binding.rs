use std::thread;
use std::sync::mpsc::channel;

fn thread3() {
    let (sender, receiver) = channel();
    let handle = thread::spawn(move || {
        let v = vec![1, 2, 3];
        sender.send(v).unwrap();
    });
    handle.join().ok();
    let r = receiver.recv().unwrap();
    println!("{:?}", r);
}

fn main() {
    thread3();
}
