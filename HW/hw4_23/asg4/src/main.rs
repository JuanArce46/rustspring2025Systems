use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // Spawn producer threads
    let mut producers = vec![];
    for i in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let item_count = ITEM_COUNT / NUM_PRODUCERS;
        producers.push(thread::spawn(move || {
            producer(i, tx_clone, item_count);
        }));
    }

    // Spawn consumer threads
    let mut consumers = vec![];
    for i in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        consumers.push(thread::spawn(move || {
            consumer(i, rx_clone);
        }));
    }

    // Wait for all producers to finish
    for p in producers {
        p.join().unwrap();
    }

    // Send termination signal for each consumer
    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for all consumers to finish
    for c in consumers {
        c.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let num = rng.gen_range(1..=100);
        println!("Producer {} sending {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {} done", id);
}

// Consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let msg = rx.lock().unwrap().recv().unwrap();
        if msg == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal", id);
            break;
        } else {
            println!("Consumer {} received {}", id, msg);
            thread::sleep(Duration::from_millis(150));
        }
    }
}
