mod keyboard;
mod event_handler;
mod commands;
mod video_opencv;
// mod video_gstream;
mod filesaver;
mod kafka_apis;
mod headers;
mod checkconnections;
 
mod Error;
// mod headers::{opencv_frame};

use std::sync::{Arc, Mutex};
use std::thread;
// use crate::headers::opencv_frame;
// use crate::headers::opencv_frame::Frame;
// use crate::headers::opencv_frame::VideoMetaData;
use crate::headers::opencv_frame;

use crossbeam_channel::bounded;
// use std::env;
// use std::time::Duration;
use opencv::{
    highgui,
    prelude::*,
    Result,
};

// struct Frame {
//   data: Mat,
//   timestamp: String,
// }

// #[derive(Debug)]
fn main() -> Result<()> {
    // tracing_subscriber::fmt::init();
    // let args: Vec<String> = env::args().collect();
    // let mode = &args[1];
    let kafka_connection = kafka_apis::KafkaConfig {
      broker: String::from("10.80.0.3:9094"),
      topic: String::from("video-topic")
    };
    checkconnections::all(10,kafka_connection,String::from("./img"));    
    let kafka_connection = kafka_apis::KafkaConfig {
      broker: String::from("10.80.0.3:9094"),
      topic: String::from("video-topic")
    };

    
    let (s, r) = bounded(1);
    let (s2, r2) = (s.clone(), r.clone());

    let (ims, imr): (crossbeam_channel::Sender<opencv_frame::Frame>, crossbeam_channel::Receiver<opencv_frame::Frame>) = bounded(10); 
    let (ims1, imr1) = (ims.clone(), imr.clone());
    let (ims2, imr2) = (ims.clone(), imr.clone());

    let (imns, imnr): (crossbeam_channel::Sender<String>,crossbeam_channel::Receiver<String>) = bounded(1);
    let (imns1, imnr1) = (imns.clone(), imnr.clone());
    

    let mut frame = Mat::default();
    let frame_lock = Arc::new(Mutex::new(false));

    let exit_flag = Arc::new(Mutex::new(false));
    let stop_flag = Arc::new(Mutex::new(false));

    let exit_video_flag = Arc::new(Mutex::new(false));

    // 키보드 입력 스레드 시작
    let exit_flag_clone = Arc::clone(&exit_flag);
    let stop_flag_clone = Arc::clone(&stop_flag);
    thread::spawn(move || {
        keyboard::handle_keyboard_input(s2, exit_flag_clone, stop_flag_clone);
    });

    // // 이벤트 처리 스레드 시작
    let exit_flag_clone_2 = Arc::clone(&exit_flag);
    let stop_flag_clone_2 = Arc::clone(&stop_flag);
    thread::spawn(move || { 
      event_handler::handle_events(r2, exit_flag_clone_2, stop_flag_clone_2);
    });
    
    // 비디오 처리 쓰레드 시작
    let exit_flag_clone_3 = Arc::clone(&exit_flag);
    let stop_flag_clone_3 = Arc::clone(&stop_flag);
    thread::spawn(move || {
      video_opencv::handle_videostream(
        0,
        ims1);
    });

    // 저쟝 쓰레드 시작
    // let exit_flag_clone_3 = Arc::clone(&exit_flag);
    thread::spawn(move|| {
       
    for received in &imr2 {
      if let Err(e) = kafka_apis::produce_message( &received.data.data_bytes().unwrap(), &kafka_connection.topic, vec![kafka_connection.broker.to_owned()]) {
        println!("Failed producing messages: {}", e);
      }
    } 
      // filesaver::frame_saver(imr2, exit_flag_clone_3);
    });

    highgui::named_window("video", highgui::WINDOW_NORMAL)?;
    // let (ims2, imr2) = (ims.clone(), imr.clone());
    // let exit_flag_clone_4 = Arc::clone(&exit_flag);
 
    for received in &imr1 {
      // let image_name = format!("{}{}{}-{}{}", basePath, "frame-", i, &received.meta_data.timestamp, format);
      println!("Rec eived frame: {:?}", &received.meta_data.timestamp);

      // nvidia-deep-stream, data-pipline to save faster
      // data pipeline to connect,
      // High-End-GPU
      // Data PipeLine
      highgui::imshow("Webcam", &received.data)?;
      // if let Err(e) = kafka_apis::produce_message( &received.meta_data.timestamp.as_bytes(), topic, vec![broker.to_owned()]) {
      // if let Err(e) = kafka_apis::produce_message( &received.data.data_bytes().unwrap(), topic, vec![broker.to_owned()]) {
      //   println!("Failed producing messages: {}", e);
      // }

      if *exit_flag.lock().unwrap() {
        break;  
      }
      if highgui::wait_key(1)? == 113 { // 'q' key to quit
          break;
      }
      }
    
    Ok(())
}
