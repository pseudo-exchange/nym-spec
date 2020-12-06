use near_sdk::{env};

// convenient logger
#[macro_export]
macro_rules! logger {
  ($($arg:tt)*) => ({
    let log_message = format!($($arg)*);
    let log = log_message.as_bytes();
    env::log(&log)
  })
}

fn only_admin() {
    // require only admins
    assert_eq!(
        &env::current_account_id(),
        &env::signer_account_id(),
        "Only owner can execute this fn",
    )
}
