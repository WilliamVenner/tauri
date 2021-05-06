// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! A layer between raw [`Runtime`] webview windows and Tauri.

use crate::{
  api::config::WindowConfig,
  event::{Event, EventHandler},
  hooks::{InvokeMessage, InvokeResolver, PageLoadPayload},
  runtime::{
    tag::ToJsString,
    webview::{FileDropHandler, InvokePayload, WebviewAttributes, WebviewRpcHandler},
    Dispatch, Monitor, Runtime,
  },
  sealed::{ManagerBase, RuntimeOrDispatch},
  Icon, Manager, Params, WindowBuilder,
};
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::hash::{Hash, Hasher};

/// UI scaling utilities.
pub mod dpi;

/// An event from a window.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum WindowEvent {
  /// The size of the window has changed. Contains the client area's new dimensions.
  Resized(dpi::PhysicalSize<u32>),
  /// The position of the window has changed. Contains the window's new position.
  Moved(dpi::PhysicalPosition<i32>),
  /// The window has been requested to close.
  CloseRequested,
  /// The window has been destroyed.
  Destroyed,
  /// The window gained or lost focus.
  ///
  /// The parameter is true if the window has gained focus, and false if it has lost focus.
  Focused(bool),
  ///The window's scale factor has changed.
  ///
  /// The following user actions can cause DPI changes:
  ///
  /// - Changing the display's resolution.
  /// - Changing the display's scale factor (e.g. in Control Panel on Windows).
  /// - Moving the window to a display with a different scale factor.
  #[non_exhaustive]
  ScaleFactorChanged {
    /// The new scale factor.
    scale_factor: f64,
    /// The window inner size.
    new_inner_size: dpi::PhysicalSize<u32>,
  },
}

/// A webview window that has yet to be built.
pub struct PendingWindow<M: Params> {
  /// The label that the window will be named.
  pub label: M::Label,

  /// The [`WindowBuilder`] that the window will be created with.
  pub window_attributes: <<M::Runtime as Runtime>::Dispatcher as Dispatch>::WindowBuilder,

  /// The [`WebviewAttributes`] that the webview will be created with.
  pub webview_attributes: WebviewAttributes,

  /// How to handle RPC calls on the webview window.
  pub rpc_handler: Option<WebviewRpcHandler<M>>,

  /// How to handle a file dropping onto the webview window.
  pub file_drop_handler: Option<FileDropHandler<M>>,

  /// The resolved URL to load on the webview.
  pub url: String,
}

impl<M: Params> PendingWindow<M> {
  /// Create a new [`PendingWindow`] with a label and starting url.
  pub fn new(
    window_attributes: <<M::Runtime as Runtime>::Dispatcher as Dispatch>::WindowBuilder,
    webview_attributes: WebviewAttributes,
    label: M::Label,
  ) -> Self {
    Self {
      window_attributes,
      webview_attributes,
      label,
      rpc_handler: None,
      file_drop_handler: None,
      url: "tauri://localhost".to_string(),
    }
  }

  /// Create a new [`PendingWindow`] from a [`WindowConfig`] with a label and starting url.
  pub fn with_config(
    window_config: WindowConfig,
    webview_attributes: WebviewAttributes,
    label: M::Label,
  ) -> Self {
    Self {
      window_attributes:
        <<<M::Runtime as Runtime>::Dispatcher as Dispatch>::WindowBuilder>::with_config(
          window_config,
        ),
      webview_attributes,
      label,
      rpc_handler: None,
      file_drop_handler: None,
      url: "tauri://localhost".to_string(),
    }
  }
}

/// A webview window that is not yet managed by Tauri.
pub struct DetachedWindow<M: Params> {
  /// Name of the window
  pub label: M::Label,

  /// The [`Dispatch`](crate::runtime::Dispatch) associated with the window.
  pub dispatcher: <M::Runtime as Runtime>::Dispatcher,
}

impl<M: Params> Clone for DetachedWindow<M> {
  fn clone(&self) -> Self {
    Self {
      label: self.label.clone(),
      dispatcher: self.dispatcher.clone(),
    }
  }
}

impl<M: Params> Hash for DetachedWindow<M> {
  /// Only use the [`DetachedWindow`]'s label to represent its hash.
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.label.hash(state)
  }
}

impl<M: Params> Eq for DetachedWindow<M> {}
impl<M: Params> PartialEq for DetachedWindow<M> {
  /// Only use the [`DetachedWindow`]'s label to compare equality.
  fn eq(&self, other: &Self) -> bool {
    self.label.eq(&other.label)
  }
}

/// We want to export the runtime related window at the crate root, but not look like a re-export.
pub(crate) mod export {
  pub(crate) use super::dpi;
  use super::*;
  use crate::command::{CommandArg, CommandItem};
  use crate::runtime::{manager::WindowManager, tag::TagRef};
  use crate::{Invoke, InvokeError};
  use dpi::{PhysicalPosition, PhysicalSize, Position, Size};
  use std::borrow::Borrow;

  /// A webview window managed by Tauri.
  ///
  /// This type also implements [`Manager`] which allows you to manage other windows attached to
  /// the same application.
  ///
  /// TODO: expand these docs since this is a pretty important type
  pub struct Window<P: Params> {
    /// The webview window created by the runtime.
    window: DetachedWindow<P>,

    /// The manager to associate this webview window with.
    manager: WindowManager<P>,
  }

  impl<M: Params> Clone for Window<M> {
    fn clone(&self) -> Self {
      Self {
        window: self.window.clone(),
        manager: self.manager.clone(),
      }
    }
  }

  impl<P: Params> Hash for Window<P> {
    /// Only use the [`Window`]'s label to represent its hash.
    fn hash<H: Hasher>(&self, state: &mut H) {
      self.window.label.hash(state)
    }
  }

  impl<P: Params> Eq for Window<P> {}
  impl<P: Params> PartialEq for Window<P> {
    /// Only use the [`Window`]'s label to compare equality.
    fn eq(&self, other: &Self) -> bool {
      self.window.label.eq(&other.window.label)
    }
  }

  impl<P: Params> Manager<P> for Window<P> {}
  impl<P: Params> ManagerBase<P> for Window<P> {
    fn manager(&self) -> &WindowManager<P> {
      &self.manager
    }

    fn runtime(&mut self) -> RuntimeOrDispatch<'_, P> {
      RuntimeOrDispatch::Dispatch(self.dispatcher())
    }
  }

  impl<'de, P: Params> CommandArg<'de, P> for Window<P> {
    /// Grabs the [`Window`] from the [`CommandItem`]. This will never fail.
    fn from_command(command: CommandItem<'de, P>) -> Result<Self, InvokeError> {
      Ok(command.message.window())
    }
  }

  impl<P: Params> Window<P> {
    /// Create a new window that is attached to the manager.
    pub(crate) fn new(manager: WindowManager<P>, window: DetachedWindow<P>) -> Self {
      Self { window, manager }
    }

    /// The current window's dispatcher.
    pub(crate) fn dispatcher(&self) -> <P::Runtime as Runtime>::Dispatcher {
      self.window.dispatcher.clone()
    }

    pub(crate) fn run_on_main_thread<F: FnOnce() + Send + 'static>(
      &self,
      f: F,
    ) -> crate::Result<()> {
      self.window.dispatcher.run_on_main_thread(f)
    }

    /// How to handle this window receiving an [`InvokeMessage`].
    pub(crate) fn on_message(self, command: String, payload: InvokePayload) -> crate::Result<()> {
      let manager = self.manager.clone();
      match command.as_str() {
        "__initialized" => {
          let payload: PageLoadPayload = serde_json::from_value(payload.inner)?;
          manager.run_on_page_load(self, payload);
        }
        _ => {
          let message = InvokeMessage::new(
            self.clone(),
            manager.state(),
            command.to_string(),
            payload.inner,
          );
          let resolver = InvokeResolver::new(self, payload.callback, payload.error);
          let invoke = Invoke { message, resolver };
          if let Some(module) = &payload.tauri_module {
            let module = module.to_string();
            crate::endpoints::handle(module, invoke, manager.config(), manager.package_info());
          } else if command.starts_with("plugin:") {
            manager.extend_api(invoke);
          } else {
            manager.run_invoke_handler(invoke);
          }
        }
      }

      Ok(())
    }

    /// The label of this window.
    pub fn label(&self) -> &P::Label {
      &self.window.label
    }

    pub(crate) fn emit_internal<E: ?Sized, S>(
      &self,
      event: &E,
      payload: Option<S>,
    ) -> crate::Result<()>
    where
      P::Event: Borrow<E>,
      E: TagRef<P::Event>,
      S: Serialize,
    {
      let js_payload = match payload {
        Some(payload_value) => serde_json::to_value(payload_value)?,
        None => JsonValue::Null,
      };

      self.eval(&format!(
        "window['{}']({{event: {}, payload: {}}}, '{}')",
        self.manager.event_emit_function_name(),
        event.to_js_string()?,
        js_payload,
        self.manager.generate_salt(),
      ))?;

      Ok(())
    }

    /// Emits an event to the current window.
    pub fn emit<E: ?Sized, S>(&self, event: &E, payload: Option<S>) -> crate::Result<()>
    where
      P::Event: Borrow<E>,
      E: TagRef<P::Event>,
      S: Serialize,
    {
      self.emit_internal(event, payload)
    }

    /// Emits an event on all windows except this one.
    pub fn emit_others<E: ?Sized, S>(&self, event: &E, payload: Option<S>) -> crate::Result<()>
    where
      P::Event: Borrow<E>,
      E: TagRef<P::Event>,
      S: Serialize + Clone,
    {
      self.manager.emit_filter(event, payload, |w| w != self)
    }

    /// Listen to an event on this window.
    pub fn listen<E: Into<P::Event>, F>(&self, event: E, handler: F) -> EventHandler
    where
      F: Fn(Event) + Send + 'static,
    {
      let label = self.window.label.clone();
      self.manager.listen(event.into(), Some(label), handler)
    }

    /// Listen to a an event on this window a single time.
    pub fn once<E: Into<P::Event>, F>(&self, event: E, handler: F) -> EventHandler
    where
      F: Fn(Event) + Send + 'static,
    {
      let label = self.window.label.clone();
      self.manager.once(event.into(), Some(label), handler)
    }

    /// Triggers an event on this window.
    pub fn trigger<E: ?Sized>(&self, event: &E, data: Option<String>)
    where
      P::Event: Borrow<E>,
      E: TagRef<P::Event>,
    {
      let label = self.window.label.clone();
      self.manager.trigger(event, Some(label), data)
    }

    /// Evaluates JavaScript on this window.
    pub fn eval(&self, js: &str) -> crate::Result<()> {
      self.window.dispatcher.eval_script(js)
    }

    /// Registers a window event listener.
    pub fn on_window_event<F: Fn(&WindowEvent) + Send + 'static>(&self, f: F) {
      self.window.dispatcher.on_window_event(f);
    }

    // Getters

    /// Returns the scale factor that can be used to map logical pixels to physical pixels, and vice versa.
    pub fn scale_factor(&self) -> crate::Result<f64> {
      self.window.dispatcher.scale_factor()
    }

    /// Returns the position of the top-left hand corner of the window's client area relative to the top-left hand corner of the desktop.
    pub fn inner_position(&self) -> crate::Result<PhysicalPosition<i32>> {
      self.window.dispatcher.inner_position()
    }

    /// Returns the position of the top-left hand corner of the window relative to the top-left hand corner of the desktop.
    pub fn outer_position(&self) -> crate::Result<PhysicalPosition<i32>> {
      self.window.dispatcher.outer_position()
    }

    /// Returns the physical size of the window's client area.
    ///
    /// The client area is the content of the window, excluding the title bar and borders.
    pub fn inner_size(&self) -> crate::Result<PhysicalSize<u32>> {
      self.window.dispatcher.inner_size()
    }

    /// Returns the physical size of the entire window.
    ///
    /// These dimensions include the title bar and borders. If you don't want that (and you usually don't), use inner_size instead.
    pub fn outer_size(&self) -> crate::Result<PhysicalSize<u32>> {
      self.window.dispatcher.outer_size()
    }

    /// Gets the window's current fullscreen state.
    pub fn is_fullscreen(&self) -> crate::Result<bool> {
      self.window.dispatcher.is_fullscreen()
    }

    /// Gets the window's current maximized state.
    pub fn is_maximized(&self) -> crate::Result<bool> {
      self.window.dispatcher.is_maximized()
    }

    /// Returns the monitor on which the window currently resides.
    ///
    /// Returns None if current monitor can't be detected.
    pub fn current_monitor(&self) -> crate::Result<Option<Monitor>> {
      self.window.dispatcher.current_monitor()
    }

    /// Returns the primary monitor of the system.
    ///
    /// Returns None if it can't identify any monitor as a primary one.
    pub fn primary_monitor(&self) -> crate::Result<Option<Monitor>> {
      self.window.dispatcher.primary_monitor()
    }

    /// Returns the list of all the monitors available on the system.
    pub fn available_monitors(&self) -> crate::Result<Vec<Monitor>> {
      self.window.dispatcher.available_monitors()
    }

    // Setters

    /// Determines if this window should be resizable.
    pub fn set_resizable(&self, resizable: bool) -> crate::Result<()> {
      self.window.dispatcher.set_resizable(resizable)
    }

    /// Set this window's title.
    pub fn set_title(&self, title: &str) -> crate::Result<()> {
      self.window.dispatcher.set_title(title.to_string())
    }

    /// Maximizes this window.
    pub fn maximize(&self) -> crate::Result<()> {
      self.window.dispatcher.maximize()
    }

    /// Un-maximizes this window.
    pub fn unmaximize(&self) -> crate::Result<()> {
      self.window.dispatcher.unmaximize()
    }

    /// Minimizes this window.
    pub fn minimize(&self) -> crate::Result<()> {
      self.window.dispatcher.minimize()
    }

    /// Un-minimizes this window.
    pub fn unminimize(&self) -> crate::Result<()> {
      self.window.dispatcher.unminimize()
    }

    /// Show this window.
    pub fn show(&self) -> crate::Result<()> {
      self.window.dispatcher.show()
    }

    /// Hide this window.
    pub fn hide(&self) -> crate::Result<()> {
      self.window.dispatcher.hide()
    }

    /// Closes this window.
    pub fn close(&self) -> crate::Result<()> {
      self.window.dispatcher.close()
    }

    /// Determines if this window should be [decorated].
    ///
    /// [decorated]: https://en.wikipedia.org/wiki/Window_(computing)#Window_decoration
    pub fn set_decorations(&self, decorations: bool) -> crate::Result<()> {
      self.window.dispatcher.set_decorations(decorations)
    }

    /// Determines if this window should always be on top of other windows.
    pub fn set_always_on_top(&self, always_on_top: bool) -> crate::Result<()> {
      self.window.dispatcher.set_always_on_top(always_on_top)
    }

    /// Resizes this window.
    pub fn set_size<S: Into<Size>>(&self, size: S) -> crate::Result<()> {
      self.window.dispatcher.set_size(size.into())
    }

    /// Sets this window's minimum size.
    pub fn set_min_size<S: Into<Size>>(&self, size: Option<S>) -> crate::Result<()> {
      self.window.dispatcher.set_min_size(size.map(|s| s.into()))
    }

    /// Sets this window's maximum size.
    pub fn set_max_size<S: Into<Size>>(&self, size: Option<S>) -> crate::Result<()> {
      self.window.dispatcher.set_max_size(size.map(|s| s.into()))
    }

    /// Sets this window's position.
    pub fn set_position<Pos: Into<Position>>(&self, position: Pos) -> crate::Result<()> {
      self.window.dispatcher.set_position(position.into())
    }

    /// Determines if this window should be fullscreen.
    pub fn set_fullscreen(&self, fullscreen: bool) -> crate::Result<()> {
      self.window.dispatcher.set_fullscreen(fullscreen)
    }

    /// Sets this window' icon.
    pub fn set_icon(&self, icon: Icon) -> crate::Result<()> {
      self.window.dispatcher.set_icon(icon)
    }

    /// Starts dragging the window.
    pub fn start_dragging(&self) -> crate::Result<()> {
      self.window.dispatcher.start_dragging()
    }

    pub(crate) fn verify_salt(&self, salt: String) -> bool {
      self.manager.verify_salt(salt)
    }
  }
}
