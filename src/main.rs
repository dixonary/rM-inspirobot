use std::fs;
use std::fs::{write, create_dir_all, remove_file};
use std::env;
use std::process::Command;
use std::time::Duration;

// http requests and retry
use reqwest;
use backoff::Operation;
use backoff::ExponentialBackoff;

// Interfacing with dbus
use dbus::blocking::Connection;
use dbus::message::SignalArgs;

// Loading in data from an adjacent file requires two statements
mod oxide_wifi_dbus;
use oxide_wifi_dbus::CodesEeemsOxide1WifiNetworkConnected;

const API_ENDPOINT:&str = "https://inspirobot.me/api?generate=true";

const SHARE_DIR:&str = "/opt/usr/share/inspirobot";
const TEMPLATE_SCRIPT:&str = "/opt/usr/share/inspirobot/mktemplate";
const TEMPLATE_IMAGE:&str = "/opt/usr/share/inspirobot/template.bmp";

const CACHE_DIR:&str = "/opt/var/cache/inspirobot";
const CACHED_IMAGE:&str      = "/opt/var/cache/inspirobot/latest-raw.bmp";
const CACHED_SUSPENDSCREEN_BMP:&str = "/opt/var/cache/inspirobot/latest.bmp";
const CACHED_SUSPENDSCREEN:&str = "/opt/var/cache/inspirobot/latest.png";

const RM_SUSPENDSCREEN:&str = "/usr/share/remarkable/suspended.png";
const BACKUP:&str        = "/usr/share/remarkable/suspended.png-backup";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Permit the first argument to be unrecognised, since it's typically the 
    // binary name
    let mut first_arg = true;

    // TODO: Replace with an actual option parser
    for argument in env::args() {
        match argument.as_str() {
            "--help"       => { print_help(); Ok(()) },
            "--backup"     => backup_suspendscreen(),
            "--update"     => update_suspendscreen(),
            "--restore"    => restore_suspendscreen(),
            _ if first_arg => { first_arg = false; continue; },
            _              => { print_help(); Ok(()) }
        }?;
        // If there are any arguments, return early
        return Ok(());
    }

    if !std::path::Path::new(SHARE_DIR).exists() {
        println!("Writing template directory...");
        fs::create_dir_all(SHARE_DIR)?;

        println!("...done.");
        fs::write(TEMPLATE_SCRIPT, include_str!("../data/mktemplate"))?;

        println!("Generating template...");
        Command::new("bash")
            .current_dir(SHARE_DIR)
            .arg(TEMPLATE_SCRIPT)
            .status()?;
        println!("...done.");
    }

    // Backup automatically on first run
    if !backup_exists() {
        backup_suspendscreen()?;
        update_suspendscreen()?;
    }

    run_listener()?;

    return Ok(());
}

fn print_help() {
    println!(
        "{}\n\n{}\n{}\n{}",
        "Usage: inspirobot [--backup | --restore | --update]",
        "--backup \tTake backup of current suspendscreen. (default on first run)",
        "--restore\tRestore backed-up suspendscreen",
        "--update \tGet a new lock screen image"
    );
}

// Run a listener which awaits a signal from the system dbus on the given
// interface, for when the device resules, and updates the saved suspend image
fn run_listener() -> Result<(), Box<dyn std::error::Error>>  {
    let conn = Connection::new_system()?;

    // Listen only for resume signals
    conn.add_match(
        CodesEeemsOxide1WifiNetworkConnected::match_rule(None,None), 
        |_signal : (),_self,_message| {
            let res = update_suspendscreen();
            match res {
                Ok(())     => (),
                Err(error) =>
                    println!("Could not update suspendscreen:\n{:?}", error)
            }
            return true;
        }
    )?;

    loop {
        conn.process(Duration::from_millis(1000)).unwrap();
    }
}

// Check whether some file already exists in the backup location.
fn backup_exists() -> bool {
    return std::path::Path::new(BACKUP).exists();
}

// Store the existing suspendscreen in a safe location and symlink to our own.
fn backup_suspendscreen() -> Result<(), Box<dyn std::error::Error>> {
    if backup_exists() {
        println!("A suspendscreen backup already exists. No action has been taken.");
        return Ok(());
    }
    println!("Backing up existing suspendscreen...");
    fs::rename(RM_SUSPENDSCREEN, BACKUP)?;
    std::os::unix::fs::symlink(CACHED_SUSPENDSCREEN, RM_SUSPENDSCREEN)?;
    println!("...done.");

    return Ok(());
}

// Delete our symlink and return the backed-up file to its rightful place
fn restore_suspendscreen() -> Result<(), Box<dyn std::error::Error>> {
    println!("Restoring suspendscreen backup...");
    // Fail if no backup is found
    if !backup_exists() {
        println!("No backup file currently exists. No action has been taken.");
        return Ok(());
    }
    // Fail if it's not our symlink
    match fs::read_link(RM_SUSPENDSCREEN) {
        Ok(path) if path.to_str().unwrap() == CACHED_SUSPENDSCREEN => (),
        _ => {
            println!("Could not remove inspirobot suspendscreen, because the current suspendscreen was not placed there by inspirobot!\n\nPlease investigate by hand.");
            return Ok(());
        }
    }
    fs::rename(BACKUP, RM_SUSPENDSCREEN)?;
    println!("...done.");

    return Ok(());
}

fn update_suspendscreen() -> Result<(), Box<dyn std::error::Error>> {

    println!("Updating suspendscreen now.");

    let policy = &mut ExponentialBackoff::default();

    // Get image url (with retry)
    let mut get_image_url = || {
        reqwest::blocking::get(API_ENDPOINT)
            .and_then(|resp| resp.text())
            .map_err(|err| backoff::Error::Transient(err.to_string()))
    };
    let log_retry_gen = |_,_| { println!("Retrying generation..."); };
    let image_url = get_image_url
        .retry_notify(policy, log_retry_gen)
        .map_err(|err| err.to_string())?;

    println!("Got image URL: {:?}", image_url);

    // Get image data (with retry)
    let mut get_image_data = || {
        reqwest::blocking::get(&image_url)
            .and_then(|resp| resp.bytes())
            .map_err(|err| {backoff::Error::Transient(err.to_string())})
    };
    let log_retry_data = |_,_| { println!("Retrying data download..."); };
    let image_data = get_image_data
        .retry_notify(policy, log_retry_data)
        .map_err(|err| err.to_string())?;

    println!("Got image data");

    // Make the directory if it does not yet exist
    create_dir_all(CACHE_DIR)?;
    
    // Store the image data
    write(CACHED_IMAGE, image_data)?;

    // Upscale the newly downloaded lockscreen
    Command::new("mogrify")
        .args(&[
            "-resize", "1300x1300",
            CACHED_IMAGE,
        ])
        .status()?;


    // Composite the inspirobot image onto the template
    // We use BMP here because imagemagick can composite about twice as fast
    Command::new("composite")
        .args(&[
            "-gravity", "center",
            CACHED_IMAGE,
            TEMPLATE_IMAGE,
            CACHED_SUSPENDSCREEN_BMP
        ])
        .status()?;

    // Separately convert the bitmap into a PNG.
    // Doing this helps avoid out-of-memory issues on the device
    Command::new("convert")
        .args(&[
            CACHED_SUSPENDSCREEN_BMP,
            CACHED_SUSPENDSCREEN
        ])
        .status()?;

    // Remove temporary bitmap
    remove_file(CACHED_SUSPENDSCREEN_BMP).unwrap_or_default();

    println!("Updated suspendscreen successfully");

    return Ok(());
}