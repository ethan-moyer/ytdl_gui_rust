use druid::{AppLauncher, WindowDesc};

mod data;
use data::{AppState, Delegate};

mod view;
use view::{build_ui, build_menu};

fn main() {
    let main_window = WindowDesc::new(build_ui).title("YTDL GUI").window_size((500.0, 430.0)).menu(build_menu());

    let initial_state = AppState::new();
    
    AppLauncher::with_window(main_window).delegate(Delegate {}).launch(initial_state).expect("Failed to launch application");
}
