// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use super::InvokeResponse;
use serde::Deserialize;

#[cfg(notification_all)]
use crate::api::notification::Notification;
use crate::Config;

use std::sync::Arc;

// `Granted` response from `request_permission`. Matches the Web API return value.
#[cfg(notification_all)]
const PERMISSION_GRANTED: &str = "granted";
// `Denied` response from `request_permission`. Matches the Web API return value.
const PERMISSION_DENIED: &str = "denied";

/// The options for the notification API.
#[derive(Deserialize)]
pub struct NotificationOptions {
  /// The notification title.
  pub title: String,
  /// The notification body.
  pub body: Option<String>,
  /// The notification icon.
  pub icon: Option<String>,
}

/// The API descriptor.
#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  /// The show notification API.
  Notification { options: NotificationOptions },
  /// The request notification permission API.
  RequestNotificationPermission,
  /// The notification permission check API.
  IsNotificationPermissionGranted,
}

impl Cmd {
  #[allow(unused_variables)]
  pub fn run(self, config: Arc<Config>) -> crate::Result<InvokeResponse> {
    match self {
      #[cfg(notification_all)]
      Self::Notification { options } => send(options, &config).map(Into::into),
      #[cfg(not(notification_all))]
      Self::Notification { .. } => Err(crate::Error::ApiNotAllowlisted("notification".to_string())),
      Self::IsNotificationPermissionGranted => {
        #[cfg(notification_all)]
        return is_permission_granted(&config).map(Into::into);
        #[cfg(not(notification_all))]
        Ok(false.into())
      }
      Self::RequestNotificationPermission => {
        #[cfg(notification_all)]
        return request_permission(&config).map(Into::into);
        #[cfg(not(notification_all))]
        Ok(PERMISSION_DENIED.into())
      }
    }
  }
}

#[cfg(notification_all)]
pub fn send(options: NotificationOptions, config: &Config) -> crate::Result<InvokeResponse> {
  let mut notification =
    Notification::new(config.tauri.bundle.identifier.clone()).title(options.title);
  if let Some(body) = options.body {
    notification = notification.body(body);
  }
  if let Some(icon) = options.icon {
    notification = notification.icon(icon);
  }
  notification.show()?;
  Ok(().into())
}

#[cfg(notification_all)]
pub fn is_permission_granted(config: &Config) -> crate::Result<InvokeResponse> {
  let settings = crate::settings::read_settings(config)?;
  if let Some(allow_notification) = settings.allow_notification {
    Ok(allow_notification.into())
  } else {
    Ok(().into())
  }
}

#[cfg(notification_all)]
pub fn request_permission(config: &Config) -> crate::Result<String> {
  let mut settings = crate::settings::read_settings(config)?;
  if let Some(allow_notification) = settings.allow_notification {
    return Ok(if allow_notification {
      PERMISSION_GRANTED.to_string()
    } else {
      PERMISSION_DENIED.to_string()
    });
  }
  let answer = crate::api::dialog::ask(
    "Permissions",
    "This app wants to show notifications. Do you allow?",
  );
  match answer {
    crate::api::dialog::AskResponse::Yes => {
      settings.allow_notification = Some(true);
      crate::settings::write_settings(config, settings)?;
      Ok(PERMISSION_GRANTED.to_string())
    }
    crate::api::dialog::AskResponse::No => {
      settings.allow_notification = Some(false);
      crate::settings::write_settings(config, settings)?;
      Ok(PERMISSION_DENIED.to_string())
    }
  }
}
