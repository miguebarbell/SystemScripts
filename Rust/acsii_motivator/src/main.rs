use std::env;

use cfonts::{Align, Colors, Fonts, Options, render};
use cfonts::Colors::*;
use cfonts::Fonts::{
    Font3d, FontBlock, FontChrome, FontConsole, FontGrid, FontHuge, FontPallet, FontShade,
    FontSimple, FontSimple3d, FontSimpleBlock, FontSlick, FontTiny,
};
use itertools::Itertools;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Must provide a word!");
        return;
    }
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(1..args.len());
    let selected_word = &args[random_index];
    let one_color_fonts = [
        FontTiny,
        FontSimple,
        FontSimpleBlock,
        FontSimple3d,
        FontConsole,
    ];
    let two_colors_fonts = [
        FontBlock, FontSlick, FontGrid, FontPallet, FontShade, Font3d, FontHuge,
    ];
    let three_colors_fonts = [FontChrome];

    let available_options = ["simple", "gradient", "transition"];
    let option = available_options[rng.gen_range(0..available_options.len())];
    let mut font: Fonts = FontTiny;
    let mut colors = vec![Candy];
    if option == "simple" {
        let random_number_of_colors = rng.gen_range(1..=3);
        if random_number_of_colors == 1 {
            font = one_color_fonts[rng.gen_range(0..one_color_fonts.len())].clone();
            colors = get_random_colors(1);
        } else if random_number_of_colors == 2 {
            font = two_colors_fonts[rng.gen_range(0..two_colors_fonts.len())].clone();
            colors = get_random_colors(2);
        } else {
            font = three_colors_fonts[rng.gen_range(0..three_colors_fonts.len())].clone();
            colors = get_random_colors(3);
        }
    } else if option == "gradient" {
        let all_fonts_for_gradient: Vec<Fonts> = one_color_fonts
            .into_iter()
            .chain(two_colors_fonts)
            .chain(three_colors_fonts)
            .collect();
        font = all_fonts_for_gradient[rng.gen_range(0..all_fonts_for_gradient.len())].clone();
        colors = get_random_colors(2);
    } else if option == "transition" {
        let all_fonts_for_gradient: Vec<Fonts> = one_color_fonts
            .into_iter()
            .chain(two_colors_fonts)
            .chain(three_colors_fonts)
            .collect();
        font = all_fonts_for_gradient[rng.gen_range(0..all_fonts_for_gradient.len())].clone();
        colors = get_random_colors(rng.gen_range(1..15))
    }
    fn get_random_colors(quantity: usize) -> Vec<Colors> {
        let mut rng = rand::thread_rng();
        let available_colours = [
            Red,
            Green,
            Yellow,
            Blue,
            Magenta,
            Cyan,
            Gray,
            RedBright,
            GreenBright,
            YellowBright,
            BlueBright,
            MagentaBright,
            CyanBright,
            Candy,
        ];
        let mut unique_colors_index: Vec<usize> = vec![];
        while unique_colors_index.len() < quantity
            && unique_colors_index.len() <= available_colours.len()
        {
            let random_index_generated = rng.gen_range(0..available_colours.len());
            unique_colors_index.append(&mut vec![random_index_generated]);
            unique_colors_index = unique_colors_index.into_iter().unique().collect();
        }
        unique_colors_index
            .into_iter()
            .map(|index| available_colours[index].clone())
            .collect()
    }
    let output = render(Options {
        text: selected_word.to_string(),
        font,
        align: Align::Left,
        colors,
        ..Options::default()
    });
    print!("{}", output.text);
}
