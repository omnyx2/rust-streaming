 
use opencv::{
    imgcodecs,
    core,
    highgui,
    videoio,
    prelude::*,
    Result,
};
use std::fs;
use std::path::Path;
use crate::kafka_apis;

pub fn all(max_cameras: i32, kafka_config: kafka_apis::KafkaConfig, img_dir_path: String) 
-> Result<()>{
    let mut err1 = false;
    kafka_check(kafka_config);
    cameras_check(max_cameras);
    dir_check(&img_dir_path);
    if !err1 {
        panic!("camera is not opend at all");
    }
    Ok(())
}

pub fn kafka_check(kafka_config: kafka_apis::KafkaConfig) -> Result<bool>{
    let mut kafka_is_vaild = false;
    let test_data2 = "TEST_CONNECTION_VAILD";
    if let Err(e) = kafka_apis::test_produce_message( &test_data2, &kafka_config.topic, vec![kafka_config.broker.to_owned()]) {
        println!("Failed producing messages: {}", e);
        kafka_is_vaild = true;
    }

    if kafka_is_vaild {
        Ok(false)
    } else {
        Ok(true)
    }
}

// pub fn all(max_cameras: i32) -> Result<()>{

// }

pub fn cameras_check(max_cameras: i32) -> Result<bool> {
    let mut num_of_camera = 0;

    println!("Checking available webcams...");
    for i in 0..max_cameras {
    
        // Try to open the video capture device
        let mut cam = videoio::VideoCapture::new(i, videoio::CAP_ANY)?;

        if cam.is_opened()? {
            num_of_camera += 1;
            drop(cam);
        }
    }

    if num_of_camera != 0 {
        Ok(true)
    } else {
        Ok(false)

    }
}

pub fn dir_check(dir_path: &str) -> Result<bool> {
    if !Path::new(dir_path).exists() {
        // println!("Directory '{}' does not exist. Creating it...", dir_path);
        // fs::create_dir_all(dir_path).with_context(|| format!("Failed to create directory '{}'", dir_path))?;
        Ok(false)
    } else {
        println!("Directory '{}' exists.", dir_path);
        Ok(true)

    }
}

// pub fn file_check(file_path: &str) -> Result<bool> {
//     let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => true,
//         Err(ref error) if error.kind() == ErrorKind::NotFound => {
//             match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => {
//                     panic!(
//                         "Tried to create file but there was a problem: {:?}",
//                         e
//                     )
//                 },
//             }
//         },
//         Err(error) => {
//             panic!(
//                 "There was a problem opening the file: {:?}",
//                 error
//             )
//         },
//     };
// }