// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Internal runtime between Tauri and the underlying webview runtime.

pub use crate::runtime::manager::Args;

use crate::{
  runtime::{
    webview::AttributesBase,
    window::{DetachedWindow, PendingWindow},
  },
  Attributes, Icon, Params,
};
use std::convert::TryFrom;

pub(crate) mod app;
pub mod flavors;
pub(crate) mod manager;
pub mod tag;
pub mod webview;
pub mod window;

/// The webview runtime interface.
pub trait Runtime: Sized + 'static {
  /// The message dispatcher.
  type Dispatcher: Dispatch<Runtime = Self>;

  /// Creates a new webview runtime.
  fn new() -> crate::Result<Self>;

  /// Create a new webview window.
  fn create_window<P: Params<Runtime = Self>>(
    &mut self,
    pending: PendingWindow<P>,
  ) -> crate::Result<DetachedWindow<P>>;

  /// Run the webview runtime.
  fn run(self);
}

/// Webview dispatcher. A thread-safe handle to the webview API.
pub trait Dispatch: Clone + Send + Sized + 'static {
  /// The runtime this [`Dispatch`] runs under.
  type Runtime: Runtime;

  /// Representation of a window icon.
  type Icon: TryFrom<Icon, Error = crate::Error>;

  /// The webview builder type.
  type Attributes: Attributes<Icon = Self::Icon> + AttributesBase + Clone + Send;

  /// Create a new webview window.
  fn create_window<P: Params<Runtime = Self::Runtime>>(
    &mut self,
    pending: PendingWindow<P>,
  ) -> crate::Result<DetachedWindow<P>>;

  /// Updates the window resizable flag.
  fn set_resizable(&self, resizable: bool) -> crate::Result<()>;

  /// Updates the window title.
  fn set_title<S: Into<String>>(&self, title: S) -> crate::Result<()>;

  /// Maximizes the window.
  fn maximize(&self) -> crate::Result<()>;

  /// Unmaximizes the window.
  fn unmaximize(&self) -> crate::Result<()>;

  /// Minimizes the window.
  fn minimize(&self) -> crate::Result<()>;

  /// Unminimizes the window.
  fn unminimize(&self) -> crate::Result<()>;

  /// Shows the window.
  fn show(&self) -> crate::Result<()>;

  /// Hides the window.
  fn hide(&self) -> crate::Result<()>;

  /// Closes the window.
  fn close(&self) -> crate::Result<()>;

  /// Updates the hasDecorations flag.
  fn set_decorations(&self, decorations: bool) -> crate::Result<()>;

  /// Updates the window alwaysOnTop flag.
  fn set_always_on_top(&self, always_on_top: bool) -> crate::Result<()>;

  /// Updates the window width.
  fn set_width(&self, width: f64) -> crate::Result<()>;

  /// Updates the window height.
  fn set_height(&self, height: f64) -> crate::Result<()>;

  /// Resizes the window.
  fn resize(&self, width: f64, height: f64) -> crate::Result<()>;

  /// Updates the window min size.
  fn set_min_size(&self, min_width: f64, min_height: f64) -> crate::Result<()>;

  /// Updates the window max size.
  fn set_max_size(&self, max_width: f64, max_height: f64) -> crate::Result<()>;

  /// Updates the X position.
  fn set_x(&self, x: f64) -> crate::Result<()>;

  /// Updates the Y position.
  fn set_y(&self, y: f64) -> crate::Result<()>;

  /// Updates the window position.
  fn set_position(&self, x: f64, y: f64) -> crate::Result<()>;

  /// Updates the window fullscreen state.
  fn set_fullscreen(&self, fullscreen: bool) -> crate::Result<()>;

  /// Updates the window icon.
  fn set_icon(&self, icon: Self::Icon) -> crate::Result<()>;

  /// Executes javascript on the window this [`Dispatch`] represents.
  fn eval_script<S: Into<String>>(&self, script: S) -> crate::Result<()>;
}
