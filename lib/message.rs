use std::fs;
use std::path::Path;
use std::sync::Arc;
use whatsapp_rust::buffa::MessageField;
use whatsapp_rust::Client;
use whatsapp_rust::wacore::download::MediaType;
use whatsapp_rust::wacore_binary::Jid;
use whatsapp_rust::waproto::whatsapp as wa;

pub async fn send_text(
    client: &Arc<Client>,
    chat: Jid,
    content: impl Into<String>,
) -> Result<(), anyhow::Error> {
    let mut message = wa::Message::default();
    message.conversation = Some(content.into());
    client.send_message(chat, message).await?;
    Ok(())
}

pub async fn send_reply(
    client: &Arc<Client>,
    chat: Jid,
    content: impl Into<String>,
    stanza_id: String,
    participant: Option<String>,
    quoted_message: wa::Message,
) -> Result<(), anyhow::Error> {
    let mut context_info = wa::ContextInfo::default();
    context_info.stanza_id = Some(stanza_id);
    context_info.participant = participant;
    context_info.quoted_message = MessageField::some(quoted_message);

    let mut ext = wa::message::ExtendedTextMessage::default();
    ext.text = Some(content.into());
    ext.context_info = MessageField::some(context_info);

    let mut message = wa::Message::default();
    message.extended_text_message = MessageField::some(ext);

    client.send_message(chat, message).await?;
    Ok(())
}

pub async fn send_image_bytes(
    client: &Arc<Client>,
    chat: Jid,
    image_bytes: Vec<u8>,
    mimetype: &str,
    caption: Option<String>,
) -> Result<(), anyhow::Error> {
    let upload = client
        .upload(image_bytes, MediaType::Image, Default::default())
        .await?;

    let mut image = wa::message::ImageMessage::default();
    image.url = Some(upload.url);
    image.direct_path = Some(upload.direct_path);
    image.media_key = Some(upload.media_key.to_vec());
    image.file_enc_sha256 = Some(upload.file_enc_sha256.to_vec());
    image.file_sha256 = Some(upload.file_sha256.to_vec());
    image.file_length = Some(upload.file_length);
    image.media_key_timestamp = Some(upload.media_key_timestamp);
    image.mimetype = Some(mimetype.to_string());
    image.caption = caption;

    let mut message = wa::Message::default();
    message.image_message = MessageField::some(image);

    client.send_message(chat, message).await?;
    Ok(())
}

pub async fn send_image_file(
    client: &Arc<Client>,
    chat: Jid,
    file_path: impl AsRef<Path>,
    caption: Option<String>,
) -> Result<(), anyhow::Error> {
    let bytes = fs::read(file_path)?;
    send_image_bytes(client, chat, bytes, "image/jpeg", caption).await
}

pub async fn send_reaction(
    client: &Arc<Client>,
    chat: Jid,
    stanza_id: String,
    participant: Option<String>,
    from_me: bool,
    emoji: &str,
) -> Result<(), anyhow::Error> {
    let mut key = wa::MessageKey::default();
    key.remote_jid = Some(chat.to_string());
    key.id = Some(stanza_id);
    key.from_me = Some(from_me);
    key.participant = participant;

    let mut reaction = wa::message::ReactionMessage::default();
    reaction.key = MessageField::some(key);
    reaction.text = Some(emoji.to_string());
    reaction.sender_timestamp_ms = Some(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64,
    );

    let mut message = wa::Message::default();
    message.reaction_message = MessageField::some(reaction);

    client.send_message(chat, message).await?;
    Ok(())
}
