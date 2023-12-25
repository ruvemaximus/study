use zmq;
use sqlite;
use std::{fs::OpenOptions, io::Write, time::UNIX_EPOCH};

use std::time::{Duration, SystemTime};
use std::thread::sleep;


const DB_URL: &str = "./zmq-client.db";
const CREATE_EVENTS_TABLE: &str = "CREATE TABLE IF NOT EXISTS events(time TEXT, type TEXT, value INT)"; 
const INSERT_LOG_QUERY: &str = "INSERT INTO events(time,type,value) VALUES(:time,:type,:value)";


fn log(level: &str, message: String) {
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open("zmq-client.log").unwrap();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let message = format!("{level}::{current_time}::{message}\n");
    f.write_all(&message.as_bytes()).unwrap();
}


fn main() {
    let conn = sqlite::open(DB_URL).unwrap();
    conn.execute(CREATE_EVENTS_TABLE).unwrap();

    let context = zmq::Context::new();
    let client = context.socket(zmq::SUB).unwrap();
    client.connect("tcp://127.0.0.1:5555")
        .expect("Failed to connect");

    log("info", "Zmq client started!".to_string());

    loop {
        client.set_subscribe(b"").unwrap_or({
            log("warn", "Subscribtion failed... Waiting 500ms".to_string());
            sleep(Duration::from_millis(500));
        });

        loop {
            let message = client.recv_string(0).unwrap().unwrap();
            let event: Vec<&str> = message.split(' ').collect();

            if event.len() < 1 || event.len() > 2 {
                log("error", "Unexpected event type".to_string());
                continue;
            }

            let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

            let value = match event.len() {
                1 => "-1",
                2 => event[1],
                _ => unreachable!("Unexpected event word count")
            };

            let mut stmt = conn.prepare(INSERT_LOG_QUERY).unwrap();
            stmt.bind::<&[(_, sqlite::Value)]>(&[
                (":time", current_time.to_string().into()),
                (":type", event[0].into()),
                (":value", value.into())
            ][..]).unwrap();

            stmt.next().unwrap();
            log("info", format!("recieved: \"{message}\""));
        }
    }
}
