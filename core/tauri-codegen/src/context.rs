// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::embedded_assets::{AssetOptions, EmbeddedAssets, EmbeddedAssetsError};
use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;
use tauri_utils::config::Config;

/// Necessary data needed by [`context_codegen`] to generate code for a Tauri application context.
pub struct ContextData {
  pub dev: bool,
  pub config: Config,
  pub config_parent: PathBuf,
  pub root: TokenStream,
}

/// Build a `tauri::Context` for including in application code.
pub fn context_codegen(data: ContextData) -> Result<TokenStream, EmbeddedAssetsError> {
  let ContextData {
    dev,
    config,
    config_parent,
    root,
  } = data;
  let assets_path = if dev {
    // if dev_path is a dev server, we don't have any assets to embed
    if config.build.dev_path.starts_with("http") {
      None
    } else {
      Some(config_parent.join(&config.build.dev_path))
    }
  } else {
    Some(config_parent.join(&config.build.dist_dir))
  };

  // generate the assets inside the dist dir into a perfect hash function
  let assets = if let Some(assets_path) = assets_path {
    let mut options = AssetOptions::new();
    if let Some(csp) = &config.tauri.security.csp {
      options = options.csp(csp.clone());
    }
    EmbeddedAssets::new(&assets_path, options)?
  } else {
    Default::default()
  };

  // handle default window icons for Windows targets
  let default_window_icon = if cfg!(windows) {
    let icon_path = config
      .tauri
      .bundle
      .icon
      .iter()
      .find(|i| i.ends_with(".ico"))
      .cloned()
      .unwrap_or_else(|| "icons/icon.ico".to_string());
    let icon_path = config_parent.join(icon_path).display().to_string();
    quote!(Some(include_bytes!(#icon_path).to_vec()))
  } else {
    quote!(None)
  };

  let package_name = if let Some(product_name) = &config.package.product_name {
    quote!(#product_name.to_string())
  } else {
    quote!(env!("CARGO_PKG_NAME").to_string())
  };
  let package_version = if let Some(version) = &config.package.version {
    quote!(#version.to_string())
  } else {
    quote!(env!("CARGO_PKG_VERSION").to_string())
  };

  // double braces are purposeful to force the code into a block expression
  Ok(quote!(#root::Context {
    config: #config,
    assets: ::std::sync::Arc::new(#assets),
    default_window_icon: #default_window_icon,
    package_info: #root::api::PackageInfo {
      name: #package_name,
      version: #package_version,
    },
  }))
}
