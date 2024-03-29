import paho.mqtt.client as mqtt
import time


# Установите адрес вашего MQTT-брокера
mqtt_broker_host = "broker.emqx.io"
mqtt_broker_port = 1883
mqtt_topic = "mytopic"


def on_connect(client, userdata, flags, rc):
    print(f"Connected with result code {rc}")


client = mqtt.Client()
client.on_connect = on_connect

# Теперь у вас есть переменная mqtt_broker_host с адресом брокера
client.connect(mqtt_broker_host, mqtt_broker_port, 60)

while True:
    message = input("Введите сообщение для отправки: ")
    client.publish(mqtt_topic, message)
    print(f"Отправлено сообщение: {message}")
    time.sleep(1)
