use esp_idf_svc::hal::gpio::{PinDriver, Output, Level, Gpio0, Gpio1};
use esp_idf_svc::hal::gpio::Level::{Low, High};
use std::{thread::sleep, time::Duration};

pub struct I2CBus<'a, >{
    pub scl: PinDriver<'a, Gpio0, Output> ,
    pub sda: PinDriver<'a, Gpio1, Output> ,
}

impl I2CBus<'_> {

    pub fn send_start_cond(mut self)->Self{
        self.sda.set_high().unwrap();
        sleep(Duration::from_millis(10));
        self.scl.set_high().unwrap();
        sleep(Duration::from_millis(10));
        self.sda.set_low().unwrap();
        sleep(Duration::from_millis(10));
        self.scl.set_low().unwrap();
        sleep(Duration::from_millis(10));
        self
    }
    pub fn send_stop_cond(mut self)->Self{
        self.scl.set_low().unwrap();
        sleep(Duration::from_millis(10));
        self.sda.set_low().unwrap();
        sleep(Duration::from_millis(10));
        self.scl.set_high().unwrap();
        sleep(Duration::from_millis(10));
        self.sda.set_high().unwrap();
        sleep(Duration::from_millis(10));
        self
    }
 
    pub fn send_ack(mut self, data:u8)->Self{
        self = self.send_bit(data);
        self
    }
    pub fn send_bit(mut self, data:u8)->Self{
        self.sda.set_low().unwrap();
        sleep(Duration::from_millis(1));
        self.scl.set_low().unwrap();
        sleep(Duration::from_millis(1));
        if data == 1 {
           self.sda.set_high().unwrap();
        }else{
           self.sda.set_low().unwrap();
        }
        sleep(Duration::from_millis(1));
        self.scl.set_high().unwrap();
        sleep(Duration::from_millis(1));
        self.scl.set_low().unwrap();
        sleep(Duration::from_millis(1));
        self.sda.set_low().unwrap();
        sleep(Duration::from_millis(1));
        self
    }
  
    pub fn send_byte(mut self, data:u8) -> Self {
        let mut mask = 0x80;
        //print!("send_byte()\n");
        //print!("{:x}\n",data);
        for _ in 0..8 {
            if data & mask != 0{
                self = self.send_bit(1);
            }else{
                self = self.send_bit(0);
            }
            mask >>= 1;
        }
        self
    }
  
    pub fn send_device_address_rw(mut self, device_address:u8, rw:u8)->Self{
        let data = (device_address << 1) | rw;
        self = self.send_byte(data);
        self
    }
  
    pub fn recv_ack(mut self)->(Self,Level){   
        self.scl.set_low().unwrap();
        let sda_in = self.sda.into_input().unwrap();
        sleep(Duration::from_millis(1));
        self.scl.set_high().unwrap();
        sleep(Duration::from_millis(1));
        let level = sda_in.get_level(); 
        sleep(Duration::from_millis(1));
        self.scl.set_low().unwrap();
        sleep(Duration::from_millis(1));
        self.sda = sda_in.into_output().unwrap();
        self.sda.set_low().unwrap();
        if level == High {
     	   print!("NACK<<<<<<<<<<<<<\n");
        }
        (self, level)
    }  
  
    pub fn recv_byte(mut self)->(Self,u8){
        let mut data:u8 = 0;
        let sda_in = self.sda.into_input().unwrap();
        for _ in 0..8 {
            self.scl.set_low().unwrap();
            sleep(Duration::from_millis(1));
            self.scl.set_high().unwrap();
            sleep(Duration::from_millis(1));
            data <<= 1;
            if sda_in.get_level() == High {
                data += 1;
            }
            sleep(Duration::from_millis(1));
            self.scl.set_low().unwrap();
            sleep(Duration::from_millis(1));
        }
        self.sda = sda_in.into_output().unwrap();
        self.sda.set_low().unwrap();
        (self, data)
    }  
  
    pub fn read_registers(mut self, device_address:u8, mut register_address:u8, size:u8) -> (Self,Vec<u8>){
  
        let mut values = Vec::<u8>::new();  
        for _ in 0..size{
            let data:u8;
            (self, data) = self.read_register(device_address, register_address);
            values.push(data);
            register_address += 1;
        }
        (self, values)
    } 
  
    pub fn read_register(mut self, device_address:u8, register_address:u8)->(Self,u8){

        let mut level:Level; 
        let data: u8;
  
        self = self.send_start_cond();
        self = self.send_device_address_rw(device_address, 0);   // 0: write
        (self, level) = self.recv_ack(); //(1/4)
        self = self.send_byte(register_address);        
        (self, level) = self.recv_ack(); //(2/4)
        self = self.send_start_cond();            // start cond
        self = self.send_device_address_rw(device_address, 1);   // 1: read
        (self, level) = self.recv_ack(); //(3/4)
        (self, data) = self.recv_byte();
        self = self.send_ack(1);         //(4/4)       // 1: nack
        self = self.send_stop_cond();
        (self, data)
    }
  
    pub fn write_register(mut self, device_address:u8, register_address:u8, data:u8) -> Self{

        let mut level:Level;
  
        self = self.send_start_cond();
        self = self.send_device_address_rw(device_address, 0);   // 0: write
        (self, level) = self.recv_ack(); //(1/4)
        if level == High {
          print!("NACK<<<<<<<<<<<<<\n");
        }
        self = self.send_byte(register_address);        
        (self, level) = self.recv_ack(); //(2/4)
        if level == High {
          print!("NACK<<<<<<<<<<<<<\n");
        }
        //print!("send:{:x}\n",data);
        self = self.send_byte(data);        
        (self, level) = self.recv_ack(); //(3/4)
        if level == High {
          print!("NACK<<<<<<<<<<<<<\n");
        }
        self = self.send_stop_cond();
        self
    }
  
}

  
  