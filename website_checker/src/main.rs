use reqwest::blocking::Client;
use std::{
    env,
    fs::{self, File},
    io::{self, BufRead},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant, SystemTime},
};

// ---------- Structs ----------
struct WebsiteStatus {
    url: String,
    action_status: Result<u16, String>,
    response_time: Duration,
    timestamp: SystemTime,
}

impl WebsiteStatus {
    fn to_json_string(&self) -> String {
        format!(
            "{{\"url\":\"{}\",\"action_status\":{},\"response_time_ms\":{},\"timestamp\":\"{:?}\"}}",
            self.url,
            match &self.action_status {
                Ok(code) => code.to_string(),
                Err(err) => format!("\"{}\"", err.replace("\"", "\\\"")),
            },
            self.response_time.as_millis(),
            self.timestamp
        )
    }
}

// ---------- Worker Pool ----------
type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    sender: mpsc::Sender<Job>,
    _workers: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::new();

        for _ in 0..size {
            let rx = Arc::clone(&receiver);
            workers.push(thread::spawn(move || loop {
                let job = rx.lock().unwrap().recv();
                match job {
                    Ok(job) => job(),
                    Err(_) => break,
                }
            }));
        }

        ThreadPool {
            sender,
            _workers: workers,
        }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

// ---------- Utils ----------
fn load_urls_from_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut urls = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            urls.push(trimmed.to_string());
        }
    }

    Ok(urls)
}

fn check_website(url: String, timeout_secs: u64, retries: u32) -> WebsiteStatus {
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .unwrap();

    let mut attempts = 0;
    let start_time = Instant::now();

    loop {
        attempts += 1;
        let response = client.get(&url).send();

        let result = match response {
            Ok(resp) => Ok(resp.status().as_u16()),
            Err(e) => Err(e.to_string()),
        };

        if result.is_ok() || attempts > retries {
            break WebsiteStatus {
                url,
                action_status: result,
                response_time: start_time.elapsed(),
                timestamp: SystemTime::now(),
            };
        }

        thread::sleep(Duration::from_millis(100));
    }
}

// ---------- Main ----------
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut urls: Vec<String> = Vec::new();
    let mut file_mode = false;
    let mut file_path = "";
    let mut workers = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let mut timeout_secs = 5;
    let mut retries = 0;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                file_mode = true;
                i += 1;
                file_path = &args[i];
            }
            "--workers" => {
                i += 1;
                workers = args[i].parse().unwrap_or(workers);
            }
            "--timeout" => {
                i += 1;
                timeout_secs = args[i].parse().unwrap_or(5);
            }
            "--retries" => {
                i += 1;
                retries = args[i].parse().unwrap_or(0);
            }
            _ => {
                urls.push(args[i].clone());
            }
        }
        i += 1;
    }

    if file_mode {
        match load_urls_from_file(file_path) {
            Ok(mut file_urls) => urls.append(&mut file_urls),
            Err(e) => {
                println!("Failed to load file: {}", e);
                return;
            }
        }
    }

    if urls.is_empty() {
        println!("No URLs provided.");
        return;
    }

    println!(
        "Starting website checker with {} workers, {}s timeout, {} retries",
        workers, timeout_secs, retries
    );

    let pool = ThreadPool::new(workers);
    let (result_tx, result_rx) = mpsc::channel();

    for url in urls {
        let tx = result_tx.clone();
        let u = url.clone();
        pool.execute(move || {
            let result = check_website(u, timeout_secs, retries);
            tx.send(result).unwrap();
        });
    }

    drop(result_tx);

    let mut results = Vec::new();
    for result in result_rx {
        match &result.action_status {
            Ok(code) => println!(
                "[{}] {} OK (status {}) {} ms",
                result.timestamp
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                result.url,
                code,
                result.response_time.as_millis()
            ),
            Err(e) => println!(
                "[{}] {} ERROR: {}",
                result.timestamp
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                result.url,
                e
            ),
        }

        results.push(result);
    }

    let json_strings: Vec<String> = results.iter().map(|r| r.to_json_string()).collect();
    let json_output = format!("[\n{}\n]", json_strings.join(",\n"));
    fs::write("status.json", json_output).expect("Unable to write file");
    println!("All results written to status.json");
}
