//
//  Indoor Air Quality (IAQ) monitor 
//  v.0.0.1 (2024/7/20)
//

use std::thread;
use std::thread::sleep;
use core::time::Duration;
use log::*;

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::prelude::*; // *KHz

use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::mqtt::client::*;

//
// define for Thingspeak
//
const THINGSPEAK_URL: &str = "mqtts://mqtt3.thingspeak.com:8883";
const THINGSPEAK_CLIENT_ID: &str = "set_your_CLIENT_ID";
const THINGSPEAK_USER_NAME: &str = "set_your_USER_NAME";
const THINGSPEAK_PASSWORD: &str = "set_your_PASSWORD";
const THINGSPEAK_CHANNEL_ID: &str = "set_your_CHANNEL_ID";

mod mylib;
mod scd41;
mod qmp6988;
mod soft_i2c;

fn main() -> Result<(), EspError> {

    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();


    let peripherals = Peripherals::take()?;

    // enable I2C and setup SCD41
    let i2c = peripherals.i2c0;
    let scl = peripherals.pins.gpio18;
    let sda = peripherals.pins.gpio19;

    let config = I2cConfig::new().baudrate(10.kHz().into());
    let mut i2c = I2cDriver::new(i2c, sda, scl, &config)?;

    // sensor setup SCD41
    if let Err(e) = scd41::setup(&mut i2c){
         println!("Error in scd41 setup{}",e);
    }

    //	
    // sensor setup QMP6988
    //	
    let mut i2c_bus = soft_i2c::I2CBus{

         // M5 Stamp C3
         scl: PinDriver::output(peripherals.pins.gpio0)?,
         sda: PinDriver::output(peripherals.pins.gpio1)?,
    };

    // dummy sequence for connection stability (only send start/stop cond)
    i2c_bus = i2c_bus.send_start_cond();
    i2c_bus = i2c_bus.send_stop_cond();
    i2c_bus = i2c_bus.send_start_cond();
    i2c_bus = i2c_bus.send_stop_cond();

    let mut qmp6988 = qmp6988::Qmp6988{
       i2c : i2c_bus,
    };

    // connection check
    let status:u8;
    (qmp6988, status) = qmp6988.connection_check();
    if status == 1 {
        print!("QMP6988: Connection OK\n");
    }else{
        print!("QMP6988: Connection Error\n");
    }
    qmp6988 = qmp6988.start();

    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let modem = peripherals.modem;
    let _wifi = mylib::wifi_create(modem, &sys_loop, &nvs).unwrap();

    info!("wait for 3sec");
    sleep(Duration::from_secs(3));  // wait for 10 sec

    let (mut client, mut connection) = mylib::mqtt_create(THINGSPEAK_URL, THINGSPEAK_CLIENT_ID, THINGSPEAK_USER_NAME, THINGSPEAK_PASSWORD).unwrap();
    thread::spawn(move || {
       info!("MQTT Listening for messages");
       while let Ok(event) = connection.next() {
             info!("[Queue] Event: {}",event.payload());
       }
       info!("MQTT connection loop exit");
   });

    info!("Connected to MQTT");
    let topic: &str = "channels/{THINGSPEAK_CHANNEL_ID}/publish";  

    info!("wait for 3sec");
    sleep(Duration::from_secs(3));  // wait for 10 sec
    info!("try to Subscribe to topics...");

    if let Err(e) = client.subscribe(topic, QoS::AtMostOnce) {
       error!("Failed to subscribe to topic \"{topic}\": {e} ");
    }

    info!("Subscribed to topic \"{topic}\"");
    let topics = format!("channels/{}/publish", THINGSPEAK_CHANNEL_ID);
    info!("topics: {}",topics);

    //
    // get sensor data and upload to Thingspeak by MQTT
    //
    loop{

        let mut hum:f32 = 0.0;
        let mut co2:u16 = 0;
        let tmp:f32;
        let press:f32;

        // get sensor value (SCD41)
        match scd41::read_measurement(&mut i2c){
            Err(e) => println!("Error in read_measurement{}",e),
            Ok(v) => (_, hum, co2) = v,    // measured temperature value is not used
        }

        // get sensor value (QMP6988)
        (qmp6988, tmp, press) = qmp6988.get_temp_press();
        print!("tmp:{}, pr:{}hPa\n",tmp,press);

        let payload = format!("field1={}&field2={}&field3={}&field4={}", tmp, hum, press, co2);
        info!("payload: {}",payload);
        client.publish(&topics, QoS::AtMostOnce, false, payload.as_bytes())?;
        sleep(Duration::from_secs(20 * 60));  // wait for 20 min

    }
}






