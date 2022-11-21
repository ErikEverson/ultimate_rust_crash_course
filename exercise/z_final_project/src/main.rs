// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" if args.len() == 3 => {
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let blur_amount: f32 = args.remove(0).parse().expect("float was not provided");
            
            blur(infile, outfile, blur_amount);
        }

        // **OPTION**
        // Brighten -- see the brighten() function below
        "brighten" if args.len() == 3 => {
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let brighten_by: i32 = args.remove(0).parse().expect("int was not provided");

            brighten(infile, outfile, brighten_by);
        },

        // **OPTION**
        // Crop -- see the crop() function below
        "crop" if args.len() == 6 => {
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x: u32 = args.remove(0).parse().expect("int was not provided");
            let y: u32 = args.remove(0).parse().expect("int was not provided");
            let width: u32 = args.remove(0).parse().expect("int was not provided");
            let height: u32 = args.remove(0).parse().expect("int was not provided");

            crop(infile, outfile, x, y, width, height);
        },

        // **OPTION**
        // Rotate -- see the rotate() function below
        "rotate" if args.len() == 3 => {
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let degree: i32 = args.remove(0).parse().expect("int was not provided");

            let rotation: Rotation = Rotation::new(degree);

            rotate(infile, outfile, rotation);
        },

        // **OPTION**
        // Invert -- see the invert() function below
        "invert" if args.len() == 2 => {
            let infile = args.remove(0);
            let outfile = args.remove(0);

            invert(infile, outfile);
        },

        // **OPTION**
        // Grayscale -- see the grayscale() function below
        "grayscale" if args.len() == 2 => {
            let infile = args.remove(0);
            let outfile = args.remove(0);

            grayscale(infile, outfile);
        },

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" if args.len() == 1 => {
            let outfile = args.remove(0);
            fractal(outfile);
        },

        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!
        "generate" if args.len() == 4 => {
            let outfile = args.remove(0);
            let red: u8 = args.remove(0).parse().expect("float was not provided");
            let green: u8 = args.remove(0).parse().expect("float was not provided");
            let blue: u8 = args.remove(0).parse().expect("float was not provided");
            // error checkinfg for real code inputs

            generate(outfile, red, green, blue);
        },

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE BLUR_AMOUNT");
    println!("brighten INFILE OUTFILE BRIGHTEN_BY");
    println!("crop INFILE OUTFILE X_COORD Y_COORD WIDTH HEIGHT");
    println!("rotate INFILE OUTFILE DEGREE");
    println!("invert INFILE OUTFILE");
    println!("grayscale INFILE OUTFILE");
    println!("fractal OUTFILE");
    println!("generate OUTFILE RGB_R RGB_G RGB_B");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, blur_amount: f32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(blur_amount);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, brighten_by: i32) {
    let image = image::open(infile).expect("Failed to open INFILE.");
    let brightened_image = image.brighten(brighten_by);
    brightened_image.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let mut image = image::open(infile).expect("Failed to open INFILE.");
    let cropped_image = image.crop(x, y, width, height);
    cropped_image.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String, rotation: Rotation) {
    let image = image::open(infile).expect("Failed to open INFILE.");
    let rotated_image = match rotation {
        Rotation::Ninty => image.rotate90(),
        Rotation::OneEighty => image.rotate180(),
        Rotation::TwoSeventy => image.rotate270(),
    };
    rotated_image.save(outfile).expect("Failed writing OUTFILE.");
}

enum Rotation {
    Ninty,
    OneEighty,
    TwoSeventy,
}

impl Rotation {
    fn new(degree: i32) -> Self {
        match degree {
            x if x <= 90 => Self::Ninty,
            x if x < 270 => Self::OneEighty,
            _ => Self::TwoSeventy,
        }
    }
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    image::open(infile).expect("Failed to open INFILE.")
        .grayscale()
        .save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String, r: u8, g: u8, b: u8) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        // set to red rgb
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
