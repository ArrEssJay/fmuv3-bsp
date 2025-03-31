use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;
use base64::Engine;

use std::fs::File;
use std::io::Write;

const BOARD_ID: u32 = 9; // Cube Black (FMUv3)
const FLASH_SIZE: u32 = 2_080_768;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <firmware_path> <package_name>", args[0]);
        std::process::exit(1);
    }

    print!("apj_gen: input_firmware: {}",args[1] );
    let firmware_path = PathBuf::from(&args[1]);
    let package_name = args[2].clone();
    
    
    if !firmware_path.exists() {
        panic!("{} not found. Skipping APJ generation.", &args[1]);
    }
    
    let mut apj_path = firmware_path.clone();
    if !apj_path.set_extension("apj") {
        panic!("Could not set extension");
    }

    let mut bin_path = firmware_path.clone();
    if !bin_path.set_extension("bin") {
        panic!("Could not set extension");
    }

    // Objcopy ELF to flat binary
    let status: std::process::ExitStatus = Command::new("rust-objcopy")
    .args(["-O", "binary", firmware_path.to_str().unwrap(),
    bin_path.to_str().unwrap()])
    .status()
    .expect("Failed to run objcopy");
    assert!(status.success());

   
    // Read flat binary firmware
    let firmware = fs::read(bin_path).expect("Failed to read firmware");
    let firmware_len = firmware.len() as u32;

    // Construct APJ header
    let description = format!("Firmware for {}", package_name);
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
    encoder.write_all(&firmware).expect("Failed to compress firmware");
    let compressed_image = encoder.finish().expect("Compression finish failed");
    let encoded_image = base64::engine::general_purpose::STANDARD.encode(&compressed_image);

    let apj_string = serde_json::json!({
        "board_id": BOARD_ID,
        "magic": "APJFWv1",
        "description": description,
        "image": encoded_image,
        "summary": package_name,
        "version": "0.1",
        "image_size": firmware_len,
        "board_revision": 0,
        "signed_firmware": false,
        "extf_image_size": 0,
        "flash_total": FLASH_SIZE,
        "image_maxsize": FLASH_SIZE,
        "flash_free": FLASH_SIZE-firmware_len,
        "extflash_total": 0,
        "extflash_free": 0,
        "git_identity": "ffffffff",
        "board_revision": 0,
        "USBID": "0x2dae/0x1011"
    }).to_string();

    // Write new APJ firmware file
    let mut output_file = File::create(&apj_path).expect("Failed to create output file");
    output_file.write_all(apj_string.as_bytes()).expect("Failed to write firmware");

}