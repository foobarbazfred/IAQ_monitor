# IAQ_monitor
### Indoor Air Quality (IAQ) monitorの外観<br>
マイコンはM5 Stamp(ESP32 C3)を利用、センサとして、SCD41、QMP6988をI2Cで接続<br>
<img src="https://github.com/foobarbazfred/IAQ_monitor/blob/main/img/iaq_monitor.png" width="40%"><br>
### データ可視化(ThingView)
データはThingSpeakに蓄積される仕様で、ThingViewを使うことでグラフ化が可能<br>
<img src="https://github.com/foobarbazfred/IAQ_monitor/blob/main/img/thingview.png" width="40%"><br>
### システム構成<br>
- M5 Stampと ThingSpeakはMQTTで接続されている
- センサから取得したデータをPublishする
- 過去の履歴はThingSpeakに蓄積される
- ThingSpeakの履歴データはスマフォ上のアプリ(ThinView)で閲覧可能
<img src="https://github.com/foobarbazfred/IAQ_monitor/blob/main/img/system_architecture.png" width="50%"><br>
### ソフトウエア構成<br>
ソフトウエアは以下で構成される
- scd41.rc　　温湿度、CO2センサ(SCD41)を制御するドライバ
- qmp6988.rc  気圧センサ(QMP6988)を制御するドライバ   
- mylib.rc  WiFi/MQTTをセットアップするサブルーチン類
- softi2c.rc  周辺I/OとしてI2Cが1チャンネルしかないため、不足分をソフト版 I2C Driverで補充
- main.rc 　システム全体を制御(センサからデータを取得してThingSpeakにPublishする)
<img src="https://github.com/foobarbazfred/IAQ_monitor/blob/main/img/software_architecture.png" width="50%"><br>
