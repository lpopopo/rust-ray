fn main() {
    const IMAGE_HEIGHT: f64 = 256.0;
    const IMAGE_WIDTH: f64 = 256.0;
    const COLOR: f64 = 255.999;
    println!("P3\n256 256\n255");
    for height in 0..256 {
        for width in 0..256 {
            //生成rgb
            let r = f64::from(width) / (IMAGE_WIDTH - 1.0);
            let g = f64::from(height) / (IMAGE_HEIGHT - 1.0);
            let b: f64 = 0.25;

            let ir = (r * COLOR) as u32;
            let ig = (g * COLOR) as u32;
            let ib = (b * COLOR) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
