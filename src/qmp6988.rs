use crate::soft_i2c;

pub struct Qmp6988<'a,> {    
    pub i2c: soft_i2c::I2CBus<'a,>, 
}

impl Qmp6988<'_>{ 

    pub const QMP6988_I2C_ADDRESS: u8 = 0x70;

    pub const REG_PRESS_TXD2: u8 = 0xF7;
    pub const REG_IO_SETUP: u8 = 0xF5;
    pub const REG_CTRL_MEAS: u8 = 0xF4;
    pub const REG_DEVICE_STAT: u8 = 0xF3;
    pub const REG_CHIP_ID: u8 = 0xD1; 
    pub const QMP6988_CHIP_ID: u8 = 0x5D;   // chip ID

    const a0 :f32 = -2105.9375;
    const a1 :f32 = -0.006391493270668661;
    const a2 :f32 = -3.732941679128391e-11;
    const b00:f32 = 23489.5;
    const bt1:f32 = 0.11356932279427473;
    const bt2:f32 = 6.638398388622699e-08;
    const bp1:f32 = 0.03578676717429121;
    const b11:f32 = 2.248985259559923e-07;
    const bp2:f32 = -6.172249519333475e-10;
    const b12:f32 = 4.795419171727653e-13;
    const b21:f32 = 4.15744499038667e-16;
    const bp3:f32 = 1.2433906064027832e-16;

    pub fn connection_check(mut self)-> (Self, u8) {

        let mut values = Vec::<u8>::new();  
        let connection:u8;
        (self.i2c, values) = self.i2c.read_registers(Self::QMP6988_I2C_ADDRESS, Self::REG_CHIP_ID, 1);
        if values[0] as u8 == Self::QMP6988_CHIP_ID {
             connection = 1;
        }else{
             connection = 0;
        }

        (self, connection)
   }

    pub fn start(mut self)-> Self {

        self.i2c = self.i2c.write_register(Self::QMP6988_I2C_ADDRESS, Self::REG_CTRL_MEAS, 0x93);  // TEMP_AVE:8, PRESS_AVE:8, POWER:3
        self
   }

    pub fn read_register(mut self, addr:u8) -> (Self, u8){

        let mut values = Vec::<u8>::new();  
        (self.i2c, values) = self.i2c.read_registers(Self::QMP6988_I2C_ADDRESS, addr, 1);
        (self, values[0])

    }

    pub fn get_temp_press(mut self)->(Self, f32, f32){   // <'static>

        let mut values = Vec::<u8>::new();  
        let addr = Self::REG_PRESS_TXD2;

        (self.i2c, values) = self.i2c.read_registers(Self::QMP6988_I2C_ADDRESS, addr, 6);

        for i in 0..values.len(){
             print!("{:x}: {:x}\n", addr + i as u8, values[i]);
        }
        let pr_txd2 = values[0] as u8;
        let pr_txd1 = values[1] as u8;
        let pr_txd0 = values[2] as u8;
        let tp_txd2 = values[3] as u8;
        let tp_txd1 = values[4] as u8;
        let tp_txd0 = values[5] as u8;
        let (temp, press) = self.calc_temp_press(pr_txd2, pr_txd1, pr_txd0, tp_txd2, tp_txd1, tp_txd0);
        (self, temp, press)

    }

    fn calc_temp_press(&self, press_txd2:u8, press_txd1:u8, press_txd0:u8, temp_txd2:u8, temp_txd1:u8, temp_txd0:u8) -> (f32, f32){

        let dt = (((temp_txd2 as u32) << 16) + ((temp_txd1 as u32) << 8) + temp_txd0 as u32) as f32 - 2_u64.pow(23) as f32;
        let dp = (((press_txd2 as u32) << 16) + (( press_txd1 as u32) << 8) + press_txd0 as u32) as f32 - 2_u64.pow(23) as f32;
        let tr = Self::a0 + Self::a1 * dt + Self::a2 * dt * dt;
        let pr = Self::b00 + Self::bt1 * tr + Self::bp1 * dp + Self::b11 * tr * dp + Self::bt2 * tr * tr + Self::bp2 * dp * dp + Self::b12 * dp * tr * tr + Self::b21 * dp * dp * tr + Self::bp3 * dp * dp * dp;
        (tr/256.0, pr/100.0)
  
    }

}


