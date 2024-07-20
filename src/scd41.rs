//
// driver for Sensirion SCD41
//
// reference documents and source:
//    https://crates.io/crates/scd4x
//    https://github.com/hauju/scd4x-rs
//    https://github.com/adafruit/Adafruit_CircuitPython_SCD4X
//
//use esp_idf_hal::i2c::*;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::delay::{FreeRtos, BLOCK};

const SCD41_ADDRESS: u8 = 0x62;

pub fn setup(i2c : &mut I2cDriver) -> anyhow::Result<()>{  

    if let Err(e) = stop_periodic_measurement(i2c){
          println!("Error in stop_periodic_measurement{}",e);
    }
    if let Err(e) = get_serial_number(i2c){
          println!("Error in get_serial_number{}",e);
    }
    //if let Err(e) = perform_self_test(i2c){
    //      println!("Error in perform_self_test{}",e);
    //}
    if let Err(e) = start_periodic_measurement(i2c){
          println!("Error in start_periodic_measurement{}",e);
    }
    Ok(())
}


pub fn get_serial_number(i2c : &mut I2cDriver) -> anyhow::Result<()>{  

    println!("--- get serial number -----");
    for n in 0..9 {
        if let Err(e) = i2c.write(SCD41_ADDRESS, &[0x36, 0x82], BLOCK){
             println!("Error in write {}", e);
             println!("write retry {}of{}",n,9);
             FreeRtos::delay_ms(1000);
        }else{
              println!("ok write");
              break;
        }
    }    
    FreeRtos::delay_ms(500);
    let buf = &mut[0u8; 9];
    for n in 0..9 {
       if let Err(e) = i2c.read(SCD41_ADDRESS, buf, 10){
           println!("Error in  read {}", e);
           println!("retry to read {}of{}",n,9);
           FreeRtos::delay_ms(1000);
       }else{
          println!("ok read");          
          println!("values..");
          for n in 0..9 {
                print!(">>[{:02x}]",buf[n]);
          }
          println!("");
          break;
       }
    }
    Ok(())

}
pub fn start_periodic_measurement(i2c : &mut I2cDriver) -> anyhow::Result<()>{

    println!("--- start periodic measurement ----");
    for n in 0..9 {
       if let Err(e) = i2c.write(SCD41_ADDRESS, &[0x21, 0xb1], BLOCK){
            println!("Error in write {}", e);
            println!("retry to write {} of {}",n,9);
            FreeRtos::delay_ms(1000);
       }else{
             println!("ok write");
             break;
       }
    }
    Ok(())
}


pub fn stop_periodic_measurement(i2c : &mut I2cDriver) -> anyhow::Result<()>{

    println!("--- stop periodic measurement ----");
    for n in 0..9 {
       if let Err(e) = i2c.write(SCD41_ADDRESS, &[0x3f, 0x86], BLOCK){
            println!("Error in write {}", e);
            println!("retry to write {} of {}",n,9);
            FreeRtos::delay_ms(1000);
       }else{
             println!("ok write");
             break;
       }
    }
    FreeRtos::delay_ms(500);
    Ok(())
}

pub fn read_measurement(i2c : &mut I2cDriver) -> Result<(f32,f32,u16), &'static str>{

    println!("---- read measure ment -----");
    for n in 0..9 {
       if let Err(e) = i2c.write(SCD41_ADDRESS, &[0xec, 0x05], BLOCK){
            println!("Error in write {}", e);
            println!("retry to write {} of {}",n , 9);
            FreeRtos::delay_ms(1000);
       }else{
             println!("ok write");
             break;
       }
    }
    let buf = &mut[0u8; 9];
    FreeRtos::delay_ms(1000);

    for n in 0..9 {
        if let Err(e) = i2c.read(SCD41_ADDRESS, buf, 10){
             println!("Error in read {}", e);
             println!("retry to read {} of {}", n, 9);
             FreeRtos::delay_ms(1000);
        }else{
            println!("ok read");
            println!("values..");
            for n in 0..9 {
                print!(">>[{:02x}]",buf[n]);
            }
            println!("");
            break;
       }
    }

    // calculate CO2
    let mut val : u16;
    val = (buf[0] as u16) << 8;
    val |= buf[1] as u16;
    let co2 = val;

    // calculate Temperature
    val = (buf[3] as u16) << 8;
    val |= buf[4] as u16;
    let temp = -45.0 + 175.0 * (val as f32) / 65535.0;  // 65535 = 0xffff

    // calculate Humidity
    val = (buf[6] as u16) << 8;
    val |= buf[7] as u16;
    let hum = 100.0 * (val as f32)  / 65535.0 ;  // 65535 = 0xffff
    println!("temp:{:.1}, hum:{:.1}, co2:{}",temp,hum,co2);

    Ok((temp,hum,co2))
}

