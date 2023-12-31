use embedded_hal::serial::Write;

pub mod uart;

fn main() {
    println!("TEST INIT!");
    let mut transmitter = uart::SoftUartTransmitter::new(
        1000,
        1000,
        uart::StopBitsOption::Two,
        uart::ParityMode::None, 
    );

    println!("Baud rate: {}",transmitter.get_baud_rate());
    println!("System clock: {}",transmitter.get_system_clock());
    println!("Clocks by iter (1 by second in simulation): {}",transmitter.get_clocks_iter());

    transmitter.write(8).unwrap();
}
