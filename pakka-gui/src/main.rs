#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;

use iced::widget::text_input::cursor::State;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Element, Length, Renderer, Task, Theme};

pub fn main() -> iced::Result {
  #[cfg(all(not(target_arch = "wasm32"), debug_assertions))]
  tracing_subscriber::fmt::init();

  iced::application(Counter::title, Counter::update, Counter::view)
    .window_size((1280.0, 720.0))
    .run_with(Counter::new)
}

#[derive(Debug, Clone)]
pub enum Error {
  DialogClosed,
  IoError(io::ErrorKind),
}

#[derive(Debug, Clone)]
enum Message {
  TextContentChanged(String),
  InstallPackage,
  UninstallPackage,
}

struct Counter {
  package_name: Option<String>,
  output: Option<String>,
}

impl Counter {
  fn new() -> (Self, Task<Message>) {
    (
      Self { package_name: Default::default(), output: Default::default() },
      Task::none(),
    )
  }

  fn title(&self) -> String {
    String::from("Counter")
  }

  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::TextContentChanged(path) => {
        self.package_name = Some(path);
        Task::none()
      }

      Message::InstallPackage => {
        println!(
          "InstallPackage, {}",
          self.package_name.clone().unwrap_or_default()
        );

        self.output = match run_cli_command(
          "install",
          self.package_name.clone().unwrap_or_default(),
        ) {
          Ok(e) => Some(e),
          Err(e) => Some(e),
        };

        Task::none()
      }

      Message::UninstallPackage => {
        println!(
          "UninstallPackage, {}",
          self.package_name.clone().unwrap_or_default()
        );

        self.output = match run_cli_command(
          "uninstall",
          self.package_name.clone().unwrap_or_default(),
        ) {
          Ok(e) => Some(e),
          Err(e) => Some(e),
        };

        Task::none()
      }
    }
  }

  fn view(&self) -> Element<Message> {
    let content = column![
      row![text("package name").size(16)],
      row![text("").size(16)],
      row![text_input(
        "Enter package name here...",
        &self.package_name.clone().unwrap_or_default()
      )
      .on_input(Message::TextContentChanged)],
      row![text("").size(16)],
      row![button("Install").on_press(Message::InstallPackage)],
      row![text("").size(16)],
      row![button("Uninstall").on_press(Message::UninstallPackage)],
      row![text("").size(16)],
      row![text("output").size(16)],
      row![text(self.output.clone().unwrap_or_default()).size(16)],
    ];

    container(content)
      .width(Length::Fill)
      .height(Length::Fill)
      .center_x(Length::Fill)
      .center_y(Length::Fill)
      .padding(20)
      .into()
  }
}

fn run_cli_command(command: &str, package: String) -> Result<String, String> {
  println!("{}, {}", command, package);

  let output = std::process::Command::new("pakka")
    .arg(command)
    .arg(&package)
    .output()
    .map_err(|e| e.to_string())?;

  let result = (String::from_utf8_lossy(&output.stdout)
    + String::from_utf8_lossy(&output.stderr))
  .to_string();

  return Ok(result);

  #[allow(unreachable_code)]
  if output.status.success() {
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
  } else {
    Err(String::from_utf8_lossy(&output.stderr).to_string())
  }
}
