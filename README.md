# IAQ_monitor
Indoor Air Quality Monitoring System 
### Overview
The basic functions of an air quality monitor system are as follows:
- Sensors are used to measure room temperature, humidity, air pressure, and CO2 concentration
- Send data to IoT Platform (ThingSpeak)
- ThingSpeak accumulates historical data
- Graph display using the ThingSpeak data viewer (smartphone app; ThingView)
  
### Appearance of the Indoor Air Quality (IAQ) monitor
M5 Stamp (microcontroller is ESP32 C3(RISC-V)), and the sensors are SCD41 and QMP6988 connected via I2C.<br>
<img src="https://github.com/foobarbazfred/IAQ_monitor/blob/main/img/iaq_monitor.png" width="40%">

### Data viewer(Thingview)
The data is stored in ThingSpeak and can be graphed using Thingview.<br>
<img src="https://github.com/foobarbazfred/IAQ_monitor/blob/main/img/thingview.png" width="40%">

### System Architecture
- IAQ monitor(M5 Stamp) and ThingSpeak are connected via MQTT
- Publish the data obtained from the sensor
- Past history is stored in IoT PF; ThingSpeak
- ThingSpeak history data can be viewed using the smartphone app (Thingview)
<img src="https://github.com/foobarbazfred/IAQ_monitor/blob/main/img/system_architecture.png" width="50%">

### Software Architecture
The software consists of the following:
- scd41.rc Driver for controlling temperature, humidity and CO2 sensor (SCD41)
- qmp6988.rc Driver to control the barometric pressure sensor (QMP6988)
- mylib.rc Subroutines for setting up WiFi/MQTT
- softi2c.rc Since there is only one I2C channel as a peripheral I/O, the missing channel is supplemented with a software version of the I2C Driver.
- main.rc Controls the entire system (obtains data from sensors and publishes it to ThingSpeak)
<img src="https://github.com/foobarbazfred/IAQ_monitor/blob/main/img/software_architecture.png" width="50%">
