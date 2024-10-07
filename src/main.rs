use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let(tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move ||{
        let messages = vec![
            "thread 1: Hi",
            "thread 1: from",
            "thread 1: the",
            "thread 1: thread 1",
        ];
        for message in messages{
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_millis(1));
        } 
    });
    thread::spawn(move ||{
        let messages = vec![
            "thread 2: Hi",
            "thread 2: from",
            "thread 2: the",
            "thread 2: thread 2",
        ];
        for message in messages{
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(1));
        } 
    });

    for rec in rx{
        println!("{rec}");
    }
}
