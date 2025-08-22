// SONY, FUCK YOU!

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::Manager;
use tokio::time::{sleep, Duration};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const HEADPHONE_NAME: &str = "WH-1000XM5"; // Enter the device name

    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = adapters.into_iter().nth(0).expect("No Bluetooth adapters found");

    println!("Starting BLE scan...");
    adapter.start_scan(ScanFilter::default()).await?;
    sleep(Duration::from_secs(2)).await;

    let peripherals = adapter.peripherals().await?;
    let mut found_peripheral = None;

    for p in peripherals {
        if let Some(props) = p.properties().await? {
            if let Some(name) = props.local_name {
                if name.contains(HEADPHONE_NAME) {
                    found_peripheral = Some(p);
                    break;
                }
            }
        }
    }

    let peripheral = found_peripheral.expect(&format!("{} headphones not found", HEADPHONE_NAME));

    let properties = peripheral.properties().await?.unwrap();
    println!("Found device: {:?}", properties.local_name);

    // Connect to the device
    peripheral.connect().await?;
    println!("Connected!");

    // Discover all services
    peripheral.discover_services().await?;
    println!("Discovered services:");
    for service in peripheral.services() {
        println!("Service UUID: {}", service.uuid);
        for characteristic in &service.characteristics {
            println!("  Characteristic UUID: {}", characteristic.uuid);
            println!("  Properties: {:?}", characteristic.properties);
        }
    }

    println!("Keeping connection alive for 10 seconds...");
    sleep(Duration::from_secs(10)).await;

    peripheral.disconnect().await?;
    println!("Disconnected.");

    Ok(())
}