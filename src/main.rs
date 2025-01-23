#![no_std]
#![no_main]

use wut::prelude::*;
use wut::*;

#[wut_main(Udp)]
fn main() {
    // net_test();

    while process::running() {
        // println!("{:?}", time::SystemTime::now());
        // thread::sleep(time::Duration::from_secs(1));
    }

    // wups::wups();

    println!("ok");
    // println!("{}", process::running());
}

#[allow(dead_code)]
fn fs_test() {
    let p = "/vol/external01/wiiu/test.txt";
    // let p = "/vol/storage_mlc01/usr/test.txt"; // cemu
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        // .append(true)
        .create(true)
        .truncate(true)
        .open(p)
        .unwrap();
    println!("{:?}", file);

    println!("{}", file.metadata().unwrap().created().unwrap());

    println!("seek: {}", file.seek_position().unwrap());

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    println!("{:?}", String::from_utf8_lossy(&buffer));

    println!("seek: {}", file.seek_position().unwrap());

    println!("{:?}", file.write_all(b"abc"));

    println!("seek: {}", file.seek(fs::SeekFrom::Start(0)).unwrap());

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    println!("{:?}", String::from_utf8_lossy(&buffer));
}

#[allow(dead_code)]
fn net_test() {
    let listener = net::TcpListener::bind("0.0.0.0:7331").unwrap();

    println!("{:?}", listener);

    let (mut stream, _) = listener.accept().unwrap();

    println!("{:?}", stream);

    // let read = stream.read(&mut buffer).unwrap();
    // println!("{}", String::from_utf8_lossy(&buffer[..read]));

    let mut buffer = [0; 64];
    while let Ok(length) = stream.read(&mut buffer) {
        println!("{}", String::from_utf8_lossy(&buffer[..length]));
        let _ = stream.write(&buffer);
    }
}
