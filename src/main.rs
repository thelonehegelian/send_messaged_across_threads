use crossbeam;
use crossbeam_channel::unbounded;
use std::{thread, time};

fn main() {
    let (sender, receiver) = unbounded();

    let num_messages = 10;
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..num_messages {
                sender.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        });
    })
    .unwrap();

    for _ in 0..num_messages {
        let received = receiver.recv().unwrap();
        println!("received: {}", received);
    }
}
