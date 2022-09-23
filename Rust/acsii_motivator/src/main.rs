use cfonts::{Fonts, Options, render};
use cfonts::Fonts::{Font3d, FontBlock, FontChrome, FontConsole, FontGrid, FontHuge, FontPallet, FontShade, FontSimple, FontSimple3d, FontSimpleBlock, FontSlick, FontTiny};
use itertools::Itertools;
use rand::Rng;
use std::env;

fn main() {
    // this accept and array of words, and randomly style and print the word in the console.
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Must provide a word!")
    }
    //args.remove(0);
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(1..args.len());
    let selected_word = &args[random_index];
    // generate a random style
    let one_color_fonts = [FontTiny, FontSimple, FontSimpleBlock, Fonts::FontSimple3d, Fonts::FontConsole];
    let two_colors_fonts = [FontBlock, FontSlick, FontGrid, Fonts::FontPallet, Fonts::FontShade, Fonts::Font3d, Fonts::FontHuge];
    let three_colors_fonts = [FontChrome];

    let available_options = ["simple", "gradient", "transition"];
    let option = available_options[rng.gen_range(0..available_options.len())];
    let mut font: Fonts = FontTiny;
    let mut colors = "".to_string();
    if option == "simple" {
        // select a font
        let random_number_of_colors = rng.gen_range(1..=3);
        println!("number of random colors: {}", random_number_of_colors);
        if random_number_of_colors == 1 {
            font = one_color_fonts[rng.gen_range(0..one_color_fonts.len())].clone();
            colors = get_random_colors(1);
            //colors = available_colours[rng.gen_range(0..available_colours.len())];
        } else if random_number_of_colors == 2 {
            font = two_colors_fonts[rng.gen_range(0..two_colors_fonts.len())].clone();
            colors = get_random_colors(2);
            // this will be used in gradient
        } else {
            font = three_colors_fonts[rng.gen_range(0..three_colors_fonts.len())].clone();
            colors = get_random_colors(3);
        }
    } else if option == "gradient" {
        let all_fonts_for_gradient: Vec<Fonts> = one_color_fonts.into_iter().chain(two_colors_fonts).chain(three_colors_fonts).collect();
        font = all_fonts_for_gradient[rng.gen_range(0..all_fonts_for_gradient.len())].clone();
        colors = get_random_colors(2);
    } else if option == "transition" {
        let all_fonts_for_gradient: Vec<Fonts> = one_color_fonts.into_iter().chain(two_colors_fonts).chain(three_colors_fonts).collect();
        font = all_fonts_for_gradient[rng.gen_range(0..all_fonts_for_gradient.len())].clone();
        colors = get_random_colors(rng.gen_range(1..15))
    }
    fn get_random_colors(quantity: usize) -> String {
        let mut rng = rand::thread_rng();
        let available_colours = [
            "red",
            "green",
            "yellow",
            "blue",
            "magenta",
            "cyan",
            "gray",
            "redBright",
            "greenBright",
            "yellowBright",
            "blueBright",
            "magentaBright",
            "cyanBright",
            "whiteBright",
        ];
        let mut unique_colors_index: Vec<usize> = Vec::new();
        while unique_colors_index.len() < quantity && unique_colors_index.len() <= available_colours.len()
        {
            let random_index_generated = rng.gen_range(0..available_colours.len());
            unique_colors_index.append(&mut vec!(random_index_generated));
            unique_colors_index = unique_colors_index.into_iter().unique().collect();
        }
        return unique_colors_index.into_iter().map(|index| available_colours[index]).join(",");
    }
    println!("selected word: {:?}", selected_word);
    println!("font: {:?}", font);
    println!("colors: {}", colors);
    println!("option: {}", option);
    let output = render(Options {
        text: selected_word.to_string(),
        font,
        ..Options::default()
    });
    println!("{}", output.text);
}
