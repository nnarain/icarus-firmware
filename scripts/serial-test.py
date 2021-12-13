import serial
from argparse import ArgumentParser

parser = ArgumentParser()
parser.add_argument('-p', '--port', required=True, help='Serial port')
parser.add_argument('-b', '--baud', default=115200, type=int, help='Baud rate')
parser.add_argument('-s', '--state', default=False, type=bool, help='Led state')

args = parser.parse_args()

with serial.Serial(args.port, args.baud, timeout=1) as ser:
    ser.write(bytes([0x7E, 0x02, 0x00, 0x01, 0xFF, 0xFF]))
