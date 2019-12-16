extern crate image;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

struct Layer<'a> {
    length: usize,
    width: usize,
    pixels: &'a [usize],
}

impl<'a> Layer<'a> {
    fn new(length: usize, width: usize, pixels: &'a [usize]) -> Self {
        Layer{
            length,
            width,
            pixels,
        }
    }

    fn count(&self, value: usize) -> usize {
        self.pixels.iter().filter(|pixel| **pixel == value ).count()
    }
}

fn layers_from_pixels<'a>(length: usize, width: usize, pixels: &'a [usize]) -> Vec<Layer<'a>> {
    let layer_size = length * width;
    (0..pixels.len())
        .step_by(layer_size)
        .map(|i| Layer{
            length: length,
            width: width,
            pixels: &pixels[i..i+layer_size]
        })
        .collect()
}

struct Image {
    length: usize,
    width: usize,
    pixels: Vec<usize>,
}

impl Image {
    fn new<'a>(layers: Vec<Layer<'a>>) -> Self {
        let (length, width) = (layers[0].length, layers[0].width);
        let mut image = Image{
            length: length,
            width: width,
            pixels: vec![],
        };

        for idx in 0..(length * width) {
            let mut pixel = 2;
            for layer in layers.iter().rev() {
                if layer.pixels[idx] != 2 {
                    pixel = layer.pixels[idx];
                }
            }
            image.pixels.push(pixel);
        }

        image
    }

    fn png(&self, path: &str) {
        let mut img: image::RgbImage = image::ImageBuffer::new(
            self.width as u32,
            self.length as u32,
        );
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let idx = (self.width as u32 * y + x) as usize;
            *pixel = image::Rgb(if self.pixels[idx] == 1 {
                [255, 255, 255]
            } else {
                [0, 0, 0]
            });
        }
        img.save(path).unwrap()
    }

    fn to_string(&self) -> String {
        let rows = (&self.pixels)
            .chunks(self.width)
            .map(|row| row.iter().map(|p| {
                if *p == 1 {
                    "■"
                } else {
                    " "

                }
            }).collect::<String>() + "\n")
            .collect();
        rows
    }
}

#[aoc(day8, part1)]
fn day8_part1(pixels: &[usize]) -> usize {
    let (length, width) = (6, 25);
    let layers = layers_from_pixels(length, width, pixels);
    let fewest_zeros = layers.iter().min_by_key(|l| l.count(0)).unwrap();
    fewest_zeros.count(1) * fewest_zeros.count(2)
}

#[aoc(day8, part2)]
fn day8_part2(pixels: &[usize]) -> String {
    let (length, width) = (6, 25);
    let layers = layers_from_pixels(length, width, pixels);
    let img = Image::new(layers);
    img.png("/tmp/day8.png");
    img.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_input("123456789012"),
            vec![1,2,3,4,5,6,7,8,9,0,1,2],
        )
    }

    #[test]
    fn test_example_1() {
        let pixels = vec![1,2,3,4,5,6,7,8,9,0,1,2];
        let layers = layers_from_pixels(2, 3, &pixels);
        assert_eq!(layers[0].pixels, &[1,2,3,4,5,6]);
        assert_eq!(layers[1].pixels, &[7,8,9,0,1,2]);
    }

    #[test]
    fn test_example_2() {
        let pixels = vec![0,2,2,2,1,1,2,2,2,2,1,2,0,0,0,0];
        let layers = layers_from_pixels(2, 2, &pixels);
        assert_eq!(Image::new(layers).pixels, &[0,1,1,0]);
    }
}
