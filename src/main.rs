use raylib::prelude::*;

fn main() {
    let image_width = 500;
    let image_height = 500;

    let mut new_image = Image::gen_image_color(
        image_width,
        image_height,
        Color::BLACK
    );

    let pixel1_position_x = 50;
    let pixel1_position_y = 50;

    new_image.draw_pixel(
        pixel1_position_x,
        pixel1_position_y,
        Color::WHITE
    );

    let pixel2_position_x = 150;
    let pixel2_position_y = 150;

    new_image.draw_pixel(
        pixel2_position_x,
        pixel2_position_y,
        Color::WHITE
    );

    let pixel3_position_x = 250;
    let pixel3_position_y = 250;

    new_image.draw_pixel(
        pixel3_position_x,
        pixel3_position_y,
        Color::WHITE
    );

    let pixel4_position_x = 350;
    let pixel4_position_y = 350;

    new_image.draw_pixel(
        pixel4_position_x,
        pixel4_position_y,
        Color::WHITE
    );

    let pixel5_position_x = 450;
    let pixel5_position_y = 450;

    new_image.draw_pixel(
        pixel5_position_x,
        pixel5_position_y,
        Color::WHITE
    );

    let pixel6_position_x = 450;
    let pixel6_position_y = 50;

    new_image.draw_pixel(
        pixel6_position_x,
        pixel6_position_y,
        Color::WHITE
    );

    let pixel7_position_x = 350;
    let pixel7_position_y = 150;

    new_image.draw_pixel(
        pixel7_position_x,
        pixel7_position_y,
        Color::WHITE
    );

    let pixel8_position_x = 150;
    let pixel8_position_y = 350;

    new_image.draw_pixel(
        pixel8_position_x,
        pixel8_position_y,
        Color::WHITE
    );

    let pixel9_position_x = 50;
    let pixel9_position_y = 450;

    new_image.draw_pixel(
        pixel9_position_x,
        pixel9_position_y,
        Color::WHITE
    );

    let pixel10_position_x = 100;
    let pixel10_position_y = 100;

    new_image.draw_pixel(
        pixel10_position_x,
        pixel10_position_y,
        Color::WHITE
    );

    let pixel11_position_x = 200;
    let pixel11_position_y = 200;

    new_image.draw_pixel(
        pixel11_position_x,
        pixel11_position_y,
        Color::WHITE
    );

    let pixel12_position_x = 300;
    let pixel12_position_y = 300;

    new_image.draw_pixel(
        pixel12_position_x,
        pixel12_position_y,
        Color::WHITE
    );

    let pixel13_position_x = 400;
    let pixel13_position_y = 400;

    new_image.draw_pixel(
        pixel13_position_x,
        pixel13_position_y,
        Color::WHITE
    );

    let pixel14_position_x = 400;
    let pixel14_position_y = 100;

    new_image.draw_pixel(
        pixel14_position_x,
        pixel14_position_y,
        Color::WHITE
    );

    let pixel15_position_x = 300;
    let pixel15_position_y = 200;

    new_image.draw_pixel(
        pixel15_position_x,
        pixel15_position_y,
        Color::WHITE
    );

    let pixel16_position_x = 200;
    let pixel16_position_y = 300;

    new_image.draw_pixel(
        pixel16_position_x,
        pixel16_position_y,
        Color::WHITE
    );

    let pixel17_position_x = 100;
    let pixel17_position_y = 400;

    new_image.draw_pixel(
        pixel17_position_x,
        pixel17_position_y,
        Color::WHITE
    );

    let output_file_name = "my_first_image.png";

    new_image.export_image(output_file_name);

    println!("Image saved successfully as '{}'!", output_file_name);
 }