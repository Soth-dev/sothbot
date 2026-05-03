use crate::func::*;
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage, imageops};
use rand::{prelude::IndexedRandom, rng};
use rayon::prelude::*;
use std::{collections::HashMap, fs, io::Cursor, path::Path};
use teloxide::{
    prelude::*,
    types::{InputFile, ParseMode, ReplyParameters},
};

pub async fn maze(bot: Bot, msg: Message, size: String) -> ResponseResult<()> {
    let (width, height) = size
        .trim()
        .split_once(" ")
        .map(|(w, h)| {
            let w1 = w.trim().parse::<usize>().unwrap_or(10);
            let h1 = h.trim().parse::<usize>().unwrap_or(10);
            if w1 > 100 || h1 > 100 {
                (100, 100)
            } else {
                (w1, h1)
            }
        })
        .unwrap_or((10, 10));
    let grid = create(width, height);
    let fixed = fix(&grid);
    let image_buff = render(&fixed);
    let msg1 = bot
        .send_message(msg.chat.id, q(m("Creating...")))
        .parse_mode(ParseMode::Html)
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
    let mut bytes: Vec<u8> = Vec::new();
    image_buff
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .unwrap_or_default();
    bot.send_photo(msg.chat.id, InputFile::memory(bytes))
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;
    bot.delete_message(msg1.chat.id, msg1.id).await?;
    Ok(())
}

fn create(width: usize, height: usize) -> Vec<Vec<char>> {
    let grid_width = width * 2 + 1;
    let grid_height = height * 2 + 1;

    let mut grid = vec![vec!['#'; grid_width]; grid_height];

    let mut visited = vec![vec![false; width]; height];

    let mut stack = vec![(0_usize, 0_usize)];
    visited[0][0] = true;
    grid[1][1] = ' '; // Carve out the starting cell

    let mut rng = rng();

    let dirs = [(-1_isize, 0_isize), (1, 0), (0, -1), (0, 1)];

    while let Some(&(cx, cy)) = stack.last() {
        let mut unvisited_neighbors = Vec::new();

        for &(dx, dy) in &dirs {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;

            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if !visited[ny][nx] {
                    unvisited_neighbors.push((nx, ny, dx, dy));
                }
            }
        }

        if unvisited_neighbors.is_empty() {
            stack.pop();
        } else {
            let &(nx, ny, dx, dy) = unvisited_neighbors.choose(&mut rng).unwrap();
            visited[ny][nx] = true;

            grid[ny * 2 + 1][nx * 2 + 1] = ' ';

            let wall_x = (cx as isize * 2 + 1 + dx) as usize;
            let wall_y = (cy as isize * 2 + 1 + dy) as usize;
            grid[wall_y][wall_x] = ' ';

            stack.push((nx, ny));
        }
    }

    grid[1][0] = ' ';
    grid[grid_height - 2][grid_width - 1] = ' ';
    grid
}

fn expand_horizontal(row: &[char]) -> Vec<char> {
    let mut new_row = Vec::with_capacity(row.len() * 2);

    for i in 0..row.len() - 1 {
        match (row[i], row[i + 1]) {
            ('#', '#') => {
                new_row.push('#');
                new_row.push('#');
            }
            ('#', ' ') => {
                new_row.push('#');
                new_row.push('l');
            }
            (' ', '#') => {
                new_row.push(' ');
                new_row.push('r');
            }
            (' ', ' ') => {
                new_row.push(' ');
                new_row.push(' ');
            }
            _ => {
                new_row.push(' ');
                new_row.push(' ');
            }
        }
    }
    new_row.push('#');
    new_row
}

fn get_interpolated_tile(c1: char, c2: char, last: &mut char) -> char {
    let res = match (c1, c2, *last) {
        ('#', '#', '#' | 'r' | 'a' | 'd') => '#',
        (' ', ' ', ' ' | 'l' | 'h' | 'k') => ' ',
        ('l', 'l', '#') => 'l',
        ('r', 'r', ' ') => 'r',
        ('#', ' ', 't' | 'w' | 'j') => 't',
        (' ', '#', 'b' | 's' | 'p') => 'b',
        ('#', 'l', '#') => 'w',
        ('#', 'r', 't') => 'a',
        ('l', '#', '#') => 's',
        ('r', '#', 'b') => 'd',
        ('l', ' ', 't') => 'h',
        ('r', ' ', ' ') => 'j',
        (' ', 'l', 'b') => 'k',
        (' ', 'r', ' ') => 'p',
        _ => '#',
    };
    *last = res;
    res
}

fn fix(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }

    let expanded_rows: Vec<Vec<char>> = grid.par_iter().map(|row| expand_horizontal(row)).collect();

    let interpolated_rows: Vec<Vec<char>> = expanded_rows
        .par_windows(2)
        .enumerate()
        .map(|(irow, window)| {
            let new_row1 = &window[0];
            let new_row2 = &window[1];
            let mut new_row3 = Vec::with_capacity(new_row1.len());

            let mut last = if irow == 0 {
                't'
            } else if irow == 1 {
                'b'
            } else {
                '#'
            };

            for i in 0..new_row1.len() - 1 {
                let char_to_push = get_interpolated_tile(new_row1[i], new_row2[i], &mut last);
                new_row3.push(char_to_push);
            }
            new_row3.push('#');
            new_row3
        })
        .collect();

    let row_len = expanded_rows[0].len();
    let mut new_grid = Vec::with_capacity(grid.len() * 2);
    let mut exp_iter = expanded_rows.into_iter();
    let mut int_iter = interpolated_rows.into_iter();

    for _ in 0..grid.len() - 1 {
        new_grid.push(exp_iter.next().unwrap()); // Original expanded row
        new_grid.push(int_iter.next().unwrap()); // Interpolated row
    }

    new_grid.push(vec!['#'; row_len]);

    let rows = new_grid.len();
    if rows >= 4 && row_len >= 2 {
        new_grid[rows - 3][row_len - 1] = ' ';
        new_grid[rows - 4][row_len - 1] = 't';
        new_grid[rows - 2][row_len - 1] = 'b';
        new_grid[rows - 2][row_len - 2] = 'b';
    }

    new_grid
}

fn render(grid: &[Vec<char>]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let path = "assets/tile/";
    let tile_mapping = [
        ('#', "full.png"),
        (' ', "empty.png"),
        ('t', "top.png"),
        ('b', "bottom.png"),
        ('l', "left.png"),
        ('r', "right.png"),
        ('w', "top_left.png"),
        ('a', "top_right.png"),
        ('s', "bottom_left.png"),
        ('d', "bottom_right.png"),
        ('h', "empty_top_left.png"),
        ('j', "empty_top_right.png"),
        ('k', "empty_bottom_left.png"),
        ('p', "empty_bottom_right.png"),
    ];

    let images: HashMap<char, RgbImage> = tile_mapping
        .par_iter()
        .filter_map(|&(char_key, filename)| {
            let full_path = format!("{}{}", path, filename);
            match image::open(&full_path) {
                Ok(img) => Some((char_key, img.to_rgb8())),
                Err(_) => {
                    eprintln!("Warning: Could not load {}", full_path);
                    None
                }
            }
        })
        .collect();

    let cell_size: u32 = 16;
    let width = grid[0].len() as u32 * cell_size;
    let height = grid.len() as u32 * cell_size;

    let rendered_rows: Vec<RgbImage> = grid
        .par_iter()
        .map(|row| {
            let mut row_img = RgbImage::new(width, cell_size);
            for (i, &char_key) in row.iter().enumerate() {
                let x = i as u32 * cell_size;
                if let Some(tile) = images.get(&char_key) {
                    imageops::replace(&mut row_img, tile, x as i64, 0);
                }
            }
            row_img
        })
        .collect();

    let mut img_buf = RgbImage::new(width, height);
    for (ir, row_img) in rendered_rows.iter().enumerate() {
        let y = ir as u32 * cell_size;
        imageops::replace(&mut img_buf, row_img, 0, y as i64);
    }

    let out_path = "/storage/emulated/0/Download/1/out/outputrs.png";

    if let Some(parent) = Path::new(out_path).parent() {
        let _ = fs::create_dir_all(parent);
    }

    if let Err(e) = img_buf.save(out_path) {
        eprintln!("Failed to save image: {}", e);
    }
    img_buf
}
