# IAQ_monitor
Indoor Air Quality (IAQ) monitorの外観<br>
マイコンはM5 Stamp(ESP32 C3)を利用、センサとして、SCD41、QMP6988をI2Cで接続<br>
<img src="https://github.com/foobarbazfred/IAQ_monitor/blob/main/img/iaq_monitor.png" width="40%"><br>
Data viewer(ThingView)<br>
データはThingSpeakに蓄積される仕様で、ThingViewを使うことでグラフ化が可能<br>
<img src="https://github.com/foobarbazfred/IAQ_monitor/blob/main/img/thingview.png" width="40%"><br>
システム構成<br>
- M5 Stampと ThingSpeakはMQTTで接続されている
- センサから取得したデータをPublishする
- 過去の履歴はThingSpeakに蓄積される
- ThingSpeakの履歴データはスマフォ上のアプリ(ThinView)で閲覧可能
<img src="https://github.com/foobarbazfred/IAQ_monitor/blob/main/img/system_architecture.png" width="50%"><br>
ソフトウエア構成<br>
センサ類を制御するドライバ、WiFi/MQTTをセットアップするmylib、周辺I/OとしてI2Cが1チャンネルしかないため、ソフトで代用するSoft I2C Driverで構成される。
<img src="https://github.com/foobarbazfred/IAQ_monitor/blob/main/img/software_architecture.png" width="50%"><br>
