#[macro_use]
use serde::Serialize;
use std::time::SystemTime;
use std::env;

// #[derive(Serialize)]
// struct NumberResponse {
    // number: u64,
    // is_prime_number: bool,
    // execution_time_in_micros: u128
// }



// #[get("/")]
// fn index() -> &'static str {
    // "This is my Rust prime number REST API"
// }

// #[get("/isPrime?<number>")]
// fn get_is_prime(number: u64) -> Json<NumberResponse> {
    // let now = SystemTime::now();

    // Json(NumberResponse {
        // number,
        // is_prime_number: is_prime(number),
        // execution_time_in_micros: now.elapsed().unwrap().as_micros(),
    // })
// }

// fn is_prime(n: u64) -> bool {
    // if n <= 1 {
        // return false;
    // }

    // for a in 2..n {
        // if n % a == 0 {
            // return false;
        // }
    // }

    // true
// }
use tokio::{task, time};

use rumqttc::{self, AsyncClient, MqttOptions, QoS};
use std::error::Error;
use std::time::Duration;

#[derive(Serialize)]
pub struct Person {
    person_id: i32,
    person_name: String,
	time: u128
}

#[tokio::main(flavor = "multi_thread",)]
async fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();
    //let mut mqttoptions = MqttOptions::new("test-1", "broker.hivemq.com", 1883);
	println!("{:?}", args);
	println!("{}", args[1]);
		println!("{}", args[2]);
	let mut mqttoptions = MqttOptions::new("test-1", args[1].clone(), args[2].parse::<u16>().unwrap());
    mqttoptions.set_keep_alive(Duration::from_secs(45));
	
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
	    //client
        //.subscribe("scooter/Status", QoS::AtMostOnce)
        //.await
        //.unwrap();
     task::spawn(async move {
	  loop {
	 
         let event = eventloop.poll().await;
         //println!("{:?}", event.unwrap());
		//publish(&client).await;
		//time::sleep(Duration::from_millis(1000)).await;
    }
         //publish(&client).await;
        //println!("{:?}", event.unwrap());
     });

    loop {
	 
         //let event = eventloop.poll().await;
         //println!("{:?}", event.unwrap());
		publish(&client).await;
		time::sleep(Duration::from_millis(1000)).await;
    }
}


async fn publish(client: &AsyncClient) {
	//loop {
	let per= Person {
        person_id: 1,
        person_name: "Karl San".to_string(),
		time: 11
    };
	let per_json = serde_json::to_string(&per).unwrap();
        client
            .publish("scooter/Status", QoS::ExactlyOnce, false, per_json)
            .await
            .unwrap();
			//}

        //time::sleep(Duration::from_secs(1)).await;

   // time::sleep(Duration::from_secs(120)).await;
}