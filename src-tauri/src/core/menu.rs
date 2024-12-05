use std::io;
use tauri::menu::{AboutMetadata, Menu, MenuBuilder, SubmenuBuilder};
use tauri::{AppHandle, Runtime};

pub fn build<R: Runtime>(handle: &AppHandle<R>) -> Result<Menu<R>, io::Error> {
    #[cfg(target_os = "macos")]
    let app_name = handle.config().product_name.clone().expect("Producer name is required");

    #[cfg(target_os = "macos")]
    let app_submenu = SubmenuBuilder::new(handle, app_name)
        .about(Some(AboutMetadata::default()))
        .separator()
        .services()
        .separator()
        .hide()
        .hide_others()
        .show_all()
        .separator()
        .quit()
        .build()
        .expect("Menu builder failed");

    Ok(MenuBuilder::new(handle)
        .items(
            #[cfg(target_os = "macos")]
            &[&app_submenu],
        )
        .build()
        .expect("Final menu building failed"))
}
