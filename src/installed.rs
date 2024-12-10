#[cfg(target_os = "linux")]
use std::process::Command;
#[cfg(target_os = "windows")]
use std::process::Command;
#[cfg(target_os = "windows")]
pub fn list_installed_windows_apps() -> Result<String, String> {
    // Define the PowerShell command to list installed apps from the Registry
    let ps_command = r#"
    Get-ChildItem -Path "HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall" |
    Get-ItemProperty | Select-Object DisplayName, DisplayVersion |
    Where-Object { $_.DisplayName -ne $null } |
    Format-Table -AutoSize
    "#;

    // Run the PowerShell command
    match Command::new("powershell")
        .arg("-Command")
        .arg(ps_command)
        .output()
    {
        Ok(output) => match output.status.success() {
            true => Ok(format!("{:?}", String::from_utf8_lossy(&output.stdout))),
            false => Err(format!(
                "Error executing PowerShell: {}",
                String::from_utf8_lossy(&output.stderr)
            )),
        },
        Err(e) => Err(format!("Failed to execute PowerShell command ({:?})", e)),
    }
}
#[cfg(target_os = "linux")]
pub fn get_installed_linux_app() -> Result<Vec<String>, String> {
    match detect_package_manager() {
        Some("rpm") => {
            // Use `rpm -qa` to list all installed RPM packages
            let output = Command::new("rpm")
                .arg("-qa")
                .output()
                .map_err(|e| format!("Failed to execute rpm: {:?}", e))?;

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let packages = stdout.lines().map(|line| line.to_string()).collect();
                Ok(packages)
            } else {
                Err(format!(
                    "Failed to list RPM packages: {}",
                    String::from_utf8_lossy(&output.stderr)
                ))
            }
        }
        Some("dpkg") => {
            // Use `dpkg-query -W -f='${binary:Package}\n'` to list all installed DEB packages
            let output = Command::new("dpkg-query")
                .arg("-W")
                .arg("-f=${binary:Package}\n")
                .output()
                .map_err(|e| format!("Failed to execute dpkg-query: {:?}", e))?;

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let packages = stdout.lines().map(|line| line.to_string()).collect();
                Ok(packages)
            } else {
                Err(format!(
                    "Failed to list DEB packages: {}",
                    String::from_utf8_lossy(&output.stderr)
                ))
            }
        }
        _ => Err("No supported package manager found.".to_string()),
    }
}

#[cfg(target_os = "linux")]
fn detect_package_manager() -> Option<&'static str> {
    if Command::new("which")
        .arg("rpm")
        .output()
        .map_or(false, |o| o.status.success())
    {
        Some("rpm")
    } else if Command::new("which")
        .arg("dpkg-query")
        .output()
        .map_or(false, |o| o.status.success())
    {
        Some("dpkg")
    } else {
        None
    }
}
