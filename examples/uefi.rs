#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::proto::device_path::text::{AllowShortcuts, DevicePathToText, DisplayOnly};
use uefi::proto::loaded_image::LoadedImage;
use uefi::table::boot::SearchType;
use uefi::{Identify, Result};

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    let boot_services = system_table.boot_services();
    print_image_path(boot_services).unwrap();

    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}

fn print_image_path(boot_service: &BootServices) -> Result {
    let loaded_image =
        boot_service.open_protocol_exclusive::<LoadedImage>(boot_service.image_handle())?;
    let device_path_to_text_handle = *boot_service
        .locate_handle_buffer(SearchType::ByProtocol(&DevicePathToText::GUID))?
        .first()
        .expect("DevicePathToText is missing");

    let device_path_to_text =
        boot_service.open_protocol_exclusive::<DevicePathToText>(device_path_to_text_handle)?;

    let image_device_path = loaded_image.file_path().expect("File path is not set");
    let image_device_path_text = device_path_to_text
        .convert_device_path_to_text(
            boot_service,
            image_device_path,
            DisplayOnly(true),
            AllowShortcuts(false),
        )
        .expect("convert device image path to text failed");

    info!("Image path: {}", &*image_device_path_text);
    Ok(())
}
