// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{
  api::{assets::Assets, config::WindowUrl},
  hooks::{InvokeHandler, OnPageLoad, PageLoadPayload, SetupHook},
  plugin::{Plugin, PluginStore},
  runtime::{
    flavors::wry::Wry,
    manager::{Args, WindowManager},
    tag::Tag,
    webview::{CustomProtocol, WebviewAttributes, WindowBuilder},
    window::PendingWindow,
    Dispatch, Runtime,
  },
  sealed::{ManagerBase, RuntimeOrDispatch},
  Context, Invoke, Manager, Params, StateManager, Window,
};

use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "updater")]
use crate::updater;

/// A handle to the currently running application.
///
/// This type implements [`Manager`] which allows for manipulation of global application items.
pub struct App<P: Params> {
  runtime: P::Runtime,
  manager: WindowManager<P>,
}

impl<P: Params> Manager<P> for App<P> {}
impl<P: Params> ManagerBase<P> for App<P> {
  fn manager(&self) -> &WindowManager<P> {
    &self.manager
  }

  fn runtime(&mut self) -> RuntimeOrDispatch<'_, P> {
    RuntimeOrDispatch::Runtime(&mut self.runtime)
  }
}

#[cfg(feature = "updater")]
impl<M: Params> App<M> {
  /// Runs the updater hook with built-in dialog.
  fn run_updater_dialog(&self, window: Window<M>) {
    let updater_config = self.manager.config().tauri.updater.clone();
    let package_info = self.manager.package_info().clone();
    crate::async_runtime::spawn(async move {
      updater::check_update_with_dialog(updater_config, package_info, window).await
    });
  }

  /// Listen updater events when dialog are disabled.
  fn listen_updater_events(&self, window: Window<M>) {
    let updater_config = self.manager.config().tauri.updater.clone();
    updater::listener(updater_config, self.manager.package_info().clone(), &window);
  }

  fn run_updater(&self, main_window: Option<Window<M>>) {
    if let Some(main_window) = main_window {
      let event_window = main_window.clone();
      let updater_config = self.manager.config().tauri.updater.clone();
      // check if updater is active or not
      if updater_config.dialog && updater_config.active {
        // if updater dialog is enabled spawn a new task
        self.run_updater_dialog(main_window.clone());
        let config = self.manager.config().tauri.updater.clone();
        let package_info = self.manager.package_info().clone();
        // When dialog is enabled, if user want to recheck
        // if an update is available after first start
        // invoke the Event `tauri://update` from JS or rust side.
        main_window.listen(
          updater::EVENT_CHECK_UPDATE
            .parse::<M::Event>()
            .unwrap_or_else(|_| panic!("bad label")),
          move |_msg| {
            let window = event_window.clone();
            let package_info = package_info.clone();
            let config = config.clone();
            // re-spawn task inside tokyo to launch the download
            // we don't need to emit anything as everything is handled
            // by the process (user is asked to restart at the end)
            // and it's handled by the updater
            crate::async_runtime::spawn(async move {
              updater::check_update_with_dialog(config, package_info, window).await
            });
          },
        );
      } else if updater_config.active {
        // we only listen for `tauri://update`
        // once we receive the call, we check if an update is available or not
        // if there is a new update we emit `tauri://update-available` with details
        // this is the user responsabilities to display dialog and ask if user want to install
        // to install the update you need to invoke the Event `tauri://update-install`
        self.listen_updater_events(main_window);
      }
    }
  }
}

/// Builds a Tauri application.
pub struct Builder<E, L, A, R>
where
  E: Tag,
  L: Tag,
  A: Assets,
  R: Runtime,
{
  /// The JS message handler.
  invoke_handler: Box<InvokeHandler<Args<E, L, A, R>>>,

  /// The setup hook.
  setup: SetupHook<Args<E, L, A, R>>,

  /// Page load hook.
  on_page_load: Box<OnPageLoad<Args<E, L, A, R>>>,

  /// windows to create when starting up.
  pending_windows: Vec<PendingWindow<Args<E, L, A, R>>>,

  /// All passed plugins
  plugins: PluginStore<Args<E, L, A, R>>,

  /// The webview protocols available to all windows.
  uri_scheme_protocols: HashMap<String, Arc<CustomProtocol>>,

  /// App state.
  state: StateManager,
}

impl<E, L, A, R> Builder<E, L, A, R>
where
  E: Tag,
  L: Tag,
  A: Assets,
  R: Runtime,
{
  /// Creates a new App builder.
  pub fn new() -> Self {
    Self {
      setup: Box::new(|_| Ok(())),
      invoke_handler: Box::new(|_| ()),
      on_page_load: Box::new(|_, _| ()),
      pending_windows: Default::default(),
      plugins: PluginStore::default(),
      uri_scheme_protocols: Default::default(),
      state: StateManager::new(),
    }
  }

  /// Defines the JS message handler callback.
  pub fn invoke_handler<F>(mut self, invoke_handler: F) -> Self
  where
    F: Fn(Invoke<Args<E, L, A, R>>) + Send + Sync + 'static,
  {
    self.invoke_handler = Box::new(invoke_handler);
    self
  }

  /// Defines the setup hook.
  pub fn setup<F>(mut self, setup: F) -> Self
  where
    F: Fn(&mut App<Args<E, L, A, R>>) -> Result<(), Box<dyn std::error::Error + Send>>
      + Send
      + 'static,
  {
    self.setup = Box::new(setup);
    self
  }

  /// Defines the page load hook.
  pub fn on_page_load<F>(mut self, on_page_load: F) -> Self
  where
    F: Fn(Window<Args<E, L, A, R>>, PageLoadPayload) + Send + Sync + 'static,
  {
    self.on_page_load = Box::new(on_page_load);
    self
  }

  /// Adds a plugin to the runtime.
  pub fn plugin<P: Plugin<Args<E, L, A, R>> + 'static>(mut self, plugin: P) -> Self {
    self.plugins.register(plugin);
    self
  }

  /// Add `state` to the state managed by the application.
  ///
  /// This method can be called any number of times as long as each call
  /// refers to a different `T`.
  ///
  /// Managed state can be retrieved by any request handler via the
  /// [`State`](crate::State) request guard. In particular, if a value of type `T`
  /// is managed by Tauri, adding `State<T>` to the list of arguments in a
  /// request handler instructs Tauri to retrieve the managed value.
  ///
  /// # Panics
  ///
  /// Panics if state of type `T` is already being managed.
  ///
  /// # Example
  ///
  /// ```rust,ignore
  /// use tauri::State;
  ///
  /// struct MyInt(isize);
  /// struct MyString(String);
  ///
  /// #[tauri::command]
  /// fn int_command(state: State<'_, MyInt>) -> String {
  ///     format!("The stateful int is: {}", state.0)
  /// }
  ///
  /// #[tauri::command]
  /// fn string_command<'r>(state: State<'r, MyString>) {
  ///     println!("state: {}", state.inner().0);
  /// }
  ///
  /// fn main() {
  ///     tauri::Builder::default()
  ///         .manage(MyInt(10))
  ///         .manage(MyString("Hello, managed state!".to_string()))
  ///         .run(tauri::generate_context!())
  ///         .expect("error while running tauri application");
  /// }
  /// ```
  pub fn manage<T>(self, state: T) -> Self
  where
    T: Send + Sync + 'static,
  {
    let type_name = std::any::type_name::<T>();
    if !self.state.set(state) {
      panic!("state for type '{}' is already being managed", type_name);
    }

    self
  }

  /// Creates a new webview.
  pub fn create_window<F>(mut self, label: L, url: WindowUrl, setup: F) -> Self
  where
    F: FnOnce(
      <R::Dispatcher as Dispatch>::WindowBuilder,
      WebviewAttributes,
    ) -> (
      <R::Dispatcher as Dispatch>::WindowBuilder,
      WebviewAttributes,
    ),
  {
    let (window_attributes, webview_attributes) = setup(
      <R::Dispatcher as Dispatch>::WindowBuilder::new(),
      WebviewAttributes::new(url),
    );
    self.pending_windows.push(PendingWindow::new(
      window_attributes,
      webview_attributes,
      label,
    ));
    self
  }

  /// Registers a URI scheme protocol available to all webviews.
  /// Leverages [setURLSchemeHandler](https://developer.apple.com/documentation/webkit/wkwebviewconfiguration/2875766-seturlschemehandler) on macOS,
  /// [AddWebResourceRequestedFilter](https://docs.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2.addwebresourcerequestedfilter?view=webview2-dotnet-1.0.774.44) on Windows
  /// and [webkit-web-context-register-uri-scheme](https://webkitgtk.org/reference/webkit2gtk/stable/WebKitWebContext.html#webkit-web-context-register-uri-scheme) on Linux.
  ///
  /// # Arguments
  ///
  /// * `uri_scheme` The URI scheme to register, such as `example`.
  /// * `protocol` the protocol associated with the given URI scheme. It's a function that takes an URL such as `example://localhost/asset.css`.
  pub fn register_global_uri_scheme_protocol<
    N: Into<String>,
    H: Fn(&str) -> crate::Result<Vec<u8>> + Send + Sync + 'static,
  >(
    mut self,
    uri_scheme: N,
    protocol: H,
  ) -> Self {
    self.uri_scheme_protocols.insert(
      uri_scheme.into(),
      Arc::new(CustomProtocol {
        protocol: Box::new(protocol),
      }),
    );
    self
  }

  /// Runs the configured Tauri application.
  pub fn run(mut self, context: Context<A>) -> crate::Result<()> {
    let manager = WindowManager::with_handlers(
      context,
      self.plugins,
      self.invoke_handler,
      self.on_page_load,
      self.uri_scheme_protocols,
      self.state,
    );

    // set up all the windows defined in the config
    for config in manager.config().tauri.windows.clone() {
      let url = config.url.clone();
      let label = config
        .label
        .parse()
        .unwrap_or_else(|_| panic!("bad label found in config: {}", config.label));

      self.pending_windows.push(PendingWindow::with_config(
        config,
        WebviewAttributes::new(url),
        label,
      ));
    }

    let mut app = App {
      runtime: R::new()?,
      manager,
    };

    app.manager.initialize_plugins(&app)?;

    let pending_labels = self
      .pending_windows
      .iter()
      .map(|p| p.label.clone())
      .collect::<Vec<_>>();

    #[cfg(feature = "updater")]
    let mut main_window = None;

    for pending in self.pending_windows {
      let pending = app.manager.prepare_window(pending, &pending_labels)?;
      let detached = app.runtime.create_window(pending)?;
      let _window = app.manager.attach_window(detached);
      #[cfg(feature = "updater")]
      if main_window.is_none() {
        main_window = Some(_window);
      }
    }

    #[cfg(feature = "updater")]
    app.run_updater(main_window);

    (self.setup)(&mut app).map_err(|e| crate::Error::Setup(e))?;

    app.runtime.run();
    Ok(())
  }
}

/// Make `Wry` the default `Runtime` for `Builder`
impl<A: Assets> Default for Builder<String, String, A, Wry> {
  fn default() -> Self {
    Self::new()
  }
}
