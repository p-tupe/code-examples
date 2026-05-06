use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    println!("Main started");

    // if let Err(err) = thread_move().join() {
    //     return eprintln!("Error: {:?}", err);
    // }
    //
    // if let Err(err) = thread_chan().join() {
    //     return eprintln!("Error: {:?}", err);
    // }

    thread_shared();

    println!("Main ended");
}

fn thread_move() -> thread::JoinHandle<()> {
    let mut x = vec![1];

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        x[0] += 1;
        println!("value of x = {x:?}");
    })
}

fn thread_chan() -> thread::JoinHandle<()> {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        for val in vec!["hi", "from", "the", "thread"] {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in vec!["more", "msgs", "from", "another", "thread"] {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        for v in rx {
            println!("Received: {v}",);
        }
    })
}

fn thread_shared() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }

    for h in handlers {
        h.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}
