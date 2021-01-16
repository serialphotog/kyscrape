extern crate crossbeam;
extern crate clap;
extern crate select;
extern crate reqwest;

mod gpx;
mod landform;
mod net;

use crossbeam::thread;
use std::path::Path;
use std::process;

use clap::{Arg, App};

// Base URLs for the Kentucky databases
const WATERFALL_BASE: &str = "http://kywaterfalls.com";
const ARCH_BASE: &str = "http://kyarches.com";

// Output file names
const ARCH_FILE: &str = "kentucky_arch_database.gpx";
const WATERFALL_FILE: &str = "kentucky_waterfall_database.gpx";

fn main() {
    let args = App::new("kyscrape")
        .version("0.1.0")
        .about("A simply utility for scraping the Kentucky Landforms databases")
        .author("Adam Thompson <adam@serialphotog.com>")
        .args(&[
            Arg::new("waterfalls")
                .about("Download the waterfall dataset")
                .short('w')
                .long("waterfalls"),
            Arg::new("arches")
                .about("Download the arch dataset")
                .short('a')
                .long("arches"),
            Arg::new("downloadPath")
                .about("Path to the folder to download the data to.")
                .short('p')
                .long("path")
                .takes_value(true)
        ]).get_matches();

    // Handle the output path
    let download_path = Path::new(args.value_of("downloadPath").unwrap_or("./"));
    if !download_path.is_dir() {
        eprintln!("[ERROR]: The path {} does not exist! Exiting...", download_path.display());
        process::exit(1);
    } 
    println!("Setting the output path to: {}", download_path.display());

    // Run the jobs
    if args.is_present("waterfalls") && args.is_present("arches") {
        thread::scope(|s| {
            s.spawn(move |_| {
                download_waterfalls(download_path);
            });
            s.spawn(move |_| {
                download_arches(download_path);
            });
        }).unwrap();
    } else {
        if args.is_present("arches") {
            download_arches(download_path);
        } else {
            download_waterfalls(download_path);
        }
    }
}

/// Downloads the arch dataset from the Kentucky Arches Database
fn download_arches(path: &Path) {
    println!("Downloading the arch dataset!");
    let mut pages = Vec::new();
    net::get_database_pages(ARCH_BASE, &mut pages, "Types".to_owned(), None);

    // Build the list of arch entry links
    let mut arch_pages = Vec::new();
    for page in pages {
        // Build the URL
        let url = net::build_url(ARCH_BASE, page); 
        net::get_landforms(&url, &mut arch_pages);    
    }

    let mut arches = Vec::new();    
    for arch_page in arch_pages {
        let arch_url = net::build_url(ARCH_BASE, arch_page);
        let arch = net::download_landform(arch_url);
        arches.push(arch);
    }

    // Write out the GPX
    let out_path = path.join(ARCH_FILE);
    match gpx::write_gpx(out_path.as_path(), &arches) {
        Err(err) => eprintln!("[ERROR]: Failed to write GPX! \n {}", err),
        _ => (),
    }
}

/// Downloads the waterfall dataset from the Kentucky Waterfall Database
fn download_waterfalls(path: &Path) {
    println!("Downloading the waterfall dataset!");
    let mut pages = Vec::new();
    let exclusions = vec!["/dir/index.php/geography-of-kentucky".to_owned(), "/dir/index.php/busted-destroyed".to_owned()];
    net::get_database_pages(WATERFALL_BASE, &mut pages, "Geographic Regions".to_owned(), Some(&exclusions));

    let mut waterfall_pages = Vec::new();
    for page in pages {
        let url = net::build_url(WATERFALL_BASE, page);
        net::get_landforms(&url, &mut waterfall_pages);
    }

    let mut waterfalls = Vec::new();
    for waterfall_page in waterfall_pages {
        let waterfall_url = net::build_url(WATERFALL_BASE, waterfall_page);
        let waterfall = net::download_landform(waterfall_url);
        waterfalls.push(waterfall);
    }

    // Write out the GPX
    let out_path = path.join(WATERFALL_FILE);
    match gpx::write_gpx(out_path.as_path(), &waterfalls) {
        Err(err) => eprintln!("[ERROR]: Failed to write GPX! \n {}", err),
        _ => (),
    }
}
