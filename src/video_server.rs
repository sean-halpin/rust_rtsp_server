extern crate gstreamer;
use gstreamer::prelude::*;
extern crate futures;
use futures::executor::LocalPool;
use futures::prelude::*;

async fn message_loop(bus: gstreamer::Bus) {
    let mut messages = gstreamer::BusStream::new(&bus);

    while let Some(msg) = messages.next().await {
        use gstreamer::MessageView;
        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        };
    }
}

pub fn serve_rtp(
    remote_host: String,
    client_rtp_port: String,
    client_rtcp_port: String,
    server_rtcp_port: String,
) {
    gstreamer::init().unwrap();
    let video_pattern = "ball";
    let _pipeline_string = format!("rtpbin name=rtpman autoremove=true 
               videotestsrc pattern={} ! videoconvert ! x264enc ! rtph264pay ! rtpman.send_rtp_sink_0 
               rtpman.send_rtp_src_0 ! udpsink name=rtpudpsink host={} port={} 
               rtpman.send_rtcp_src_0 ! udpsink name=rtcpudpsink  host={} port={} sync=false async=false 
               udpsrc name=rtcpudpsrc port={} ! rtpman.recv_rtcp_sink_0", 
               video_pattern,
               remote_host,
               client_rtp_port,
               remote_host, 
               client_rtcp_port,
               server_rtcp_port);

    let pipeline = gstreamer::parse_launch(&_pipeline_string).unwrap();
    let bus = pipeline.get_bus().unwrap();

    pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let mut pool = LocalPool::new();
    pool.run_until(message_loop(bus));

    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}
