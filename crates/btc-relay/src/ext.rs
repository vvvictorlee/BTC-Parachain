#[cfg(test)]
use mocktopus::macros::mockable;

#[cfg_attr(test, mockable)]
pub(crate) mod security {
    #[cfg(test)]
    use security::types::ErrorCode;
    #[cfg(test)]
    use security::types::StatusCode;

    use frame_support::dispatch::DispatchError;
    type UnitResult = Result<(), DispatchError>;

    #[cfg(test)]
    pub fn ensure_parachain_status_running<T: security::Trait>() -> UnitResult {
        <security::Module<T>>::ensure_parachain_status_running()
    }

    pub fn ensure_parachain_status_not_shutdown<T: security::Trait>() -> UnitResult {
        <security::Module<T>>::ensure_parachain_status_not_shutdown()
    }

    pub fn is_parachain_error_invalid_btcrelay<T: security::Trait>() -> bool {
        <security::Module<T>>::is_parachain_error_invalid_btcrelay()
    }

    pub fn is_parachain_error_no_data_btcrelay<T: security::Trait>() -> bool {
        <security::Module<T>>::is_parachain_error_no_data_btcrelay()
    }

    pub fn recover_from_btc_relay_failure<T: security::Trait>() -> UnitResult {
        <security::Module<T>>::recover_from_btc_relay_failure()
    }

    #[cfg(test)]
    pub fn set_parachain_status<T: security::Trait>(status: StatusCode) -> () {
        <security::Module<T>>::set_parachain_status(status)
    }

    #[cfg(test)]
    pub fn insert_error<T: security::Trait>(error: ErrorCode) -> () {
        <security::Module<T>>::insert_error(error)
    }
}
