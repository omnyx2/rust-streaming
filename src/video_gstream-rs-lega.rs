use gstreamer::prelude::*;
use std::env;

fn gstreamer_recode() {
    // Initialize GStreamer
    gstreamer::init().expect("Failed to initialize GStreamer");

    // Create a new pipeline
    let pipeline = gstreamer::Pipeline::new(None);

    // Create elements
    let source = gstreamer::ElementFactory::make("filesrc", None)
        .expect("Failed to create 'filesrc' element");
    let decoder = gstreamer::ElementFactory::make("decodebin", None)
        .expect("Failed to create 'decodebin' element");
    let sink = gstreamer::ElementFactory::make("autovideosink", None)
        .expect("Failed to create 'autovideosink' element");

    // Set properties
    source.set_property("location", "video.mp4").expect("Failed to set 'location' property");

    // Add elements to the pipeline
    pipeline.add_many(&[&source, &decoder, &sink]).expect("Failed to add elements to pipeline");
    gstreamer::Element::link_many(&[&source, &decoder]).expect("Failed to link elements");

    // Link decoder to sink using a pad-added signal
    decoder.connect_pad_added(move |_, src_pad| {
        let sink_pad = sink.get_static_pad("sink").expect("Failed to get sink pad");
        src_pad.link(&sink_pad).expect("Failed to link pads");
    });

    // Start playing
    pipeline.set_state(gstreamer::State::Playing).expect("Failed to set pipeline to Playing state");

    // Wait until the pipeline finishes
    let bus = pipeline.get_bus().expect("Failed to get bus");
    for msg in bus.timed_pop_filtered(gstreamer::ClockTime::from_seconds(10), &[gstreamer::MessageType::Eos, gstreamer::MessageType::Error]) {
        match msg.view() {
            gstreamer::MessageView::Error(err) => {
                eprintln!("Error: {}", err.get_error());
                eprintln!("Debug info: {:?}", err.get_debug());
            }
            gstreamer::MessageView::Eos(_) => {
                println!("End of stream reached.");
            }
            _ => (),
        }
    }

    // Stop the pipeline
    pipeline.set_state(gstreamer::State::Null).expect("Failed to set pipeline to Null state");
}
