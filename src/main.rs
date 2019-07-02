extern crate rand;
extern crate image;

use rand::prelude::*;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

// A crystal pattern is build by letting particles perform random walks from
// "far away" (the edge of the picture) until they meet a particle. Then they stick and
// and a new particle is released.
// Particles is represented as values above 10 and empty space is value 0
// Near particles sticking to each other the space is rendered "sticky" by giving it a value
// between 1 and 5
#[allow(dead_code)]
fn main() {
    // initialise number of particles, picture size, array to hold data and set a particle in the center
    let number_of_particles: usize = 1000; // The number of particles (N)
    let pic_size: usize = 200;
    let mut release_distance: usize = 20; // Needs to be less than pic_size
    let mut pixels = vec![0; (pic_size + 1)*(pic_size + 1)];
    let mut wander_pixels = vec![0; (pic_size + 1)*(pic_size + 1)]; // Follows 1 particle. N has to be 1
    let mut start_pixels = vec![0; (pic_size + 1)*(pic_size + 1)];  // Track where the particles start
    pixels[pic_size*pic_size/2 + pic_size/2 as usize] = 10; // A particle is placed in the center
    // println!("{:?}", pixels);

    // release N particles, let them wander until they connect and then update the release distance
    for number in 0..number_of_particles {
        if number % 50 == 0 {
            println!("{:?}", number);
        }
        let coor = release_particle(&mut start_pixels, &pic_size, &release_distance);
        let final_coor = let_particle_wander(&mut pixels, &mut wander_pixels, &pic_size, coor);
        release_distance = update_release_distance(&pic_size, release_distance, final_coor);
    }
    // Write results to files:
    write_image("Crystal.png", &mut pixels, pic_size).expect("Error writing to file");
    write_image("Wandering_Crystal.png", &mut wander_pixels, pic_size).expect("Error writing to file");
    write_image("Start_Crystal.png", &mut start_pixels, pic_size).expect("Error writing to file");
}

fn update_release_distance(pic_size: &usize, release_distance: usize, coor: (usize, usize)) -> usize {
    // In order to have a greater chance to connect early the particles are released near the blob.
    // The distance is getting bigger if the euclidian distance of the last particle to the center
    // is greater than the previous release distance
    let x_center = pic_size/2;
    let y_center = x_center.clone();
    if sqr_root((x_center*x_center + coor.0*coor.0 - 2*coor.0*x_center) +
                (y_center*y_center + coor.1*coor.1 - 2*coor.1*y_center)) > release_distance
                && release_distance < pic_size/2 - 1 {
        let new_release_distance = release_distance + 1;
        return new_release_distance
    }
    release_distance
}

fn release_particle(start_pixels: &mut Vec<u8>,pic_size: &usize, release_distance: &usize) -> (usize, usize) {
    // A particle is released randomly from a circle with radius release_distance and centered
    // on the original particle. The equation for a cirle is (x - a)*2 + (y - b)*2 = r*2
    let x_center = pic_size/2;
    let y_center = x_center.clone();
    let x = thread_rng().gen_range(x_center - release_distance, x_center + release_distance);
    let mut y = 0;
    if thread_rng().gen_range(0, 2) == 0 {
        y = y_center - (sqr_root(release_distance*release_distance - (x_center*x_center + x*x - 2*x*x_center)));
    } else {
        y = y_center + (sqr_root(release_distance*release_distance - (x_center*x_center + x*x - 2*x*x_center)));
    }
    start_pixels[x + y*pic_size] = 250;  // Where the particles is released is tracked here
    (x, y)
}

fn sqr_root(number: usize) -> usize {  // Returns an integer approximating the square root of 'number'
    let mut n = 1;
    while n*n < number {
        n += 1;
    }
    n
}

fn let_particle_wander(pixels: &mut Vec<u8>, wander_pixels: &mut Vec<u8>, pic_size: &usize, mut coor: (usize, usize)) -> (usize, usize) {
    // The particle is randomly moved left/right/down/up until it is near a fixed particle
    // It is not allowed to move out of the picture. It'll just wait for next step if overstepping.
    while pixels[coor.0 + coor.1*pic_size] == 0 {
        let rnd = thread_rng().gen_range(0, 4); // rnd become one of the numbers 0, 1, 2, 3
        if coor.0 > 0 && rnd == 0 {
            coor.0 -= 1;
        }
        if coor.0 < *pic_size && rnd == 1 {
            coor.0 += 1;
        }
        if coor.1 > 0 && rnd == 2 {
            coor.1 -= 1;
        }
        if coor.1 < *pic_size && rnd == 3 {
            coor.1 += 1;
        }
        wander_pixels[coor.0 + coor.1*pic_size] = 250; // The track of the particles is stored here. Only meaningful for N = 1
    }
    // println!("Ny x,y {},{}", coor.0, coor.1 );
    if coor.0 > 0 && coor.0 < *pic_size && coor.1 > 0 && coor.1 < *pic_size {
        pixels[coor.0 + coor.1*pic_size] = 250; // Here the particle is fixed in memory
        pixels[coor.0 + 1 + coor.1*pic_size] += 1; // Every cell around the particle is rendered
        pixels[coor.0 - 1 + coor.1*pic_size] += 1; // "sticky" by setting it's value higher than 0.
        pixels[coor.0 + (coor.1 + 1)*pic_size] += 1; // A new particle trying to go here will stick
        pixels[coor.0 + (coor.1 -1)*pic_size] += 1;
    }
    coor
}


fn write_image(filename: &str, pixels: &mut Vec<u8>, side: usize) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, side as u32, side as u32, ColorType::Gray(8))?;
    Ok(())
}
