use std::sync::Arc;
use whatsapp_rust::Client;
use whatsapp_rust::wacore_binary::Jid;

pub async fn resolve_to_pn(client: &Arc<Client>, jid: &Jid) -> Jid {
    if !jid.is_lid() {
        return jid.to_non_ad();
    }

    if let Ok(Some(entry)) = client.get_lid_pn_entry(jid).await {
        return Jid::pn(entry.phone_number.as_ref());
    }

    if let Ok(user_info_map) = client.contacts().get_user_info(&[jid.clone()]).await {
        if let Some(info) = user_info_map.get(jid) {
            if let Some(pn) = &info.pn_jid {
                return pn.to_non_ad();
            }
        }
    }

    jid.to_non_ad()
}

pub async fn get_phone_number(client: &Arc<Client>, jid: &Jid) -> String {
    let resolved = resolve_to_pn(client, jid).await;
    resolved.user.to_string()
}

pub async fn format_jid_display(client: &Arc<Client>, jid: &Jid) -> String {
    let resolved = resolve_to_pn(client, jid).await;
    resolved.to_string()
}
