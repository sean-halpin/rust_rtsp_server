#### Rust Rtsp Server 

A simple RTSP server implementation using rust to parse & respond to requests.  
Then GStreamer to serve the video for the negotiated session. 

#### Sample server output & session streamed using ffplay

![Alt text](rust_rtsp_server.png "Rust Rtsp Server")

#### Docker  

Build  
```
docker build -t rust_rtsp .
```

Run
```
docker run --rm -d --network host rust_rtsp                                         
``` 

Build in Docker
```
docker run -it --rm -d -v $(pwd):/src --network host rust_rtsp /bin/bash
```
Then
```
cd /src
cargo build
```