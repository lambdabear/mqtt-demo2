use rumqtt::{MqttClient, MqttOptions, QoS, ReconnectOptions};
use std::sync::{Arc, Mutex};
use std::{io, io::Write, thread, time::Duration};

fn main() {
    //pretty_env_logger::init();
    //let broker = "prod-mqtt-broker.atherengineering.in";
    let broker = "test.mosquitto.org";
    let port = 1883;

    let reconnection_options = ReconnectOptions::Always(10);
    let mqtt_options = MqttOptions::new("hello-spb-msg", broker, port)
        .set_keep_alive(10)
        .set_reconnect_opts(reconnection_options)
        .set_clean_session(false);

    let (mqtt_client, _notifications) = MqttClient::start(mqtt_options).unwrap();

    let client: Arc<Mutex<Option<MqttClient>>> = Arc::new(Mutex::new(None));
    let c1 = client.clone();
    let c2 = client.clone();
    // mqtt_client
    //     .subscribe("hello/world", QoS::AtLeastOnce)
    //     .unwrap();

    // thread::spawn(move || {
    //     for i in 0..100 {
    //         let payload = format!("publish {}", i);
    //         thread::sleep(Duration::from_millis(100));
    //         mqtt_client
    //             .publish("hello/world", QoS::AtLeastOnce, false, payload)
    //             .unwrap();
    //     }
    // });

    // for notification in notifications {
    //     match notification {
    //         rumqtt::client::Notification::Publish(publish) => {
    //             io::stdout().write_all(&publish.payload).unwrap();
    //             print!("\n")
    //         }
    //         _ => (),
    //     }
    // }

    let handler = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));
        match c1.lock().unwrap().as_mut() {
            Some(c) => {
                println!("send msg");
                thread::sleep(Duration::from_millis(500));
                c.publish(
                    "hello/world",
                    QoS::AtLeastOnce,
                    false,
                    String::from("hello!!!!!!!!!!!").into_bytes(),
                )
                .unwrap();
            }
            None => {
                println!("mqtt client is not ready");
                thread::sleep(Duration::from_millis(1000))
            }
        }
    });

    thread::sleep(Duration::from_secs(2));
    loop {
        {
            let mut c = c2.lock().unwrap();
            match *c {
                Some(_) => *c = None,
                None => {
                    *c = Some(mqtt_client.clone());
                }
            }
        }
        thread::sleep(Duration::from_secs(2));
    }
}
