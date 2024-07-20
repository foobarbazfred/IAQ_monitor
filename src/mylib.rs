//
//  mylib.rs
//
use log::info;
use core::time::Duration;

use esp_idf_svc::sys::EspError;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
//use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::peripheral::Peripheral;


use esp_idf_svc::wifi::*;
use esp_idf_svc::hal::modem::WifiModemPeripheral;

use esp_idf_svc::mqtt::client::EspMqttClient;
use esp_idf_svc::mqtt::client::EspMqttConnection;
use esp_idf_svc::mqtt::client::MqttClientConfiguration;


const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");

pub fn wifi_create<P: Peripheral + 'static>(modem: P, sys_loop: &EspSystemEventLoop, nvs: &EspDefaultNvsPartition,) -> Result<EspWifi<'static>, EspError> where <P as Peripheral>::P: WifiModemPeripheral {

    let mut esp_wifi = EspWifi::new(modem, sys_loop.clone(), Some(nvs.clone()))?;
    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sys_loop.clone())?;
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        password: PASSWORD.try_into().unwrap(),
        ..Default::default()
    }))?;

    wifi.start()?;
    info!("Wifi started");

    wifi.connect()?;
    info!("Wifi connected");

    wifi.wait_netif_up()?;
    info!("Wifi netif up");

    Ok(esp_wifi)
}

pub fn mqtt_create(url: &str, client_id: &str, user_name: &str, password: &str ) -> Result<(EspMqttClient<'static>, EspMqttConnection), EspError> {
    let (mqtt_client, mqtt_conn) = EspMqttClient::new(
        url,
        &MqttClientConfiguration {
            crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
            keep_alive_interval: Some(Duration::from_secs(2 * 60)),   // 2min
            client_id: Some(client_id),
            username: Some(user_name),
            password: Some(password),
            ..Default::default()
        },
    )?;

    Ok((mqtt_client, mqtt_conn))
}



