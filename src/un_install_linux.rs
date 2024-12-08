#[cfg(target_os = "linux")]
use std::error::Error;
#[cfg(target_os = "linux")]
use std::process::Command;
#[cfg(target_os = "linux")]
use std::process::{exit, Command};

#[cfg(target_os = "linux")]
fn uninstall_linux_app(app_name: &str) -> Result<(), String> {
    // First, try to uninstall using DNF
    if let Err(e) = uninstall_via_dnf(app_name) {
        eprintln!("{}", e);
        println!("Trying to uninstall using Flatpak...");

        // If DNF fails, try uninstalling with Flatpak (if applicable)
        if let Err(e) = uninstall_via_flatpak(app_name) {
            eprintln!("{}", e);
            eprintln!("Failed to uninstall {}. Please check manually.", app_name);
            exit(1); // Exit with an error status
        }
    }

    println!("Uninstallation process completed.");
    Ok(())
}

#[cfg(target_os = "linux")]
fn uninstall_via_dnf(app_name: &str) -> Result<(), Box<dyn Error>> {
    // Run dnf command to remove the app
    let output = Command::new("sudo")
        .arg("dnf")
        .arg("remove")
        .arg("-y")
        .arg(app_name)
        .output()?;

    if output.status.success() {
        println!("Successfully uninstalled {} via DNF.", app_name);
        Ok(())
    } else {
        Err(format!("Failed to uninstall {} via DNF", app_name).into())
    }
}

#[cfg(target_os = "linux")]
fn uninstall_via_flatpak(app_name: &str) -> Result<(), Box<dyn Error>> {
    // Run flatpak command to remove the app
    let output = Command::new("flatpak")
        .arg("uninstall")
        .arg(app_name)
        .arg("-y")
        .output()?;

    if output.status.success() {
        println!("Successfully uninstalled {} via Flatpak.", app_name);
        Ok(())
    } else {
        Err(format!("Failed to uninstall {} via Flatpak", app_name).into())
    }
}
