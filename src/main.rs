use clap::{Arg, Command};
use lasso::{Key, Rodeo};
use linecount::count_lines;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn read_file(filename: &str) -> impl Iterator<Item = String> {
    let file = BufReader::new(File::open(filename).unwrap());
    file.lines().filter_map(|line| line.ok())
}

pub fn load3nt<'a>(filename: &str) -> impl Iterator<Item = (String, String, String)> + 'a {
    read_file(filename).map(move |line| {
        let mut line_clean = line;

        // Removing the .
        assert!(line_clean.pop(), ".");
        assert!(line_clean.pop(), "");

        let mut elts = line_clean.split(" ");

        (
            elts.next().unwrap().parse().unwrap(),
            elts.next().unwrap().parse().unwrap(),
            elts.next().unwrap().parse().unwrap(),
        )
    })
}

fn main() {
    let matches = Command::new(".nt format encoder")
        .version("0.1.0")
        .author("Rucy")
        .about("Encodes .nt rdf a and t boxes according to a very specific scheme")
        .arg(
            Arg::new("TBOX_PATH")
                .help("Sets the tbox file path")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("ABOX_PATH")
                .help("Sets the abox file path")
                .required(true)
                .index(2),
        )
        .get_matches();

    let t_path = matches.value_of("TBOX_PATH").unwrap();
    let a_path = matches.value_of("ABOX_PATH").unwrap();
    let wc_tbox: i64 = count_lines(File::open(t_path).unwrap()).unwrap() as i64;
    let wc_abox: i64 = count_lines(File::open(a_path).unwrap()).unwrap() as i64;

    println!("Abox size: {:?} triples", wc_abox);
    println!("Tbox size: {:?} triples", wc_tbox);

    let mut encoded_tbox_file = File::create("tbox.ntenc").unwrap();
    let mut encoded_abox_file = File::create("abox.ntenc").unwrap();
    let mut grand_ole_pry = Rodeo::default();

    let rdfsco: &str = "<http://www.w3.org/2000/01/rdf-schema#subClassOf>";
    let rdfspo: &str = "<http://www.w3.org/2000/01/rdf-schema#subPropertyOf>";
    let rdfsd: &str = "<http://www.w3.org/2000/01/rdf-schema#domain>";
    let rdfsr: &str = "<http://www.w3.org/2000/01/rdf-schema#range>";
    let rdft: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>";
    let rdfcomment: &str = "<http://www.w3.org/2000/01/rdf-schema#comment>";
    let rdfrest: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#rest>";
    let rdffirst: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#first>";
    let rdflab: &str = "<http://www.w3.org/2000/01/rdf-schema#label>";
    let rdfn: &str = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#nil>";
    let rdfl: &str = "<http://www.w3.org/2000/01/rdf-schema#Literal>";
    let owltr: &str = "<http://www.w3.org/2002/07/owl#TransitiveProperty>";
    let owlio: &str = "<http://www.w3.org/2002/07/owl#inverseOf>";
    let owlthing: &str = "<http://www.w3.org/2002/07/owl#Thing>";
    let owlmqc: &str = "<http://www.w3.org/2002/07/owl#maxQualifiedCardinality>";
    let owlsvf: &str = "<http://www.w3.org/2002/07/owl#someValuesFrom>";
    let owlec: &str = "<http://www.w3.org/2002/07/owl#equivalentClass>";
    let owlito: &str = "<http://www.w3.org/2002/07/owl#intersectionOf>";
    let owlm: &str = "<http://www.w3.org/2002/07/owl#members>";
    let owlep: &str = "<http://www.w3.org/2002/07/owl#equivalentProperty>";
    let owlopr: &str = "<http://www.w3.org/2002/07/owl#onProperty>";
    let owlpca: &str = "<http://www.w3.org/2002/07/owl#propertyChainAxiom>";
    let owldw: &str = "<http://www.w3.org/2002/07/owl#disjointWith>";
    let owlpdw: &str = "<http://www.w3.org/2002/07/owl#propertyDisjointWith>";
    let owluo: &str = "<http://www.w3.org/2002/07/owl#unionOf>";
    let owlhk: &str = "<http://www.w3.org/2002/07/owl#hasKey>";
    let owlavf: &str = "<http://www.w3.org/2002/07/owl#allValuesFrom>";
    let owlco: &str = "<http://www.w3.org/2002/07/owl#complementOf>";
    let owloc: &str = "<http://www.w3.org/2002/07/owl#onClass>";
    let owldm: &str = "<http://www.w3.org/2002/07/owl#distinctMembers>";
    let owlfp: &str = "<http://www.w3.org/2002/07/owl#FunctionalProperty>";
    let owlni: &str = "<http://www.w3.org/2002/07/owl#NamedIndividual>";
    let owlop: &str = "<http://www.w3.org/2002/07/owl#ObjectProperty>";
    let owlc: &str = "<http://www.w3.org/2002/07/owl#Class>";
    let owladc: &str = "<http://www.w3.org/2002/07/owl#AllDisjointClasses>";
    let owlr: &str = "<http://www.w3.org/2002/07/owl#Restriction>";
    let owldp: &str = "<http://www.w3.org/2002/07/owl#DatatypeProperty>";
    let owlo: &str = "<http://www.w3.org/2002/07/owl#Ontology>";
    let owlap: &str = "<http://www.w3.org/2002/07/owl#AsymmetricProperty>";
    let owlsp: &str = "<http://www.w3.org/2002/07/owl#SymmetricProperty>";
    let owlip: &str = "<http://www.w3.org/2002/07/owl#IrreflexiveProperty>";
    let owlad: &str = "<http://www.w3.org/2002/07/owl#AllDifferent>";
    let owlifp: &str = "<http://www.w3.org/2002/07/owl#InverseFunctionalProperty>";
    let owlsa: &str = "<http://www.w3.org/2002/07/owl#sameAs>";
    let owlhv: &str = "<http://www.w3.org/2002/07/owl#hasValue>";
    let owlnt: &str = "<http://www.w3.org/2002/07/owl#Nothing>";
    let owloo: &str = "<http://www.w3.org/2002/07/owl#oneOf>";

    grand_ole_pry.get_or_intern(rdfsco);
    grand_ole_pry.get_or_intern(rdfspo);
    grand_ole_pry.get_or_intern(rdfsd);
    grand_ole_pry.get_or_intern(rdfsr);
    grand_ole_pry.get_or_intern(rdft);
    grand_ole_pry.get_or_intern(rdfcomment);
    grand_ole_pry.get_or_intern(rdfrest);
    grand_ole_pry.get_or_intern(rdffirst);
    grand_ole_pry.get_or_intern(rdflab);
    grand_ole_pry.get_or_intern(rdfn);
    grand_ole_pry.get_or_intern(rdfl);
    grand_ole_pry.get_or_intern(owltr);
    grand_ole_pry.get_or_intern(owlio);
    grand_ole_pry.get_or_intern(owlthing);
    grand_ole_pry.get_or_intern(owlmqc);
    grand_ole_pry.get_or_intern(owlsvf);
    grand_ole_pry.get_or_intern(owlec);
    grand_ole_pry.get_or_intern(owlito);
    grand_ole_pry.get_or_intern(owlm);
    grand_ole_pry.get_or_intern(owlep);
    grand_ole_pry.get_or_intern(owlopr);
    grand_ole_pry.get_or_intern(owlpca);
    grand_ole_pry.get_or_intern(owldw);
    grand_ole_pry.get_or_intern(owlpdw);
    grand_ole_pry.get_or_intern(owluo);
    grand_ole_pry.get_or_intern(owlhk);
    grand_ole_pry.get_or_intern(owlavf);
    grand_ole_pry.get_or_intern(owlco);
    grand_ole_pry.get_or_intern(owloc);
    grand_ole_pry.get_or_intern(owldm);
    grand_ole_pry.get_or_intern(owlfp);
    grand_ole_pry.get_or_intern(owlni);
    grand_ole_pry.get_or_intern(owlop);
    grand_ole_pry.get_or_intern(owlc);
    grand_ole_pry.get_or_intern(owladc);
    grand_ole_pry.get_or_intern(owlr);
    grand_ole_pry.get_or_intern(owldp);
    grand_ole_pry.get_or_intern(owlo);
    grand_ole_pry.get_or_intern(owlap);
    grand_ole_pry.get_or_intern(owlsp);
    grand_ole_pry.get_or_intern(owlip);
    grand_ole_pry.get_or_intern(owlad);
    grand_ole_pry.get_or_intern(owlifp);
    grand_ole_pry.get_or_intern(owlsa);
    grand_ole_pry.get_or_intern(owlhv);
    grand_ole_pry.get_or_intern(owlnt);
    grand_ole_pry.get_or_intern(owloo);

    let mut lines = 0;
    let raw_tbox = load3nt(t_path);
    raw_tbox.for_each(|triple| {
        lines += 1;

        let s = &triple.0[..];
        let p = &triple.1[..];
        let o = &triple.2[..];

        let key_s = grand_ole_pry.get_or_intern(s);
        let key_p = grand_ole_pry.get_or_intern(p);
        let key_o = grand_ole_pry.get_or_intern(o);

        let key_s_int = key_s.into_usize();
        let key_p_int = key_p.into_usize();
        let key_o_int = key_o.into_usize();

        writeln!(
            &mut encoded_tbox_file,
            "{:?} {:?} {:?}",
            key_s_int, key_p_int, key_o_int
        );
    });

    let mut raw_abox = load3nt(a_path);
    raw_abox.for_each(|triple| {
        let s = &triple.0[..];
        let p = &triple.1[..];
        let o = &triple.2[..];

        let key_s = grand_ole_pry.get_or_intern(s);
        let key_p = grand_ole_pry.get_or_intern(p);
        let key_o = grand_ole_pry.get_or_intern(o);

        let key_s_int = key_s.into_usize();
        let key_p_int = key_p.into_usize();
        let key_o_int = key_o.into_usize();

        writeln!(
            &mut encoded_abox_file,
            "{:?} {:?} {:?}",
            key_s_int, key_p_int, key_o_int
        );
    });
}
