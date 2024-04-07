macro_rules! clap_error {
  ($msg:literal $(,)?) => (
    $crate::cli::macros::clap_error!(kind: clap::error::ErrorKind::ValueValidation, $msg)
  );
  (kind: $kind:expr, $msg:literal $(,)?) => ({
    let mut cmd = $crate::cli::CliArgs::command();
      cmd.error(
        $kind,
        core::format_args!($msg)
      ).exit();
  });
  ($fmt:expr, $($arg:tt)*) => (
    $crate::cli::macros::clap_error!(kind: clap::error::ErrorKind::ValueValidation, $fmt, $($arg)*)
  );
  (kind: $kind:expr, $fmt:expr, $($arg:tt)*) => ({
      let mut cmd = $crate::cli::CliArgs::command();
      cmd.error(
        $kind,
        format!($fmt, $($arg)*)
      ).exit();
  });
}

pub(crate) use clap_error;
