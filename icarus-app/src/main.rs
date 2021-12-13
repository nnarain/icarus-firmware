//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 06 2021
//
#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = icarus::hal::pac, peripherals = true, dispatchers = [EXTI3, EXTI4])]
mod app {
    use icarus::{
        prelude::*,
        // cortex_m,
        hal::{
            serial::Event,
            Toggle,
        },
        types::{PinStat1, PinStat2, Serial1},
    };

    use icarus_comms::{
        IcarusCommand,
        ppp::{
            ReceiveQueue,
            spsc::{Queue, Producer, Consumer}
        }
    };

    use systick_monotonic::*;

    const SERIAL_QUEUE_SIZE: usize = 50;

    #[monotonic(binds = SysTick, default = true)]
    type IcarusMono = Systick<100>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        stat1: PinStat1,
        stat2: PinStat2,

        serial1: Serial1,

        serial_producer: Producer<'static, u8, SERIAL_QUEUE_SIZE>,
        serial_consumer: Consumer<'static, u8, SERIAL_QUEUE_SIZE>,

        cmd_recv_queue: ReceiveQueue<'static, IcarusCommand, 50, 3>,
        cmd_consumer: Consumer<'static, IcarusCommand, 3>,
    }

    #[
        init(
            local = [
                serial_queue: Queue<u8, SERIAL_QUEUE_SIZE> = Queue::new(),
                cmd_queue: Queue<IcarusCommand, 3> = Queue::new()
            ]
        )
     ]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let systick = cx.core.SYST;
        let mono = Systick::new(systick, 8_000_000);

        // Initialize hardware
        let hw = Icarus::new(cx.device).unwrap();

        // LED indicators
        let stat1 = hw.stat1;
        let stat2 = hw.stat2;

        // Serial port 1. Configure interrupt for data receive
        let mut serial1 = hw.usart1;
        serial1.configure_interrupt(Event::ReceiveDataRegisterNotEmpty, Toggle::On);

        // Queue for serial data
        let (serial_producer, serial_consumer) = cx.local.serial_queue.split();

        // Queue for commands
        let (cmd_producer, cmd_consumer) = cx.local.cmd_queue.split();
        let cmd_recv_queue = ReceiveQueue::new(cmd_producer);

        // Spawn tasks
        status_task::spawn_after(500.millis()).unwrap();

        (
            Shared {},
            Local{
                stat1,
                stat2,

                serial1,

                serial_producer,
                serial_consumer,

                cmd_recv_queue,
                cmd_consumer,
            },
            init::Monotonics(mono)
        )
    }

    ///
    /// Spawn tasks to handle incoming data and system state
    ///
    #[idle(local = [serial_consumer, cmd_recv_queue, cmd_consumer])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            // Update the receive queue with the byte read from the serial port
            if let Some(byte) = cx.local.serial_consumer.dequeue() {
                // TODO: Log error
                cx.local.cmd_recv_queue.update(byte).unwrap();
            }

            // Dispatch commands
            if let Some(cmd) = cx.local.cmd_consumer.dequeue() {
                cmd_task::spawn(cmd).unwrap();
            }
        }
    }

    #[task(local = [stat2])]
    fn cmd_task(cx: cmd_task::Context, cmd: IcarusCommand) {
        match cmd {
            IcarusCommand::LedSet(state) => {
                if state {
                    cx.local.stat2.set_high().unwrap();
                }
                else {
                    cx.local.stat2.set_low().unwrap();
                }
            },
        }
    }

    ///
    /// Show activity using the status LEDs
    ///
    #[task(local = [stat1])]
    fn status_task(cx: status_task::Context) {
        cx.local.stat1.toggle().unwrap();
        status_task::spawn_after(500.millis()).unwrap();
    }

    ///
    /// Receive data from the serial port
    ///
    #[task(binds = USART1_EXTI25, local = [serial1, serial_producer])]
    fn serial_task(cx: serial_task::Context) {
        let serial = cx.local.serial1;

        if serial.is_event_triggered(Event::ReceiveDataRegisterNotEmpty) {
            if let Ok(byte) = serial.read() {
                cx.local.serial_producer.enqueue(byte).unwrap();
            }
        }
    }
}
