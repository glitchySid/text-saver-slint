use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use slint::SharedString;
slint::include_modules!();

#[allow(unused_variables)]
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    #[cfg(debug_assertions)]
    {
        let separator = SharedString::from("\n");
        let ui_handle = ui.as_weak();
        ui.on_save_data(move |data| {
            let ui = ui_handle.upgrade().expect("UI handle was dropped");
            let u_separator = data +  separator.clone().as_str();
            let mut file = OpenOptions::new()
                .append(true)
                .open("data.txt")
                .expect("Failed to open file");
            file.write(u_separator.as_bytes())
                .expect("Failed to write to file");
            let mut all_data = File::open("data.txt").unwrap();
            let mut file_content = String::new();
            all_data.read_to_string(&mut file_content).unwrap();
            let every_data = SharedString::from(file_content);
            ui.set_result(every_data.into());
        });

        ui.run()
    }

    // #[cfg(not(debug_assertions))]
    // {
    //     ui.on_save_data(|data| {
    //         let mut file = OpenOptions::new()
    //             .write(true)
    //             .create(true)
    //             .truncate(true)
    //             .open("data.json")
    //             .expect("Failed to open file");
    //         file.write_all(data.as_bytes())
    //             .expect("Failed to write to file");
    //     });

    //     ui.run()
    // 
}
