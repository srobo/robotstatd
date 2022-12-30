mod astoria;
mod messages;
mod stats;

use std::process::exit;
use std::time::{Duration};
use std::thread::sleep;
use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
use mqtt::{AsyncClient, Message};
use sysinfo::{System, SystemExt};
use crate::messages::{StatsMessage};
extern crate paho_mqtt as mqtt;

fn main() {
    let mut sys = System::new_all();
    let usercode_pid = Arc::new(AtomicU32::new(0));
    let mqtt_client = AsyncClient::new("tcp://localhost:1883").unwrap_or_else(|err| {
        eprintln!("Error connecting to MQTT broker, exiting.");
        eprintln!("{}", err);
        exit(1);
    });
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    mqtt_client.connect(conn_opts).wait().expect("Failed to connect to MQTT broker");

    let usercode_pid_clone = usercode_pid.clone();
    mqtt_client.set_message_callback(move |_client, message| {
        let msg = serde_json::from_str::<astoria::AstprocdMessage>(&*message.unwrap().payload_str());

        match msg {
            Ok(message) => {
                if let Some(pid) = message.pid {
                    usercode_pid_clone.store(pid, Ordering::SeqCst);
                    println!("set pid to {:?}", usercode_pid_clone);
                } else {
                    usercode_pid_clone.store(0, Ordering::SeqCst);
                    println!("reset pid to 0");
                }
            },
            Err(e) => {
                eprintln!("Failed to parse astprocd message: {:?}", e);
                usercode_pid_clone.store(0, Ordering::SeqCst);
                println!("reset pid to 0");
            }
        }
    });

    match mqtt_client.subscribe("astoria/astprocd", 0).wait() {
        Ok(_) => {println!("Subscribed to astprocd messages.")}
        Err(e) => {
            eprintln!("Failed to subscribe to astprocd messages: {:?}", e);
            exit(2)
        }
    }

    loop {
        let msg = Message::new("robotstat/heartbeat", serde_json::to_string(&StatsMessage {
            cpu_usage: stats::get_cpu_usage(&mut sys),
            memory: stats::get_mem_usage(&mut sys, usercode_pid.load(Ordering::SeqCst)),
        }).unwrap(), 0);
        println!("published stats for pid {:?}", usercode_pid);
        let res = mqtt_client.publish(msg);

        match res.wait() {
            Ok(..) => {},
            Err(e) => eprintln!("Failed to publish state: {:?}", e)
        }

        sleep(Duration::from_secs(1));
    }
}
