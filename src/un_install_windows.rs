#[cfg(target_os = "windows")]
use std::process::Command;

#[cfg(target_os = "windows")]
pub fn uninstall_windows_app(app_name: &str) -> Result<(), String> {
    let ps_command = format!(
        r#"
        $app = Get-ChildItem -Path "HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall" |
               Get-ItemProperty |
               Where-Object {{ $_.DisplayName -like "*{}*" }} |
               Select-Object -First 1;

        if ($app) {{
            $app.UninstallString
        }} else {{
            Write-Error "App not found.";
        }}
        "#,
        app_name
    );

    match Command::new("powershell")
        .arg("-Command")
        .arg(ps_command)
        .output()
    {
        Ok(output) => match output.status.success() {
            true => {
                let uninstall_string = String::from_utf8_lossy(&output.stdout).trim().to_string();

                match uninstall_string.is_empty() {
                    true => Err(format!("Uninstall string not found for '{}'.", app_name)),
                    false => {
                        println!("Uninstall string found: {}", uninstall_string);

                        let uninstall_command = uninstall_string.trim_matches('"').to_string();

                        match Command::new(uninstall_command).output() {
                            Ok(result) => match result.status.success() {
                                true => Ok(()),
                                false => Err(format!(
                                    "Failed to uninstall '{}': {}",
                                    app_name,
                                    String::from_utf8_lossy(&result.stderr)
                                )),
                            },
                            Err(e) => Err(format!("{:?}", e)),
                        }
                    }
                }
            }
            false => Err(format!(
                "Failed to find uninstall string: {}",
                String::from_utf8_lossy(&output.stderr)
            )),
        },
        Err(e) => Err(format!("{:?}", e)),
    }
}
