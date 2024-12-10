#[cfg(target_os = "linux")]
use std::error::Error;
#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "linux")]
pub fn uninstall_linux_app(app_name: &str) -> Result<(), String> {
    use crate::installed::get_installed_linux_app;
    let mut result = Ok(());
    match get_installed_linux_app() {
        Ok(v) => {
            for x in v.into_iter() {
                if x.contains(app_name) {
                    let matched_name = x.as_str();
                    println!("Found {} installed.", matched_name);

                    if Command::new("which")
                        .arg("rpm")
                        .output()
                        .map_or(false, |o| o.status.success())
                    {
                        match uninstall_via_dnf(matched_name) {
                            Ok(_) => {
                                result = Ok(());
                                break;
                            }
                            Err(e) => {
                                result = Err(format!(
                                    "Failed to uninstall {}. Error: {:?}",
                                    matched_name, e
                                ));
                            }
                        }
                    } else if Command::new("which")
                        .arg("dpkg-query")
                        .output()
                        .map_or(false, |o| o.status.success())
                    {
                        match uninstall_via_deb(matched_name) {
                            Ok(_) => {
                                result = Ok(());
                                break;
                            }
                            Err(e) => {
                                result = Err(format!(
                                    "Failed to uninstall {}. Error: {:?}",
                                    matched_name, e
                                ));
                            }
                        }
                    }
                    break;
                }
            }
            println!("Result: {:?}", result);
            result
        }
        Err(e) => Err(format!("ERR::{:?}", e)),
    }
}

#[cfg(target_os = "linux")]
fn uninstall_via_dnf(app_name: &str) -> Result<(), Box<dyn Error>> {
    // Run dnf command to remove the app
    let output = Command::new("pkexec")
        .arg("sudo")
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
fn uninstall_via_deb(app_name: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("pkexec")
        .arg("apt-get")
        .arg("remove")
        .arg("-y")
        .arg(app_name)
        .output()?;

    if output.status.success() {
        println!("Successfully uninstalled {} via DEB.", app_name);
        Ok(())
    } else {
        Err(format!("Failed to uninstall {} via DEB", app_name).into())
    }
}

// #[cfg(target_os = "linux")]
// fn uninstall_via_flatpak(app_name: &str) -> Result<(), Box<dyn Error>> {
//     // Run flatpak command to remove the app
//     let output = Command::new("pkexec")
//         .arg("flatpak")
//         //    new("flatpak")
//         .arg("uninstall")
//         .arg(app_name)
//         .arg("-y")
//         .output()?;

//     if output.status.success() {
//         println!("Successfully uninstalled {} via Flatpak.", app_name);
//         Ok(())
//     } else {
//         Err(format!("Failed to uninstall {} via Flatpak", app_name).into())
//     }
// }
