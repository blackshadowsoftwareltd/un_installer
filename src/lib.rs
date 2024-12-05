pub mod installed;
pub mod un_install;

#[cfg(target_os = "windows")]
use un_install::uninstall_windows_app;

#[cfg(test)]
mod tests {
    use crate::un_install_desktop_app;

    #[test]
    fn test_uninstall() {
        let app_name = "F Sync version 1.0.0+3";
        let r = un_install_desktop_app(&app_name);
        assert_eq!(r.unwrap(), format!("{} Uninstalled Successfully", app_name));
    }
}

pub fn un_install_desktop_app(_app_name: &str) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    match uninstall_windows_app(&_app_name) {
        Ok(_) => Ok(format!("{} Uninstalled Successfully", _app_name)),
        Err(e) => Err(format!("{:?}", e)),
    }

    #[cfg(not(target_os = "windows"))]
    Err("This platform is not supported.".to_string())
}