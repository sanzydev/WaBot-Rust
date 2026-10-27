use whatsapp_rust::pair_code::PairCodeOptions;
use whatsapp_rust::wacore::companion_reg::CompanionWebClientType;
use whatsapp_rust::wacore::types::events::{
    PairSuccess, PairingCode, PairingCodeError, PairingCodeRefresh,
};

pub fn create_pair_options(phone_number: String) -> PairCodeOptions {
    PairCodeOptions {
        phone_number,
        show_push_notification: true,
        custom_code: None,
        platform_id: Some(CompanionWebClientType::Chrome),
        display_os: Some("Windows".to_string()),
        ..Default::default()
    }
}

pub fn display_pairing_code(code: &PairingCode) {
    println!("Pairing code: {}", code.code);
}

pub fn handle_pairing_error(err: &PairingCodeError) {
    eprintln!("Pairing error: {}", err.error);
}

pub fn handle_pairing_refresh(refresh: &PairingCodeRefresh) {
    println!("Pairing refresh: force_manual={}", refresh.force_manual);
}

pub fn handle_pair_success(info: &PairSuccess) {
    println!("Paired: {}", info.id);
}
