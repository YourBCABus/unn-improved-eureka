//! Logging-related.
//! 
//! Contains:
//! - macros for general logging:
//!     - [`trace`]
//!     - [`debug`]
//!     - [`info`]
//!     - [`warn`]
//!     - [`error`]
//! - [`shortened`] for displayable shortened strings
//! 
//! Usually you should just import all of it with
//! ```no_run
//! use logging::*;
//! ```

#![feature(panic_update_hook)]

use arcs_logging_rs::with_target;
with_target! { "TableJet Improved Eureka" }

/// Display struct for [`shortened`]
pub struct Shortened<'a>(&'a str, bool);
/// Get a version of a string which can be capped at a certain number of characters
/// 
/// This function is relatively fault-tolerant, and will default to the full
/// string if it can't shorten it correctly.
pub fn shortened(string: &str, max_len: usize) -> Shortened {
    let (display_name, shortened) =  if string.chars().count() >= max_len {
        if let Some((idx, _)) = string.char_indices().nth(max_len-3) {
            (&string[..idx], true)
        } else { (string, false) }
    } else { (string, false) };

    Shortened(display_name, shortened)
}

impl<'a> std::fmt::Display for Shortened<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        if self.1 {
            write!(f, "...")
        } else {
            Ok(())
        }
    }
}
impl<'a> std::fmt::Debug for Shortened<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{:?}", self.0)?;
        if self.1 {
            write!(f, "...")?;
        }
        write!(f, "`")
    }
}


pub struct SmallId(pub Option<&'static str>, pub uuid::Uuid);

impl std::fmt::Display for SmallId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<")?;
        if let Some(prefix) = self.0 {
            write!(f, "{prefix}:")?;
        }
        write!(f, "{:08x}>", self.1.as_fields().0)
    }
}

pub fn fmt_req_id(id: uuid::Uuid) -> SmallId {
    SmallId(Some("req"), id)
}
pub fn fmt_client_id(id: uuid::Uuid) -> SmallId {
    SmallId(Some("client"), id)
}
pub fn fmt_user_id(id: uuid::Uuid) -> SmallId {
    SmallId(Some("user"), id)
}

pub async fn report(message: &str, data: &impl serde::Serialize) {
    if let Err(e) = yenowa_errors::report(message, data).await {
        crate::error!("Failed to report error to the logging service {e:?}");
    }
}

#[macro_export]
macro_rules! report {
    ($message:literal: $json:tt) => {
        $crate::report($message, &::serde_json::json! { $json }).await
    };
}

static BACKTRACE: std::sync::RwLock<Option<(
    // For capturing the backtrace
    std::backtrace::Backtrace,

    // As a little note for the developers in case setting the backtrace
    // ever fails that the current one might be stale
    std::time::Instant,
)>> = std::sync::RwLock::new(None);

pub fn set_panic_hook() {
    fn additional_hook(_: &std::panic::PanicHookInfo<'_>) {
        let Ok(mut backtrace) = BACKTRACE.write() else { return };
        *backtrace = Some((std::backtrace::Backtrace::force_capture(), std::time::Instant::now()));
    }
    std::panic::update_hook(|orig, info| {
        orig(info);
        additional_hook(info);
    });
}

pub fn get_backtrace() -> String {
    let Ok(backtrace) = BACKTRACE.read() else {
        return "Unable to read recorded backtrace".to_string();
    };
    backtrace.as_ref()
        .map(|(b, t)| format!(
            "Backtrace from {:.3}ms ago:\n{}",
            t.elapsed().as_secs_f64() * 1000.0,
            b,
        ))
        .unwrap_or_else(|| "No backtrace found".to_string())
}

#[macro_export]
macro_rules! report_panics_sync {
    ($($body:tt)+) => {
        match std::panic::catch_unwind(|| { $($body)+ }) {
            Ok(v) => v,
            Err(e) => {
                let formatted = if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "<unable to read panic message>".to_string()
                };
                let backtrace = $crate::get_backtrace();

                $crate::error!("Caught panic in handler:\n{formatted}\n{backtrace}");
                tokio::task::spawn_blocking(|| async move {
                    $crate::report!("Caught panic in handler": {
                        "caught": {
                            "line": line!(),
                            "col": column!(),
                            "file": file!(),
                        },
                        "err": formatted,
                        "backtrace": backtrace,
                    });
                });
                std::panic::resume_unwind(e);
            }
        }
    };
}
#[macro_export]
macro_rules! report_panics_async {
    ($($body:tt)+) => {
        match futures::FutureExt::catch_unwind(std::panic::AssertUnwindSafe(async { $($body)+ })).await {
            Ok(v) => v,
            Err(e) => {
                let formatted = if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "<unable to read panic message>".to_string()
                };
                let backtrace = $crate::get_backtrace();

                $crate::error!("Caught panic in handler:\n{formatted}\n{backtrace}");
                $crate::report!("Caught panic in handler": {
                    "caught": {
                        "line": line!(),
                        "col": column!(),
                        "file": file!(),
                    },
                    "err": formatted,
                    "backtrace": backtrace,
                });
                std::panic::resume_unwind(e);
            }
        }
    };
}
