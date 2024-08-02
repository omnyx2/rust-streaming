// use std::thread;
 

// use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use crate::headers::opencv_frame::Frame;

use crossbeam_channel::Receiver;
use anyhow::Result; // Automatically handle the error types
use opencv::{
    core::{self},
    imgcodecs,
     videoio,
    highgui 
}; // Note, the namespace of OpenCV is changed (to better or worse). It is no longer one enormous.

pub fn frame_saver (rx: crossbeam_channel::Receiver<Frame>, exit_flag:  Arc<Mutex<bool>>) -> Result<()>  {
    let format = ".png";
    let baseName = "hehe-";
    let basePath = "./img/";
    let mut i = 0;

    let  mut parms= core::Vector::default();
    parms.push(imgcodecs::IMWRITE_JPEG_QUALITY);
    parms.push(95); 

    for received in & rx {
      i += 1;
      let image_name = format!("{}{}{}-{}{}", basePath, "frame-", i, &received.meta_data.timestamp, format);
      // println!("Rec eived frame: {:?}", image_name);
      imgcodecs::imwrite(&image_name, &received.data, &parms)?;

      if *exit_flag.lock().unwrap() {
        break;  
      }
    }
    Ok(())
  }