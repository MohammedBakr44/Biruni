use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("ffmpeg")
        // Overwrite file if it already exists
        .arg("-y")
        // Interpret the information from stdin as "raw video" ...
        .arg("-f").arg("rawvideo")
        // ... where every four bytes are [r, g, b, a] format
        .arg("-pix_fmt").arg("rgba")
        // The size of the video is 3840x2160
        .arg("-s").arg("3840x2160")
        // 60 frames per second
        .arg("-r").arg("60")
        // Don't expect any audio in the stream
        .arg("-an")
        // Get the data from stdin
        .arg("-i").arg("-")
        // encode to h264
        .arg("-c:v").arg("libx264")
        // variable video bitrate
        .arg("-crf").arg("0")
        // Output file
        .arg("test.mp4")
        // stdin, stderr, and stdout are piped
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        // Run the child command
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();

    let mut buffer = vec![0; 3840*2160*4];

    let mut frames = 0;
    for r in 0..4 {
        for gb in 0..=255 {
            println!("{}", gb);
            generate_frame(&mut buffer, 3840, 2160, r, gb, gb, 0xff);
            stdin.write_all(&buffer);
            frames += 1;
        }
    }

    let output = child.wait_with_output().unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
    println!("Generated {} frames.", frames)
}

fn generate_frame(buffer: &mut Vec<u8>, width: usize, height: usize, r: u8, g: u8, b: u8, a: u8) {
    for row in 0..height {
        for col in 0..width {
            buffer[(row*width+col)*4 + 0] = r; // red
            buffer[(row*width+col)*4 + 1] = g; // green
            buffer[(row*width+col)*4 + 2] = b; // blue
            buffer[(row*width+col)*4 + 3] = 0xff; // alpha
        }
    }
}

// Credits goes for u/kabocha_ from r/Rust