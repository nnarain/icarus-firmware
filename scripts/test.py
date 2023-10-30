import serial

ser = serial.Serial('.\\COM3', 115200)
ser.write(b'hello')
