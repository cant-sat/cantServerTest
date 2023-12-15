use std::f64::consts::PI;
use std::{any, string};
use std::thread::sleep;
use std::time::Duration;

use json::{object, stringify};
use websocket::{client, url::Url, ClientBuilder, Message};

fn main() {
    let mut client = ClientBuilder::new("ws://server.cant.hu:443")
        .unwrap()
        .connect_insecure()
        .unwrap();


        // send messages!
        let token = Message::text("asd");
        client.send_message(&token).unwrap();

    // Define parameters for the sine wave
    let amplitude = 1.0;
    let frequency = 1.0; // in Hertz
    let duration_seconds = 7.0;
    let sample_rate = 200.0; // in samples per second

    // Calculate the number of samples
    let num_samples = (duration_seconds * sample_rate) as usize;

    

    // Generate and print the values of the sine wave
    for i in 0..num_samples {
        let time = i as f64 / sample_rate as f64;
        let value = amplitude * (2.0 * PI * frequency * time).sin();
        let entrie = entrie {
            table: "sin".to_string(),
            value: value,
        };

        let data = entrieString(entrie);

        // send messages!
        let message = Message::text(data);
        client.send_message(&message).unwrap();

        let sleepTime: Duration = Duration::from_millis(((duration_seconds / sample_rate) * 1000.0) as u64);
        sleep(sleepTime);
    }
}

struct entrie {
    table: String,
    value: f64,
}

fn entrieString(toStringify: entrie) -> String {
    let ret = object! {
        table : toStringify.table,
        value : toStringify.value
    };
    return stringify(ret);
}
