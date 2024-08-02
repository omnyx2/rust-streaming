
// use std::thread;
// use std::sync::mpsc;  
 
use std::sync::{Arc,  Mutex};
use crate::headers::opencv_frame::Frame;
use crate::headers::opencv_frame::VideoMetaData;

use chrono::prelude::*;
use opencv::{
    prelude::*,
    videoio,
 };  
//fn checkMatIsEmpty(src) -> Result<()> {
//   if src.empty(){
//        println!("{}","image load failed");
//        std::process::exit(0);
//    }
//}
 

fn get_current_timestamp() -> String {
  let utc_time: DateTime<Utc> = Utc::now();
  let localTime: DateTime<Local> = utc_time.with_timezone(&Local);
  format!("{}",localTime.format("%Y-%m-%d-%H:%M:%S"))
}



// Function to capture frames
pub fn handle_videostream(camera_id: i32, sender: crossbeam_channel::Sender<Frame>) {
  let mut cam = videoio::VideoCapture::new(camera_id, videoio::CAP_ANY).unwrap();
   cam.set(videoio::CAP_PROP_FPS, 60.0).unwrap();
  loop {
      let mut frame = Mat::default();
      let fps = cam.get(videoio::CAP_PROP_FPS).unwrap();
      if cam.read(&mut frame).unwrap() {
          println!("{}",fps);
          let timestamp = get_current_timestamp();
          let data = &frame;
          // 추후에 이 로직을 비동기로 넣고 처리해야함.
          // 그러면 뭐 금방되것지
          use std::time::Instant;
          let now = Instant::now();

          sender.send(Frame { data: frame.clone(), meta_data: VideoMetaData { timestamp: timestamp.clone() } }).unwrap();
          sender.send(Frame { data: frame.clone(), meta_data: VideoMetaData { timestamp: timestamp.clone() } }).unwrap();

          let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
      }
  }
}



// pub fn handle_videostream(
//       tx: crossbeam_channel::Sender<Mat>, 
//       rx: crossbeam_channel::Receiver<String>,
//       tx_image_name: crossbeam_channel::Receiver<string>,
//     //   frame: Mat,
//       exit_flag: Arc<Mutex<bool>>, 
//       stop_flag: Arc<Mutex<bool>>, 
//       frame_lock: Arc<Mutex<bool>>) -> Result<()> { // Note, this is anyhow::Result
//     // Open a GUI window
//     let mut frame = Mat::default();

//     let utc_time: DateTime<Utc> = Utc::now();
//     let local_time: DateTime<Local> = utc_time.with_timezone(&Local);
//     println!("UTC time: {}", utc_time);
//     println!("Local time: {}", local_time);

//     let format = ".png";
//     let baseName = "hehe-";
//     let basePath = "./img/";

//     // and display in the window
//     let mut i = 0;

//     let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
  
//      // This array will store the web-cam data
//     // 1. opencv설치 
//     // 2. rust cargo설치
//     // 3. kafka로 frame전송
//     // // Read the camera


//     loop {
//         cam.read(&mut frame).expect("Failed to capture frame");
//         tx.send(frame.clone()).unwrap();
//         let utc_time: DateTime<Utc> = Utc::now();
//         let localTime: DateTime<Local> = utc_time.with_timezone(&Local);
//         let image_name = format!("{}{}{}-{}{}", basePath, "frame-", i, localTime.format("%Y%m%d%H%M%S"), format);
//         tx_image_name.send(image_name);
//         if *exit_flag.lock().unwrap() {
//             println!("will break");
//            break; 
//         }
        
//     }    
//     Ok(())
// }


// use std::thread;
// use std::sync::mpsc;
// use std::time::Duration;
// use chrono::prelude::*;
// use anyhow::Result;
// use opencv::{
//     core,
//     imgcodecs,
//     prelude::*,
//     videoio,
//     highgui,
// };

// fn main() -> Result<()> {
//     let (tx, rx) = mpsc::channel();

//     // Webcam capture thread
//     let tx_clone = tx.clone();
//     thread::spawn(move || -> Result<()> {
//         highgui::named_window("Webcam", highgui::WINDOW_NORMAL)?;

//         let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
//         if !videoio::VideoCapture::is_opened(&cam)? {
//             eprintln!("Unable to open camera");
//             return Ok(());
//         }

//         let mut frame = Mat::default();
//         loop {
//             cam.read(&mut frame)?;
//             if frame.empty() {
//                 continue;
//             }

//             highgui::imshow("Webcam", &frame)?;
//             tx_clone.send(frame.clone()).unwrap(); // Send frame to main thread

//             if highgui::wait_key(1)? == 113 { // 'q' key to quit
//                 break;
//             }
//         }
//         Ok(())
//     });

//     // Frame saving thread
//     thread::spawn(move || -> Result<()> {
//         let base_path = "./img/";
//         let format = ".png";
//         let mut i = 0;

//         for received_frame in rx {
//             let utc_time: DateTime<Utc> = Utc::now();
//             let local_time: DateTime<Local> = utc_time.with_timezone(&Local);
//             let image_name = format!("{}{}{}-{}{}", base_path, "frame-", i, local_time.format("%Y%m%d%H%M%S"), format);
//             imgcodecs::imwrite(&image_name, &received_frame, &core::Vector::new())?;
//             i += 1;
//         }
//         Ok(())
//     });

//     // Wait for threads to finish
//     loop {
//         if highgui::wait_key(1)? == 113 { // 'q' key to quit
//             break;
//         }
//         thread::sleep(Duration::from_millis(10));
//     }

//     Ok(())
// }
