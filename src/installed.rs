use std::process::Command;

pub fn list_installed_apps() -> Result<String, String> {
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
