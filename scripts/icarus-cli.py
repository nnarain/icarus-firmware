import asyncio

import time
import struct
from bleak import BleakScanner, BleakClient

from argparse import ArgumentParser

async def main(args):
    name = args.name

    device = await find_device(name)

    if not device:
        print(f'Failed to find device "{name}"')
        exit(1)

    async with BleakClient(device) as client:
        try:
            while True:
                # service_data = await client.read_gatt_char('68af1093-1df9-41ac-98e8-d524a025b4b9')
                # (pitch, roll, yaw) = struct.unpack('<fff', service_data)
                # print(f'({pitch}, {roll}, {yaw})')

                throttle = (1.0, 2.0, 3.0, 4.0)
                print('writing throttle')
                await client.write_gatt_char('c346b87e-9a11-4a56-9a53-e421c8ade193', struct.pack('<ffff', throttle))
                time.sleep(1)

        except KeyboardInterrupt:
            print("exit")
            exit(0)

async def find_device(name):
    devices = await BleakScanner.discover()
    for d in devices:
        print(d)
        if d.name == name:
            return d
    return None

parser = ArgumentParser()
parser.add_argument('-n', '--name', default='icarus', help='Device name')
# parser.add_argument('list-srv', default=)

args = parser.parse_args()

asyncio.run(main(args))
