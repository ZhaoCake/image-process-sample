use opencv::prelude::*;

fn main() {
    // read in video in ../res/OnlyBasketball.mp4
    let mut video = opencv::videoio::VideoCapture::from_file("../res/OnlyBasketball.mp4", opencv::videoio::CAP_FFMPEG).unwrap();
    let mut frame = Mat::default(); // create a Mat to store the frame

    loop {
        video.read(&mut frame).unwrap();  // read the frame
        if frame.size().unwrap().width == 0 {  // if the frame is empty, break
            break;
        }

        opencv::highgui::imshow("Video", &frame).unwrap();  // display the frame
        if opencv::highgui::wait_key(10).unwrap() > 0 {  // wait for a key press
            break;
        }
    }

    opencv::highgui::destroy_all_windows().unwrap();  // destroy all windows

    println!("Done!");
}
