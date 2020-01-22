use std::fs::{File};
use std::io::{self, Read};

#[derive(PartialEq,Debug)]
enum Side {
    White,
    Black,
}

#[derive(Debug)]
struct Board {
    fen: &'static str,

    units: i32, // relative size of a square
    //board: usize, // = 8*units
    corner_radius: i32,
    margin: i32,
    border: i32,

    font: &'static str,
    font_size: i32,
    pieces: &'static str,
    color_light: &'static str,
    color_dark: &'static str,

    show_coords: bool,
    flip: bool,
    show_indicator: bool,
    grayscale: bool,
}

impl Default for Board {
    fn default() -> Self {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let units: i32 = 64;
        let corner_radius: i32 = 0;
        let margin: i32 = 48;
        let border: i32 = 2;
        let font = "sans-serif";
        let font_size: i32 = 20;
        let pieces = "merida";
        let color_light = "a48c62";
        let color_dark = "846c40";
        let show_coords = false;
        let flip = false;
        let show_indicator = false;
        let grayscale = false;
        Board { fen, units, corner_radius, margin, border, font, font_size, pieces, color_light,
        color_dark, show_coords, flip, show_indicator, grayscale }
    }
}

/// Read a file and dump its contents to stdout
fn cat(filename: &str) -> io::Result<()> {
    let mut contents = String::new();
    let f = File::open(filename)?;
    f.read_to_string(&mut contents);
    println!("{}", contents);
    Ok(())
}

impl Board {
    fn new() -> Board {
Board::default()
    }

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
    let N = self.units*8 + 2*self.margin;
	println!("<?xml version='1.0' encoding='utf-8'?>\n\
           <svg viewBox='0 0 {} {}' style='background-color:#ffffff00' version='1.1' \
           xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' \
           xml:space='preserve' x='0px' y='0px' width='{}' height='{}'>", N, N, N, N);
	println!("<!-- Creator: Savage, Copyright 2020 David S. Smith <david.smith@gmail.com> -->");
	println!("<!-- FEN: {} -->", self.fen);
}


fn put_squares (&self) {
    let N = 8*self.units;
    println!("<!-- DARK SQUARES -->");
    if self.grayscale { // cross hatched:
		let d = self.units/12;
        println!("<pattern id='crosshatch' width='{}' height='{}' \
               patternTransform='rotate(45 0 0)' patternUnits='userSpaceOnUse'>\n\
                   <line x1='0' y1='0' x2='0' y2='{}' style='stroke:#000; stroke-width:1' />\n \
               </pattern>", d, d, d);
                 //"  <rect x='0' y='0' width='{}' height='{}' fill='#fff' />\n "
		println!("<pattern id='diagonalHatch' patternUnits='userSpaceOnUse' width='4' height='4'>\n \
  				<path d='M-1,1 l2,-2 M0,4 l4,-4 M3,5 l2,-2'\n \
        		style='stroke:black; stroke-width:1' />\n \
				</pattern>");
        println!("<rect x='{}' y='{}' width='{}' height='{}' fill='url(#crosshatch)' />",
                self.margin, self.margin, N, N);
        let color_light = "fff";
    }
    else { // shaded
        println!("<rect x='{}' y='{}' width='{}' height='{}' fill='#{}' />",
                 self.margin, self.margin, N, N, self.color_dark);
    }

    println!("<!-- LIGHT SQUARES -->");
    println!("<pattern id='checkerboard' x='{}' y='{}' width='{}' height='{}' patternUnits='userSpaceOnUse'>",
            self.margin, self.margin, 2*self.units, 2*self.units);
    println!("  <rect x='0' y='0' width='{}' height='{}' style='fill:#{};' />",
            self.units, self.units, self.color_light);
    println!("  <rect x='{}' y='{}' width='{}' height='{}' style='fill:#{};' />",
            self.units, self.units, self.units, self.units, self.color_light);
    println!("</pattern>");
    println!("<rect x='{}' y='{}' width='{}' height='{}' fill='url(#checkerboard)' />",
            self.margin, self.margin, N, N);
}


fn put_border(&self) {
    let r = self.corner_radius;
    let t = 0i32; //units / 32; // border gap thickness
    println!("<!-- BORDER -->");
    let mut n = self.margin - t;
    let board = 8*self.units;
    let mut N = board + 2*t;
    println!("<rect x='{}' y='{}' width='{}' height='{}' fill='none' \
            stroke-width='{}' stroke-location='outside' stroke='#fff' rx='{}' ry='{}' />",
            n, n, N, N, t, r, r);
    if self.border == 1 {
        n -= self.border;
        N += 2*self.border;
    } else {
        n -= self.border / 2;
        N += self.border;
    }
    println!("<rect x='{}' y='{}' width='{}' height='{}' fill='none' \
            stroke-width='{}' stroke-location='outside' stroke='#000' rx='{}' ry='{}' />",
            n, n, N, N, self.border, r, r);
}


fn put_piece(&self, c: char, x: i32, y: i32) {
    println!("<svg x='{}' y='{}' width='{}' height='{}'>\n", x, y, self.units, self.units);
    if c.is_uppercase() {
        let path = format!("svg/{}/w{}.svg", self.pieces, c);
        cat(&path);
    } else {
        let path = format!("svg/{}/b{}.svg", self.pieces, c);
        cat(&path);
    }
    println!("</svg>");
}


fn put_move_indicator(&self) {
    let cx = self.units*8 + 3*self.margin/2;
    let mut cy = self.margin + self.units/2;
    let mut dy: i32 = self.units/5;
    let dx: i32 = self.units/5;
    let stroke = if self.white_to_move() { 2 } else { 1 };

    if (self.white_to_move() && !self.flip) ||
        (self.black_to_move() && self.flip) {
        cy += 7*self.units;
        dy = -dy;
    }
    let fill = if self.white_to_move() { "fff" } else { "000" };
    println!("<path d='M{} {} l{} {} l{} {} q{} {} {} {}' stroke='black' stroke-width='{}' fill='#{}' />",
            cx-dx, cy-dy, dx, 2*dy, dx, -2*dy, -dx, dy, -2*dx, 0, stroke, fill);
}


    fn put_coords(&self) {
        let files: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let rank_y0 = self.margin + self.units/2 + self.font_size/2;
        let rank_x0 = self.margin - self.font_size - self.border;
        let file_x0 = self.margin + self.units/2 - self.font_size/4;
        let file_y0 = self.margin + self.units*8 + self.border + self.font_size;

        println!("<svg>");
         println!("<style> .small { font: normal {}px {}; } </style>", fontsize, font);

        for file in 0..8 {
            println!("<text x=\"{}\" y=\"{}\" fill=\"#000\" class=\"small\">%c</text>",
                file_x0+self.units*file, file_y0, files[if self.flip { 7-file } else { file }]);
        }
        for rank in 0..8 {
            println!("<text x=\"{}\" y=\"{}\" fill=\"#000\" class=\"small\">{}</text>",
                rank_x0, rank*self.units+rank_y0, if self.flip  { rank+1 } else { 8-rank });
        }
        println!("</svg>");
    }


    fn build_board(&self) {
        let board = 8*self.units;
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
		    x += 7*self.units;
		    y += 7*self.units;
	    }
	    print!("<!-- PIECES -->");
        for f in self.fen.chars() {
            if f.is_alphabetic() {
                self.put_piece(f, x, y);
                x += step;
            } else if f.is_numeric() {
                x += step * f.parse::<i8>().unwrap();
            } else if f == '/' {
                x = if self.flip { 7*units + margin } else { margin };
                y += step;
            } else if f == ' ' {
                break;
            } else {
                panic!("Weird FEN received: {}", fen);
            }
        }
        if self.indicator {
            self.put_move_indicator();
        }
        puts("</svg>");
    }

}


fn print_usage() {
    eprintln!("Usage: svgboard [-c] [-g] [-h] [-p <pieces>] [-r]");
    eprintln!("\t-b\t\t border thickness (default: {})", border);
    eprintln!("\t-c\t\t turn on coordinates (default: {})", self.coords);
    eprintln!("\t-d\t\t dark square color (default: {})", color_dark);
    eprintln!("\t-f\t\t font size (default: {})", fontsize);
    eprintln!("\t-F\t\t font name (default: {})", font);
    eprintln!("\t-g\t\t grayscale, suitable for print");
    eprintln!("\t-i\t\t show side-to-move indicator");
    eprintln!("\t-l\t\t light square color (default: {})", color_light);
    eprintln!("\t-m\t\t margin size (default: {})", margin);
    eprintln!("\t-p <pieces>\t pieces to use (default: {})", pieces);
    eprintln!("\t-r \t\t flip if black to move instead of indicator\n");
    eprintln!("\t-s \t\t square size (default: {})", units);
}

fn main() {

    let mut b = Board::new();

    while ((c = getopt(argc, argv, "b:cd:f:F:gil:m:p:rs:h")) != -1)
    {
        switch (c) {
        case 'b':
            border = atoi(optarg);
            break;
        case 'c':
            self.coords = 1;
            break;
        case 'd':
            strlcpy(color_dark, optarg, 9);
            break;
        case 'F':
            strlcpy(font, optarg, 64);
            break;
        case 'f':
            fontsize = atoi(optarg);
            break;
        case 'g':
            self.grayscale = 1;
            break;
        case 'i':
            self.indicator = 1;
            break;
        case 'l':
            strlcpy(color_light, optarg, 9);
            break;
        case 'm':
            margin = atoi(optarg);
            break;
        case 'p':
            strlcpy(pieces, optarg, sizeof pieces);
            break;
        case 'r':
            self.flip = 1;
            break;
        case 's':
            units = atoi(optarg);
            break;
        case 'h':
        default:
            print_usage();
            return 1;
        }
    }
    argc -= optind;
    argv += optind;
    build_board(argc > 0 ? *argv : startfen);

}
