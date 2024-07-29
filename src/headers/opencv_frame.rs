use std::sync::{Arc, Mutex};
use crossbeam_channel::Receiver;
use chrono::prelude::*;
use anyhow::Result; // Automatically handle the error types
use opencv::{
    core::{self},
    imgcodecs,
    prelude::*,
    videoio,
    highgui 
};  

pub struct VideoMetaData {
    pub timestamp: String
  }
  // Struct to hold frame data
pub struct Frame {
    pub data: Mat,
    pub meta_data: VideoMetaData,
}