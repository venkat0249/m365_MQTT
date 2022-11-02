use anyhow::Result;
use btleplug::api::{BDAddr};
use tracing_subscriber;
use tokio::io::AsyncReadExt;
use std::path::Path;
use tokio::fs::File;
use tracing::Level;
use std::env;
use tokio::{task, time};
use std::time::Duration;
use chrono::Utc;
use tracing_subscriber::fmt::format::FmtSpan;
use m365::{
  AuthToken,
  ScooterScanner,
  LoginRequest,
  ConnectionHelper,
  MiSession
};
use rumqttc::{self, AsyncClient, MqttOptions, QoS};
//use std::error::Error;
use serde_json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ScooterStatus {
	pub Id: String,
    pub charge_left: u16,
    pub Soc: u16,
    pub Amps: f32,
    pub Volts: f32,
    pub Temperature: u8,
	pub Total_distance: u32,
    pub Trip_Dist: i16,
	pub timestamp:i64
}

async fn load_token() -> Result<AuthToken> {
  let path = Path::new(".mi-token");
  tracing::debug!("Opening token: {:?}", path);

  let mut f = File::open(path).await?;
  let mut buffer : AuthToken = [0; 12];

  f.read(&mut buffer).await?;

  Ok(buffer)
}

//read and publish current status info of the scooter to mqtt borker
async fn read_send(session : &mut MiSession, client: &AsyncClient) -> Result<()> {
  //tracing::info!("  Current Speed {} km/h", session.speed().await?);
  //tracing::info!("  Motor info: {:?}", session.motor_info().await?);
  tracing::info!("  Battery info: {:?}", session.battery_info().await?);
  let dt = Utc::now();
  let statusInfo = session.battery_info().await?;
  let statusInfo2 = session.motor_info().await?;
    	let currentStatus= Person {
			Id:session.serial_number().await?,
			charge_left: statusInfo.capacity,
			Soc: statusInfo.percent,
			Amps: statusInfo.current,
			Volts: statusInfo.voltage,
			Temperature: statusInfo.temperature_1,
			Total_distance: statusInfo2.total_distance_m,
			Trip_Dist: statusInfo2.trip_distance_m,
			distance_left: session.distance_left().await?,
			timestamp:dt.timestamp()
    };
  client.publish("scooterStatus", QoS::AtMostOnce, false, serde_json::to_string(&statusInfo).unwrap())
            .await
            .unwrap();
  time::sleep(Duration::from_millis(2000)).await;
  Ok(())
}


#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()>{
  tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .with_span_events(FmtSpan::CLOSE)
    .init();
	//mqtt client
 //first arg: Mac address, 2nd arg: mqtt broker, 3rd arg: mqtt port
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 || args[1].is_empty() {
    panic!("First argument is scooter mac address");
  }
  let mut mqttoptions = MqttOptions::new("test-1", args[2], args[3].parse::<u16>().unwrap());
  mqttoptions.set_keep_alive(Duration::from_secs(5));
  let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

  let token = load_token().await?;

  let mac = BDAddr::from_str_delim(&args[1]).expect("Invalid mac address");
  tracing::info!("Searching scooter with address: {}", mac);

  let mut scanner = ScooterScanner::new().await?;
  let scooter = scanner.wait_for(&mac).await?;
  let device = scanner.peripheral(&scooter).await?;
  let connection = ConnectionHelper::new(&device);
  connection.reconnect().await?;

  let mut request = LoginRequest::new(&device, &token).await?;
  let mut session = request.start().await?;

  tracing::info!("Logged in with success, reading and sending data...");
     task::spawn(async move {
         loop{
		 //required for mqtt library
		 eventloop.poll().await;
		 }
     });
  loop {
    read_send(&mut session,&client).await;
    //time::sleep(Duration::from_millis(1000)).await;
	//let event = eventloop.poll().await;
    //println!("{:?}", event.unwrap());
  }
}
