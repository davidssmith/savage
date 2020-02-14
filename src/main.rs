use std::fs::File;
use std::io::{self, Read};

#[derive(PartialEq, Debug)]
enum Side {
    White,
    Black,
}

#[derive(Debug)]
struct Board {
    fen: String,

    units: i32, // relative size of a square
    //board: usize, // = 8*units
    corner_radius: i32,
    margin: i32,
    border: i32,

    font: String,
    font_size: i32,
    pieces: String,
    color_light: String,
    color_dark: String,

    show_coords: bool,
    flip: bool,
    show_indicator: bool,
    grayscale: bool,
}

impl Default for Board {
    fn default() -> Self {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned();
        let units: i32 = 64;
        let corner_radius: i32 = 0;
        let margin: i32 = 48;
        let border: i32 = 2;
        let font = "sans-serif".to_owned();
        let font_size: i32 = 20;
        let pieces = "merida".to_owned();
        let color_light = "a48c62".to_owned();
        let color_dark = "846c40".to_owned();
        let show_coords = false;
        let flip = false;
        let show_indicator = false;
        let grayscale = false;
        Board {
            fen,
            units,
            corner_radius,
            margin,
            border,
            font,
            font_size,
            pieces,
            color_light,
            color_dark,
            show_coords,
            flip,
            show_indicator,
            grayscale,
        }
    }
}

/// Read a file and dump its contents to stdout
fn cat(filename: &str) -> io::Result<()> {
    let mut contents = String::new();
    let mut f = File::open(filename)?;
    f.read_to_string(&mut contents)?;
    print!("{}", contents);
    Ok(())
}

impl Board {
    fn side_to_move(&self) -> Option<Side> {
        // find first space in FEN, then first char after that
        // is the side to move. Check if that char is 'w'.
        let side_char_index = self.fen.find(' ')? + 1;
        match self.fen.chars().nth(side_char_index) {
            Some('w') => Some(Side::White),
            Some('b') => Some(Side::Black),
            _ => None,
        }
    }

    fn white_to_move(&self) -> bool {
        self.side_to_move().unwrap() == Side::White
    }

    fn black_to_move(&self) -> bool {
        self.side_to_move().unwrap() == Side::Black
    }

    fn put_svg_header(&self) {
        let nouter = self.units * 8 + 2 * self.margin;
        println!(
            "<?xml version='1.0' encoding='utf-8'?>\n\
           <svg viewBox='0 0 {} {}' style='background-color:#ffffff00' version='1.1' \
           xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' \
           xml:space='preserve' x='0px' y='0px' width='{}' height='{}'>",
            nouter, nouter, nouter, nouter
        );
        println!("<!-- Creator: Savage, Copyright 2020 David S. Smith <david.smith@gmail.com> -->");
        println!("<!-- FEN: {} -->", self.fen);
    }

    fn put_squares(&mut self) {
        let nouter = 8 * self.units;
        println!("<!-- DARK SQUARES -->");
        if self.grayscale {
            // cross hatched:
            let d = self.units / 12;
            println!(
                "<pattern id='crosshatch' width='{}' height='{}' \
               patternTransform='rotate(45 0 0)' patternUnits='userSpaceOnUse'>\n\
                   <line x1='0' y1='0' x2='0' y2='{}' style='stroke:#000; stroke-width:1' />\n \
               </pattern>",
                d, d, d
            );
            //"  <rect x='0' y='0' width='{}' height='{}' fill='#fff' />\n "
            println!("<pattern id='diagonalHatch' patternUnits='userSpaceOnUse' width='4' height='4'>\n \
  				<path d='M-1,1 l2,-2 M0,4 l4,-4 M3,5 l2,-2'\n \
        		style='stroke:black; stroke-width:1' />\n \
				</pattern>");
            println!(
                "<rect x='{}' y='{}' width='{}' height='{}' fill='url(#crosshatch)' />",
                self.margin, self.margin, nouter, nouter
            );
            self.color_light = "fff".to_owned();
        } else {
            // shaded
            println!(
                "<rect x='{}' y='{}' width='{}' height='{}' fill='#{}' />",
                self.margin, self.margin, nouter, nouter, self.color_dark
            );
        }

        println!("<!-- LIGHT SQUARES -->");
        println!("<pattern id='checkerboard' x='{}' y='{}' width='{}' height='{}' patternUnits='userSpaceOnUse'>",
            self.margin, self.margin, 2*self.units, 2*self.units);
        println!(
            "  <rect x='0' y='0' width='{}' height='{}' style='fill:#{};' />",
            self.units, self.units, self.color_light
        );
        println!(
            "  <rect x='{}' y='{}' width='{}' height='{}' style='fill:#{};' />",
            self.units, self.units, self.units, self.units, self.color_light
        );
        println!("</pattern>");
        println!(
            "<rect x='{}' y='{}' width='{}' height='{}' fill='url(#checkerboard)' />",
            self.margin, self.margin, nouter, nouter
        );
    }

    fn put_border(&self) {
        let r = self.corner_radius;
        let t = 0i32; //units / 32; // border gap thickness
        println!("<!-- BORDER -->");
        let mut n = self.margin - t;
        let board = 8 * self.units;
        let mut nouter = board + 2 * t;
        println!(
            "<rect x='{}' y='{}' width='{}' height='{}' fill='none' \
            stroke-width='{}' stroke-location='outside' stroke='#fff' rx='{}' ry='{}' />",
            n, n, nouter, nouter, t, r, r
        );
        if self.border == 1 {
            n -= self.border;
            nouter += 2 * self.border;
        } else {
            n -= self.border / 2;
            nouter += self.border;
        }
        println!(
            "<rect x='{}' y='{}' width='{}' height='{}' fill='none' \
            stroke-width='{}' stroke-location='outside' stroke='#000' rx='{}' ry='{}' />",
            n, n, nouter, nouter, self.border, r, r
        );
    }

    fn put_piece(&self, c: char, x: i32, y: i32) -> io::Result<()> {
        println!(
            "<svg x='{}' y='{}' width='{}' height='{}'>",
            x, y, self.units, self.units
        );
        if c.is_uppercase() {
            let path = format!("svg/{}/w{}.svg", self.pieces, c);
            cat(&path)?;
        } else {
            let path = format!("svg/{}/b{}.svg", self.pieces, c);
            cat(&path)?;
        }
        println!("</svg>");
        Ok(())
    }

    fn put_move_indicator(&self) {
        let cx = self.units * 8 + 3 * self.margin / 2;
        let mut cy = self.margin + self.units / 2;
        let mut dy: i32 = self.units / 5;
        let dx: i32 = self.units / 5;
        let stroke = if self.white_to_move() { 2 } else { 1 };

        if (self.white_to_move() && !self.flip) || (self.black_to_move() && self.flip) {
            cy += 7 * self.units;
            dy = -dy;
        }
        let fill = if self.white_to_move() { "fff" } else { "000" };
        println!("<path d='M{} {} l{} {} l{} {} q{} {} {} {}' stroke='black' stroke-width='{}' fill='#{}' />",
            cx-dx, cy-dy, dx, 2*dy, dx, -2*dy, -dx, dy, -2*dx, 0, stroke, fill);
    }

    fn put_coords(&self) {
        let files: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let rank_y0 = self.margin + self.units / 2 + self.font_size / 2;
        let rank_x0 = self.margin - self.font_size - self.border;
        let file_x0 = self.margin + self.units / 2 - self.font_size / 4;
        let file_y0 = self.margin + self.units * 8 + self.border + self.font_size;

        println!("<svg>");
        println!(
            "<style> .small {{ font: normal {}px {}; }} </style>",
            self.font_size, self.font
        );

        for file in 0..8 {
            let i = if self.flip {
                7 - file as usize
            } else {
                file as usize
            };
            println!(
                "<text x=\"{}\" y=\"{}\" fill=\"#000\" class=\"small\">{}</text>",
                file_x0 + self.units * file,
                file_y0,
                files[i]
            );
        }
        for rank in 0..8 {
            println!(
                "<text x=\"{}\" y=\"{}\" fill=\"#000\" class=\"small\">{}</text>",
                rank_x0,
                rank * self.units + rank_y0,
                if self.flip { rank + 1 } else { 8 - rank }
            );
        }
        println!("</svg>");
    }

    fn build_board(&mut self) {
        let _board = 8 * self.units;
        let step: i32 = if self.flip { -self.units } else { self.units };

        self.put_svg_header();
        self.put_squares();
        self.put_border();

        if self.show_coords {
            self.put_coords();
        }

        let mut x: i32 = self.margin;
        let mut y: i32 = self.margin;
        if self.flip {
            x += 7 * self.units;
            y += 7 * self.units;
        }
        print!("<!-- PIECES -->");
        for f in self.fen.chars() {
            if f.is_alphabetic() {
                self.put_piece(f, x, y).unwrap();
                x += step;
            } else if f.is_numeric() {
                x += step * f.to_digit(10).unwrap() as i32;
            } else if f == '/' {
                x = if self.flip {
                    7 * self.units + self.margin
                } else {
                    self.margin
                };
                y += step;
            } else if f == ' ' {
                break;
            } else {
                panic!("Weird FEN received: {}", self.fen);
            }
        }
        if self.show_indicator {
            self.put_move_indicator();
        }
        println!("</svg>");
    }
}


fn main() -> io::Result<()> {
    use argparse::{ArgumentParser, Store, StoreTrue};
    let mut b = Board::default();
    let mut output = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("SVG chess board creator");
        ap.refer(&mut b.border)
            .add_option(&["-b", "--border"], Store, "border width");
        ap.refer(&mut b.show_coords)
            .add_option(&["-c", "--coords"], StoreTrue, "display coordinates");
        ap.refer(&mut b.color_dark)
            .add_option(&["-d", "--color-dark"], Store, "RGB hex color of dark squares");
        ap.refer(&mut b.font)
            .add_option(&["-F", "--font"], Store, "font face");
        ap.refer(&mut b.font_size)
            .add_option(&["-f", "--font-size"], Store, "font size (pts)");
        ap.refer(&mut b.grayscale)
            .add_option(&["-g", "--grayscale"], StoreTrue, "grayscale");
        ap.refer(&mut b.show_indicator)
            .add_option(&["-i", "--indicator"], StoreTrue, "show move indicator");
        ap.refer(&mut b.color_light)
            .add_option(&["-l", "--color-light"], Store, "RGB hex color of light squares");
        ap.refer(&mut b.margin)
            .add_option(&["-m", "--margin"], Store, "margin width");
        ap.refer(&mut b.pieces)
            .add_option(&["-p", "--pieces"], Store, "pieces theme");
        ap.refer(&mut b.flip)
            .add_option(&["-r", "--flip"], StoreTrue, "flip board to black side down");
        ap.refer(&mut b.units)
            .add_option(&["-s", "--units"], Store, "length units, image scale");
        ap.refer(&mut b.fen)
            .add_argument("FEN", Store, "FEN board position (default: start position)");
        ap.refer(&mut output)
            .add_argument("output", Store, "output SVG file (default: stdout)");
        ap.parse_args_or_exit();
    }
    //TODO: redirect output to file
    b.build_board();
    Ok(())
}
