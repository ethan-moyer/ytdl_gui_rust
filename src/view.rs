use druid::{Widget, widget::{Label, Flex, TextBox, Button, RadioGroup, Checkbox, ProgressBar, RawLabel}, WidgetExt, MenuDesc, platform_menus, LocalizedString};

use crate::data::{AppState, QualityOptions, VideoFormats, AudioFormats};

#[allow(unused_mut)]
pub fn build_menu() -> MenuDesc<AppState> {
    let mut menu = MenuDesc::empty();

    #[cfg(target_os = "windows")]
    {
        let file_menu = MenuDesc::new(LocalizedString::new("File")).append(platform_menus::win::file::exit());
        menu = menu.append(file_menu);
    }

    #[cfg(target_os = "macos")] {
        let file_menu = MenuDesc::new(LocalizedString::new("Application")).append(platform_menus::mac::application::quit());
        menu = menu.append(file_menu);
    }

    let edit_menu = MenuDesc::new(LocalizedString::new("Edit"))
        .append(platform_menus::common::cut())
        .append(platform_menus::common::copy())
        .append(platform_menus::common::paste());
    menu = menu.append(edit_menu);

    menu
}

pub fn build_ui() -> impl Widget<AppState> {
    let label: Label<AppState> = Label::new("YouTube Downloader").with_text_size(45.0);
    
    let url_textbox = TextBox::new().with_placeholder("Enter video url").expand_width().lens(AppState::video_url);
    let download_button = Button::new("Download").on_click(AppState::click_download);
    //let stream_button = Button::new("Stream").on_click(AppState::click_stream);
    let row2 = Flex::row().with_flex_child(url_textbox, 1.0).with_child(download_button)/*.with_child(stream_button)*/;

    let quality_label: Label<AppState> = Label::new("Preferred quality:");
    let quality_radio_group = RadioGroup::new(vec![
        ("144p", QualityOptions::Q144),
        ("240p", QualityOptions::Q240),
        ("360p", QualityOptions::Q360),
        ("480p", QualityOptions::Q480),
        ("720p", QualityOptions::Q720),
        ("1080p", QualityOptions::Q1080),
        ("1440p", QualityOptions::Q1440),
        ("2160p", QualityOptions::Q2160),
    ]).lens(AppState::preferred_quality);
    let row3_left = Flex::column().with_child(quality_label).with_child(quality_radio_group);
    
    let video_format_label: Label<AppState> = Label::new("Video Format:");
    let video_format_radio_group = RadioGroup::new(vec![
        ("mp4", VideoFormats::mp4),
        ("mkv", VideoFormats::mkv),
        ("mov", VideoFormats::mov),
    ]).lens(AppState::video_format);

    let audio_format_label: Label<AppState> = Label::new("Audio Format: ");
    let audio_format_radio_group = RadioGroup::new(vec![
        ("mp3", AudioFormats::mp3),
        ("wav", AudioFormats::wav),
        ("ogg", AudioFormats::ogg),
    ]).lens(AppState::audio_format);

    let row3_middle = Flex::column()
        .with_child(video_format_label)
        .with_child(video_format_radio_group)
        .with_child(audio_format_label)
        .with_child(audio_format_radio_group);

    let audio_only_check = Checkbox::new("Audio only").lens(AppState::audio_only);
    let row3 = Flex::row().with_child(row3_left).with_child(row3_middle).with_child(audio_only_check);
    
    //let progress_bar = ProgressBar::new().lens(AppState::progress).expand_width();
    let output_label = RawLabel::new().lens(AppState::output).align_left();
    let row4 = Flex::column().with_child(output_label);

    Flex::column().with_child(label).with_child(row2).with_child(row3).with_child(row4)
}