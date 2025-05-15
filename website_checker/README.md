Website Checker

Build instructions cargo build --release

You can run all 50 urls with cargo run --release -- --file sites.txt --workers 8 --timeout 5 --retries 2

If you want to enter a specific URL you can add into the sites.txt file.
Their is a URL in the sites.txt that will fail to show that the website checker does in fact handle the case that a URL is unaccessable


output: The description of the URL will be output to 'status.json' file in which describes the naem of the URL, status, response time.


These are some examples of the URL's in the sites.txt file
{"url":"https://www.reddit.com","action_status":403,"response_time_ms":50,"timestamp":"SystemTime { tv_sec: 1747275073, tv_nsec: 776284287 }"},
{"url":"https://www.apple.com","action_status":200,"response_time_ms":123,"timestamp":"SystemTime { tv_sec: 1747275073, tv_nsec: 803501033 }"},
{"url":"https://www.google.com","action_status":200,"response_time_ms":108,"timestamp":"SystemTime { tv_sec: 1747275073, tv_nsec: 818249432 }"},