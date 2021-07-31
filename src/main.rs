use clap::{Arg, App};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use linecount::count_lines;
use lasso::{Key, Rodeo};

fn read_file(filename: &str) -> impl Iterator<Item = String> {
    let file = BufReader::new(File::open(filename).unwrap());
    file.lines().filter_map(|line| line.ok())
}

pub fn load3nt<'a>(
    filename: &str
) -> impl Iterator<Item = (String, String, String)> + 'a {
    read_file(filename)
        .map(move |line| {
            let mut line_clean = line;

            line_clean.pop();

            line_clean.pop();

            let mut elts = line_clean.split(" ");

            (
                elts.next().unwrap().parse().unwrap(),
                elts.next().unwrap().parse().unwrap(),
                elts.next().unwrap().parse().unwrap(),
            )
        })
}

fn main() {
    let matches = App::new(".nt format encoder")
        .version("0.1.0")
        .author("Rucy")
        .about("Encodes .nt abox and tboxes")
        .arg(Arg::new("TBOX_PATH")
             .about("Sets the tbox file path")
             .required(true)
             .index(1))
        .arg(Arg::new("ABOX_PATH")
             .about("Sets the abox file path")
             .required(true)
             .index(2))
        .arg(Arg::new("SPLIT")
             .about("In how many files should the abox be split?")
             .required(true)
             .index(3))
        .get_matches();

    let t_path = matches.value_of("TBOX_PATH").unwrap();

    let a_path = matches.value_of("ABOX_PATH").unwrap();

    let split_number = matches
	.value_of("SPLIT")
	.unwrap()
	.parse::<i64>()
	.unwrap();

    let wc: i64 = (count_lines(File::open(a_path).unwrap()).unwrap() as i64) + 1;

    println!("Abox size: {:?} triples", wc);

    let mut split_line_counts = vec![0i64;split_number as usize];

    for length in 0..(split_line_counts.len()) {
	split_line_counts[length] = wc / split_number;
    }

    for length in 0..(wc % split_number) {
	split_line_counts[length as usize] = split_line_counts[length as usize] + 1;
    }

    let raw_tbox = load3nt(t_path);

    let mut encoded_tbox_file = File::create("tbox.ntenc").unwrap();

    let mut grand_ole_pry = Rodeo::default();

    let rdfsco: &str = "<http://www.w3.org/2000/01/rdf-schema#subClassOf>";
    let rdfspo:  &str = "<http://www.w3.org/2000/01/rdf-schema#subPropertyOf>";
    let rdfsd:  &str = "<http://www.w3.org/2000/01/rdf-schema#domain>";
    let rdfsr:  &str = "<http://www.w3.org/2000/01/rdf-schema#range>";
    let rdft: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>";
    let owltr:  &str = "<http://www.w3.org/2002/07/owl#TransitiveProperty>";
    let owlio:  &str = "<http://www.w3.org/2002/07/owl#inverseOf>";

    grand_ole_pry.get_or_intern(rdfsco);
    grand_ole_pry.get_or_intern(rdfspo);
    grand_ole_pry.get_or_intern(rdfsd);
    grand_ole_pry.get_or_intern(rdfsr);
    grand_ole_pry.get_or_intern(rdft);
    grand_ole_pry.get_or_intern(owltr);
    grand_ole_pry.get_or_intern(owlio);

    raw_tbox.for_each(|triple| {
	let s = &triple.0[..];
	let p = &triple.1[..];
	let o = &triple.2[..];

	let key_s = grand_ole_pry.get_or_intern(s);
	let key_p = grand_ole_pry.get_or_intern(p);
	let key_o = grand_ole_pry.get_or_intern(o);

	let key_s_int = key_s.into_usize();
	let key_p_int = key_p.into_usize();
	let key_o_int = key_o.into_usize();

	writeln!(&mut encoded_tbox_file, "{:?} {:?} {:?}", key_s_int, key_p_int, key_o_int);
    });

    let mut raw_abox = load3nt(a_path);

    for item in 0..(split_line_counts.len()) {

	let mut abox_part = File::create(format!("abox_part{}.ntenc", &item)).unwrap();

	for length in 0..split_line_counts[item] {

	    if let Some(current_triple) = raw_abox.next() {

		let s = &current_triple.0[..];
		let p = &current_triple.1[..];
		let o = &current_triple.2[..];

		let key_s = grand_ole_pry.get_or_intern(s);
		let key_p = grand_ole_pry.get_or_intern(p);
		let key_o = grand_ole_pry.get_or_intern(o);

		let key_s_int = key_s.into_usize();
		let key_p_int = key_p.into_usize();
		let key_o_int = key_o.into_usize();

		writeln!(&mut abox_part, "{:?} {:?} {:?}", key_s_int, key_p_int, key_o_int);
	    }
	    
	}
	
    }
    
}
