use clap::Parser;
use colored::*;
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use sysinfo::{System, SystemExt, DiskExt};
use num_cpus;
use std::sync::Mutex;

const BUFFER_SIZE: usize = 8 * 1024 * 1024; // 8 MB

#[derive(Parser, Clone)]
#[command(author = "Nugget (https://github.com/LazyNugget)", version = "0.0.1-alpha", about = "A fast and efficient file copying tool", long_about = None)]
struct Cli {
    #[arg(help = "Source file or directory")]
    source_file: String,

    #[arg(help = "Destination file or directory")]
    destination_file: String,

    #[arg(short, long, help = "Number of threads for file copying (0 for automatic)")]
    threads: Option<usize>,

    #[arg(short, long, help = "Archive mode (preserve attributes, copy recursively)")]
    archive: bool,

    #[arg(short, long, help = "Make a backup before overwriting")]
    backup: bool,

    #[arg(short = 'd', long, help = "Preserve links")]
    no_dereference: bool,

    #[arg(short, long, help = "Force overwrite without prompting")]
    force: bool,

    #[arg(short, long, help = "Interactive mode, prompt before overwrite")]
    interactive: bool,

    #[arg(short, long, help = "Create hard links instead of copying")]
    link: bool,

    #[arg(short = 'n', long, help = "Do not overwrite existing files")]
    no_clobber: bool,

    #[arg(short = 'P', long, help = "Do not follow symlinks")]
    no_dereference_symlinks: bool,

    #[arg(short, long, help = "Preserve file attributes")]
    preserve: bool,

    #[arg(short, long, help = "Copy directories recursively")]
    recursive: bool,

    #[arg(short, long, help = "Create symbolic links instead of copying")]
    symbolic_link: bool,

    #[arg(short, long, help = "Update, copy only when source is newer")]
    update: bool,

    #[arg(short, long, help = "Verbose mode, show what's being done")]
    verbose: bool,

    #[arg(short = 'x', long, help = "Don't cross filesystem boundaries")]
    one_file_system: bool,

    #[arg(long, help = "Enable debug mode for detailed error information")]
    debug: bool,
}

#[derive(Debug, PartialEq)]
enum StorageType {
    SSD,
    HDD,
    Removable,
    Unknown,
}

fn detect_storage_type(path: &Path) -> io::Result<StorageType> {
    let mut system = System::new_all();
    system.refresh_disks_list();

    let canonicalized_path = path.canonicalize()?;
    let path_str = canonicalized_path.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Invalid path")
    })?;

    for disk in system.disks() {
        if path_str.starts_with(disk.mount_point().to_str().unwrap_or("")) {
            if disk.is_removable() {
                return Ok(StorageType::Removable);
            }
            return Ok(match disk.kind() {
                sysinfo::DiskKind::SSD => StorageType::SSD,
                sysinfo::DiskKind::HDD => StorageType::HDD,
                _ => StorageType::Unknown,
            });
        }
    }

    Ok(StorageType::Unknown)
}

fn determine_optimal_threads(source_type: &StorageType, dest_type: &StorageType, user_threads: Option<usize>) -> usize {
    if let Some(threads) = user_threads {
        return threads;
    }

    let cpus = num_cpus::get();
    match (source_type, dest_type) {
        (StorageType::SSD, StorageType::SSD) => cpus * 2,
        (StorageType::SSD, _) | (_, StorageType::SSD) => cpus,
        (StorageType::HDD, StorageType::HDD) => cpus / 2,
        _ => cpus,
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let verbose = Arc::new(AtomicBool::new(cli.verbose));
    let debug = cli.debug;

    let result = std::panic::catch_unwind(|| {
        let source_path = Path::new(&cli.source_file);
        let destination_path = Path::new(&cli.destination_file);
        
        let source_storage_type = detect_storage_type(source_path)?;
        let destination_storage_type = detect_storage_type(destination_path)?;

        println!("{}", format!("Source storage type: {:?}", source_storage_type).blue());
        println!("{}", format!("Destination storage type: {:?}", destination_storage_type).blue());

        let num_threads = determine_optimal_threads(&source_storage_type, &destination_storage_type, cli.threads);
        println!("{}", format!("Using {} threads for copying", num_threads).blue());

        copy(&cli.source_file, &cli.destination_file, &cli, verbose, num_threads)
    });

    match result {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{}", "An unexpected error occurred:".red());
            if debug {
                eprintln!("{:#?}", e);
            } else {
                eprintln!("Run with --debug for more information.");
            }
            eprintln!("{}", "If you believe this is a bug in the tool, please report it at:".yellow());
            eprintln!("https://github.com/GoldenHand-Dev/rc/issues");
            eprintln!("Or contact the developer at: 1nu55et1@gmail.com");
            Err(io::Error::new(io::ErrorKind::Other, "Unexpected error"))
        }
    }
}

fn copy(source: &str, destination: &str, cli: &Cli, verbose: Arc<AtomicBool>, num_threads: usize) -> io::Result<()> {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    if source_path.is_dir() {
        if !cli.recursive {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Source is a directory. Use -r option to copy recursively."));
        }
        copy_directory(source_path, destination_path, cli, verbose, num_threads)
    } else {
        copy_file(source_path, destination_path, cli, verbose)
    }
}

fn copy_directory(source: &Path, destination: &Path, cli: &Cli, verbose: Arc<AtomicBool>, num_threads: usize) -> io::Result<()> {
    if !destination.exists() {
        fs::create_dir_all(destination).map_err(|e| {
            if cli.debug {
                eprintln!("Debug info: Failed to create destination directory: {:#?}", e);
            }
            e
        })?;
    }

    let (tx, rx) = mpsc::channel::<(PathBuf, PathBuf)>();
    let rx = Arc::new(Mutex::new(rx));
    let pending_files = Arc::new(AtomicUsize::new(0));

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();

    let worker_threads: Vec<_> = (0..num_threads).map(|_| {
        let rx = Arc::clone(&rx);
        let cli = cli.clone();
        let verbose = Arc::clone(&verbose);
        let pending_files = Arc::clone(&pending_files);
        thread::spawn(move || {
            while let Ok((source, destination)) = rx.lock().unwrap().recv() {
                if let Err(e) = copy_file(source.as_path(), destination.as_path(), &cli, Arc::clone(&verbose)) {
                    eprintln!("Error copying {}: {}", source.display(), e);
                }
                pending_files.fetch_sub(1, Ordering::SeqCst);
            }
        })
    }).collect();

    pool.install(|| {
        let entries: Vec<_> = fs::read_dir(source).map_err(|e| {
            if cli.debug {
                eprintln!("Debug info: Failed to read source directory: {:#?}", e);
            }
            e
        })?.collect::<Result<_, _>>()?;
        
        entries.par_iter().try_for_each(|entry| {
            let file_type = entry.file_type()?;
            let new_destination = destination.join(entry.file_name());

            if file_type.is_dir() {
                copy_directory(&entry.path(), &new_destination, cli, Arc::clone(&verbose), num_threads)
            } else {
                pending_files.fetch_add(1, Ordering::SeqCst);
                tx.send((entry.path().to_path_buf(), new_destination.to_path_buf())).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
                Ok(())
            }
        })
    })?;

    drop(tx);

    // Wait for all files to be copied
    while pending_files.load(Ordering::SeqCst) > 0 {
        thread::sleep(std::time::Duration::from_millis(100));
    }

    for thread in worker_threads {
        thread.join().map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;
    }

    Ok(())
}

fn copy_file(source: &Path, destination: &Path, cli: &Cli, verbose: Arc<AtomicBool>) -> io::Result<()> {
    if destination.exists() && !cli.force {
        if cli.interactive {
            print!("File {} already exists. Overwrite? (y/n): ", destination.display());
            io::stdout().flush()?;
            let mut response = String::new();
            io::stdin().read_line(&mut response)?;
            if response.trim().to_lowercase() != "y" {
                println!("{}", format!("Skipping file {}", destination.display()).yellow());
                return Ok(());
            }
        } else if cli.no_clobber {
            println!("{}", format!("Not overwriting existing file {}", destination.display()).yellow());
            return Ok(());
        } else {
            let err = io::Error::new(io::ErrorKind::AlreadyExists, "Destination file already exists. Use --force to overwrite.");
            if cli.debug {
                eprintln!("Debug info: {:#?}", err);
            }
            return Err(err);
        }
    }

    if cli.update && destination.exists() {
        let source_metadata = source.metadata().map_err(|e| {
            if cli.debug {
                eprintln!("Debug info: Failed to get source metadata: {:#?}", e);
            }
            e
        })?;
        let destination_metadata = destination.metadata().map_err(|e| {
            if cli.debug {
                eprintln!("Debug info: Failed to get destination metadata: {:#?}", e);
            }
            e
        })?;
        if source_metadata.modified()? <= destination_metadata.modified()? {
            println!("{}", format!("Not updating file {}", destination.display()).yellow());
            return Ok(());
        }
    }

    let mut source_file = BufReader::with_capacity(BUFFER_SIZE, File::open(source).map_err(|e| {
        if cli.debug {
            eprintln!("Debug info: Failed to open source file: {:#?}", e);
        }
        e
    })?);
    let mut destination_file = BufWriter::with_capacity(BUFFER_SIZE, File::create(destination).map_err(|e| {
        if cli.debug {
            eprintln!("Debug info: Failed to create destination file: {:#?}", e);
            eprintln!("Destination path: {}", destination.display());
            eprintln!("Current user: {:?}", std::env::var("USERNAME"));
            eprintln!("Current directory: {:?}", std::env::current_dir().unwrap());
        }
        e
    })?);

    let mut buffer = vec![0; BUFFER_SIZE];
    loop {
        let bytes_read = source_file.read(&mut buffer).map_err(|e| {
            if cli.debug {
                eprintln!("Debug info: Failed to read from source file: {:#?}", e);
            }
            e
        })?;
        if bytes_read == 0 {
            break;
        }
        destination_file.write_all(&buffer[..bytes_read]).map_err(|e| {
            if cli.debug {
                eprintln!("Debug info: Failed to write to destination file: {:#?}", e);
            }
            e
        })?;
    }
    destination_file.flush().map_err(|e| {
        if cli.debug {
            eprintln!("Debug info: Failed to flush destination file: {:#?}", e);
        }
        e
    })?;

    if verbose.load(Ordering::Relaxed) {
        println!("{}", format!("Copied: {} -> {}", source.display(), destination.display()).green());
    }

    Ok(())
}
