
mod renderer;

use std::fs::{File, self};
use std::io::Read;
use std::{env, fmt};
use qrcode_generator::QrCodeEcc;
use bevy::prelude::*;
use renderer::encryption::Encrypted;
use renderer::renderer::Renderer;
use crate::renderer::QrCode;

// OpenCV
use opencv::{
    Result,
    prelude::*,
    objdetect,
    imgproc,
    highgui,
    types,
    videoio,
    core,
};

// https://www.asciihex.com/character/control/23/0x17/etb-end-of-transmission-block
enum Protocol {
    Handshake,
    Message,
    EndOfTransmission
}

#[derive(Debug)]
enum WindowName {
    Qtx,
    QtxDebug,
    Qrx,
    QrxDebug
}

impl fmt::Display for WindowName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // test_quirc_multi();

    test_qrcode_generation_encrypted(args);

/* 
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .run();
*/

    Ok(())

    // https://learnopencv.com/wechat-qr-code-scanner-in-opencv/
    // wechat_qrcode


    // OPENCV_INCLUDE_PATHS=+/home/francois/dev/rust/opencv-projects/build/include
    // OPENCV_LINK_PATHS=+/home/francois/.local/lib
    //
    // OpenCV_DIR=/home/francois/dev/rust/opencv-projects/opencv-4.x/cmake
    // LD_LIBRARY_PATH=/home/francois/.local/lib
    // cargo clean
    // cargo build
    // cargo run

    // test_quirc_multi();
    // test_qrcode_generation(args);
    // qtx()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("qtx.png"),
        ..default()
    });
}

fn qtx() -> Result<()> {
    let mut qr_detector = objdetect::QRCodeDetector::default()?;
    let mut res = types::VectorOfPoint::new();
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut img = Mat::default();
    let mut recqr = Mat::default();

    loop {
        camera.read(&mut img)?;
        let ret = qr_detector.detect_and_decode(&img, &mut res, &mut recqr)?;
        let s = String::from_utf8_lossy(&ret);
        highgui::named_window(WindowName::Qtx.to_string().as_str(), highgui::WINDOW_NORMAL)?;
        if recqr.size()?.width > 0{
            highgui::imshow(WindowName::Qtx.to_string().as_str(), &recqr)?;
        }
        if res.len()>0 {
            imgproc::polylines(
                &mut img, 
                &res, 
                true, 
                core::Scalar::new(0f64,255f64,0f64,0f64), 
                4,
                1,
                0)?;
                println!("{:?}", res);
                println!("{:?}", s);
                }

        highgui::imshow(WindowName::QtxDebug.to_string().as_str(), &img)?;
        let key = highgui::wait_key(1)?;
        if key == 'q' as i32 {
            break;
        }
    }

    Ok(())
}

fn test_quirc_multi() -> Result<()> {
    // 
    // sudo apt-get install clang make cmake-qt-gui
    // git clone https://github.com/opencv/opencv.git --depth 1
    // git clone https://github.com/opencv/opencv_contrib.git --depth 1
    // 
    //
    // https://github.com/clchan000/Multiple-QR-codes-Detection-Segmentation-and-Decoding
    //
    // cd opencv-projects/build
    // cmake -DOPENCV_EXTRA_MODULES_PATH=../opencv_contrib-4.x/modules ../opencv-4.x -DBUILD_QUIRC=ON -DQUIRC=ON
    // cmake --build .


    let mut qr_detector = objdetect::QRCodeDetector::default()?;
    let mut res = types::VectorOfPoint::new();
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut img = Mat::default();
    let mut recqr = Mat::default();

    loop {
        camera.read(&mut img)?;
        let ret = qr_detector.detect_multi(&img, &mut res)?;
        highgui::named_window("QR Code", highgui::WINDOW_NORMAL)?;
        if recqr.size()?.width > 0{
            highgui::imshow("QR Code", &recqr)?;
        }
        if res.len()>0 {
            
            res
            .to_vec()
            .chunks(4)
            .into_iter()
            .for_each(|chunk| { 
                    imgproc::polylines(
                        &mut img, 
                        &types::VectorOfPoint::from_slice(chunk), 
                        true, 
                        core::Scalar::new(0f64,255f64,0f64,0f64), 
                        4,
                        1,
                        0
                    ).unwrap();
                    println!("chunk.len = {}", chunk.len());
                    println!("{:?}", chunk);
                }
            )
        }
           

        highgui::imshow("Frame", &img)?;
        let key = highgui::wait_key(1)?;
        if key == 'q' as i32 {
            break;
        }
    }

    Ok(())
}

fn test_quirc() -> Result<()> {
    // 
    // sudo apt-get install clang make cmake-qt-gui
    // git clone https://github.com/opencv/opencv.git --depth 1
    // git clone https://github.com/opencv/opencv_contrib.git --depth 1
    // 
    //
    // https://github.com/clchan000/Multiple-QR-codes-Detection-Segmentation-and-Decoding
    //
    // cd opencv-projects/build
    // cmake -DOPENCV_EXTRA_MODULES_PATH=../opencv_contrib-4.x/modules ../opencv-4.x -DBUILD_QUIRC=ON -DQUIRC=ON
    // cmake --build .


    let mut qr_detector = objdetect::QRCodeDetector::default()?;
    let mut res = types::VectorOfPoint::new();
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut img = Mat::default();
    let mut recqr = Mat::default();

    loop {
        camera.read(&mut img)?;
        let ret = qr_detector.detect_and_decode(&img, &mut res, &mut recqr)?;
        let s = String::from_utf8_lossy(&ret);
        highgui::named_window("QR Code", highgui::WINDOW_NORMAL)?;
        if recqr.size()?.width > 0{
            highgui::imshow("QR Code", &recqr)?;
        }
        if res.len()>0 {
            imgproc::polylines(
                &mut img, 
                &res, 
                true, 
                core::Scalar::new(0f64,255f64,0f64,0f64), 
                1,
                1,
                0)?;
                println!("{:?}", res);
                println!("{:?}", s);
                }

        highgui::imshow("Frame", &img)?;
        let key = highgui::wait_key(1)?;
        if key == 'q' as i32 {
            break;
        }
    }

    Ok(())
}

fn test_qrcode_generation(args: Vec<String>) {
    let output_filename = "./qtx.png";

    let filename = &args[1];
    let mut handle = File::open(&filename).expect("Could not open file");
    let metadata = fs::metadata(&filename).expect("Could not get file metadata");
    let mut buffer = vec![0; metadata.len() as usize];

    handle.read(&mut buffer).expect("Buffer overflow");

    qrcode_generator::to_png_to_file(buffer, QrCodeEcc::Medium, 1024, output_filename).unwrap();

    //
    // Decode
    //
    // open the image from disk
    let img = image::open(output_filename).expect("failed to open image");

    // convert to gray scale
    let img_gray = img.into_luma8();

    // create a decoder
    let mut decoder = quircs::Quirc::default();

    // identify all qr codes
    let codes = decoder.identify(img_gray.width() as usize, img_gray.height() as usize, &img_gray);

    for code in codes {
        let code = code.expect("failed to extract qr code");
        let decoded = code.decode().expect("failed to decode qr code");
        println!("qrcode: {}", std::str::from_utf8(&decoded.payload).unwrap());
    }
}

fn test_qrcode_generation_encrypted(args: Vec<String>) {
    let output_filename = "./qtx.png";

    let filename = &args[1];
    let mut handle = File::open(&filename).expect("Could not open file");
    let metadata = fs::metadata(&filename).expect("Could not get file metadata");
    let mut buffer = vec![0; metadata.len() as usize];

    handle.read(&mut buffer).expect("Buffer overflow");

    let qrcode = QrCode { data: buffer.clone() };
    let encryption = Encrypted { encryption_key : "magickey".to_string() };

    qrcode.write_to_png_file(&encryption, QrCodeEcc::Low, output_filename);

    //
    // Decode
    //
    // open the image from disk
    let img = image::open(output_filename).expect("failed to open image");

    // convert to gray scale
    let img_gray = img.into_luma8();

    // create a decoder
    let mut decoder = quircs::Quirc::default();

    // identify all qr codes
    let codes = decoder.identify(img_gray.width() as usize, img_gray.height() as usize, &img_gray);

    for code in codes {
        let code = code.expect("failed to extract qr code"); 
        let qrcode = QrCode::read_from_quirc_code(&encryption, &code);
        println!("qrcode: {}", std::str::from_utf8(&qrcode.data).unwrap());
    }
}
