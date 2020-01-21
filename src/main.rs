

struct Style {
    units: u32, // size of square
    //board: usize, // = 8*units
    corner: u32,  // corner radius
    margin: u32,
    font: String,
    fontsize: u8,
    show_coords: bool,
    flip: bool,
    show_indicator: bool,
    grayscale: bool,
}


struct Board {
    style: Style,
    fen: String,
    pieces: [char; 64],
    color_light: [char; 9],
    color_dark: [char; 9],
}


enum Side {
    White,
    Black,
}


impl Board {

    fn side_to_move(&self) -> Option<Side> {
        // find first space in FEN, then first char after that 
        // is the side to move. Check if that char is 'w'.
        let side_char_index = self.fen.find(' ') + 1;
        match self.fen.chars().nth(side_char_index) {
            'w' => Some(Side::White),
            'b' => Some(Side::Black),
            _ => None,
        }
    }

    fn white_to_move(&self) -> bool {
        self.side_to_move() == Some(Side::White)
    }

    fn black_to_move(&self) -> bool {
        self.side_to_move() == Some(Side::Black)
    }
}


/// Read a file and dump its contents to stdout
fn cat(filename: &str) {
    let mut lines: Vec<u8> = Vec::new();
    File::new(filename).read(&mut lines);
    println!("{}", lines);
}


void
put_svg_header(const char *fen)
{
    int N = units*8 + 2*margin;
	printf("<?xml version='1.0' encoding='utf-8'?>\n"
           "<svg viewBox='0 0 %d %d' style='background-color:#ffffff00' version='1.1' "
           "xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' "
           "xml:space='preserve' x='0px' y='0px' width='%d' height='%d'>\n", N, N, N, N);
	printf("<!-- Creator: SVGBoard, Copyright 2019 David S. Smith <david.smith@gmail.com> -->\n");
	printf("<!-- FEN: %s -->\n", fen);

}


void
put_squares ()
{
    int N = 8*units;

	
    //printf("<!-- BOARD MARGIN -->\n");
	//printf("<rect x='%d' y='%d' width='%d' height='%d' fill='#%s' stroke='#000' stroke-width='%d'/>\n",
			//0, 0, margin*2+N, margin*2+N, color_dark, border);

    printf("<!-- DARK SQUARES -->\n");
    if (flags.grayscale) // cross hatched: 
    {
		int d = units/12;
        printf("<pattern id='crosshatch' width='%d' height='%d' "
               "patternTransform='rotate(45 0 0)' patternUnits='userSpaceOnUse'>\n"
                 "  <line x1='0' y1='0' x2='0' y2='%d' style='stroke:#000; stroke-width:1' />\n "
               "</pattern>\n", d, d, d);
                 //"  <rect x='0' y='0' width='%d' height='%d' fill='#fff' />\n "
		printf("<pattern id='diagonalHatch' patternUnits='userSpaceOnUse' width='4' height='4'>\n "
  				"<path d='M-1,1 l2,-2 M0,4 l4,-4 M3,5 l2,-2'\n "
        		"style='stroke:black; stroke-width:1' />\n "
				"</pattern>\n");
        printf("<rect x='%d' y='%d' width='%d' height='%d' fill='url(#crosshatch)' />\n",
                margin, margin, N, N);
        strlcpy(color_light, "fff", 4);
    }
    else // shaded
        printf("<rect x='%d' y='%d' width='%d' height='%d' fill='#%s' />\n",
                margin, margin, N, N, color_dark);

    printf("<!-- LIGHT SQUARES -->\n");
    printf("<pattern id='checkerboard' x='%d' y='%d' width='%d' height='%d' patternUnits='userSpaceOnUse'>\n", 
            margin, margin, 2*units, 2*units);
    printf("  <rect x='0' y='0' width='%d' height='%d' style='fill:#%s;' />\n", 
            units, units, color_light);
    printf("  <rect x='%d' y='%d' width='%d' height='%d' style='fill:#%s;' />\n",
            units, units, units, units, color_light);
    printf("</pattern>\n");
    printf("<rect x='%d' y='%d' width='%d' height='%d' fill='url(#checkerboard)' />\n",
            margin, margin, N, N);
}


void 
put_border ()
{
    int n, N;
    int r = corner;
    int t = 0; //units / 32; // border gap thickness
    printf("<!-- BORDER -->\n"); 
    n = margin - t;
    N = board + 2*t;
    printf("<rect x='%d' y='%d' width='%d' height='%d' fill='none' " 
            "stroke-width='%d' stroke-location='outside' stroke='#fff' rx='%d' ry='%d' />\n",
            n, n, N, N, t, r, r);
    n -= border == 1 ? border : border/2;
    N += border == 1 ? 2*border : border; 
    printf("<rect x='%d' y='%d' width='%d' height='%d' fill='none' " 
            "stroke-width='%d' stroke-location='outside' stroke='#000' rx='%d' ry='%d' />\n",
            n, n, N, N, border, r, r);
    //n = margin - border/6; 
    //N = board + border/3;
    //printf("<rect x='%d' y='%d' width='%d' height='%d' fill='none' " 
     //         "stroke-width='%d' stroke='#000' rx='%d' ry='%d' />\n",
    //        n, n, N, N, border/6, r, r);
}


void
put_piece (const char c, const int x, const int y)
{
    char path[80];
    printf("<svg x='%d' y='%d' width='%d' height='%d'>\n", x, y, units, units);
    if (isupper(c))
        sprintf(path, "svg/%s/w%c.svg", pieces, c);
    else
        sprintf(path, "svg/%s/b%c.svg", pieces, toupper(c));
    cat(path);
    printf("</svg>\n");
}


void 
put_move_indicator ()
{
    int cx = units*8 + 3*margin/2;
    int cy = margin + units/2;
    int dy = units/5, dx = units/5;
    int stroke = 1;

    if ((whitetomove && !flags.flip) || (blacktomove && flags.flip)) {
        cy += 7*units;
        dy = -dy;
    }
    if (whitetomove) {
        //size -= 1;
        stroke = 2;
    }
    char *fill = whitetomove ? "fff" : "000";
    printf("<path d='M%d %d l%d %d l%d %d q%d %d %d %d' stroke='black' stroke-width='%d' fill='#%s' />\n",
            cx-dx, cy-dy, dx, 2*dy, dx, -2*dy, -dx, dy, -2*dx, 0, stroke, fill);
}


void
put_coords ()
{
    const char *files = "abcdefgh";
    int rank_y0 = margin + units/2 + fontsize/2;
    int rank_x0 = margin - fontsize - border;
    int file_x0 = margin + units/2 - fontsize/4; 
    int file_y0 = margin + units*8 + border + fontsize;
    
    printf("<svg>\n");
      printf("<style> .small { font: normal %dpx %s; } </style>\n", fontsize, font);

    for (int file = 0; file < 8; ++file)
        printf("<text x=\"%d\" y=\"%d\" fill=\"#000\" class=\"small\">%c</text>\n", 
            file_x0+units*file, file_y0, files[flags.flip ? 7-file : file]);
    for (int rank = 0; rank < 8; ++rank)
        printf("<text x=\"%d\" y=\"%d\" fill=\"#000\" class=\"small\">%d</text>\n", 
            rank_x0, rank*units+rank_y0, flags.flip ? rank+1 : 8-rank);

    printf("</svg>\n");
}


void
build_board (const char *fen)
{
    whitetomove = (sidetomove(fen) == 'w');
    blacktomove = 1 - whitetomove;
    board = 8*units;
	int step = flags.flip ? -units : units;

    put_svg_header(fen);
    put_squares();
    put_border();

    if (flags.coords) 
        put_coords();

    int i = 0, x = margin, y = margin;
	if (flags.flip) {
		x += 7*units;
		y += 7*units;
	}
	puts("<!-- PIECES -->");
    for ( ; i < strlen(fen); ++i)
    {
        if (isalpha(fen[i])) {
            put_piece(fen[i], x, y);
            x += step;
        } else if (isdigit(fen[i]))
            x += step*atoi(&fen[i]);
        else if (fen[i] == '/') {
            x = flags.flip ? 7*units + margin : margin;
            y += step;
        } else if (fen[i] == ' ')
            break;
        else
            err(EX_DATAERR, "Weird FEN received: %s\n", fen);
    }
    if (flags.indicator)
        put_move_indicator();
    puts("</svg>");
}


impl Board {
    fn new () -> Board {
        flags.coords = 0;
        flags.flip = 0;
        flags.grayscale = 0;
        flags.indicator = 0;
        strlcpy(pieces, "merida", sizeof pieces);
        units = 64;
        border = 2;
        corner = 0; //units/16;
        margin = 48;
        fontsize = 20;
        strlcpy(font, "sans-serif", 64);
	    // GRAY
        //strlcpy(color_light, "fff", 4);
        //strlcpy(color_dark, "bbb", 4);
	    // BROWN
        strlcpy(color_light, "a48c62", 7);
        strlcpy(color_dark, "846c40", 7);
	    // GREEN: #77955f #eeeed3
	    // BROWN: #846c40 #a48c62
        let startfen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    }
}


fn print_usage() {
    eprintln!("Usage: svgboard [-c] [-g] [-h] [-p <pieces>] [-r]");
    eprintln!("\t-b\t\t border thickness (default: {})", border);
    eprintln!("\t-c\t\t turn on coordinates (default: {})", flags.coords);
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

int 
main (int argc, char *argv[])
{
    int c;

    // defaults
    set_defaults();

    let mut b = Board::new();

    while ((c = getopt(argc, argv, "b:cd:f:F:gil:m:p:rs:h")) != -1)
    {
        switch (c) {
        case 'b':
            border = atoi(optarg);
            break;
        case 'c':
            flags.coords = 1;
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
            flags.grayscale = 1;
            break;
        case 'i':
            flags.indicator = 1;
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
            flags.flip = 1;
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

    return 0;
}
