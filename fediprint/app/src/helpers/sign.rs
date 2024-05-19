use anyhow::bail;
use base64::{engine::general_purpose, Engine as _};
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::sha;
use openssl::sign::Signer;
use url::Url;

fn sign_data_to_base64(key: PKey<Private>, data: &[u8]) -> anyhow::Result<String> {
    let mut signer = Signer::new(MessageDigest::sha256(), &key)?;
    signer.update(data)?;
    let signature_vec = signer.sign_to_vec()?;
    Ok(general_purpose::STANDARD.encode(signature_vec))
}

fn get_now_in_format() -> String {
    let dt = chrono::Local::now();
    dt.format("%d %b %Y %H:%M:%S %Z").to_string() // 18 Dec 2019 10:08:46 GMT
}

pub fn sign_get_request_by_details(
    path: &str,
    host: &str,
    key: PKey<Private>,
    key_id: String,
) -> anyhow::Result<String> {
    // https://docs.joinmastodon.org/spec/security/#http-sign
    let headers = "(request-target) host date";
    let now = get_now_in_format();
    let data_to_sign = format!("(request-target): get {path}\nhost: {host}\ndate: {now}");
    let signature = sign_data_to_base64(key, data_to_sign.as_ref())?;
    Ok(format!(
        "keyId=\"{key_id}\",headers=\"{headers}\",signature=\"{signature}\""
    ))
}

pub fn sign_get_request_by_url(
    url: String,
    key: PKey<Private>,
    key_id: String,
) -> anyhow::Result<String> {
    let url = Url::parse(&url)?;
    let host = match url.host_str() {
        Some(d) => d,
        None => bail!("Host is none"),
    };
    let path = url.path();
    sign_get_request_by_details(path, host, key, key_id)
}

pub fn sign_post_request_with_hash(
    path: &str,
    host: &str,
    body_hash: &str,
    key: PKey<Private>,
    key_id: String,
) -> anyhow::Result<String> {
    let headers = "(request-target) host date digest";
    let now = get_now_in_format();
    let data_to_sign = format!(
        "(request-target): get {path}\nhost: {host}\ndate: {now}\ndigest: sha-256={body_hash}"
    );
    let signature = sign_data_to_base64(key, data_to_sign.as_ref())?;
    Ok(format!(
        "keyId=\"{key_id}\",headers=\"{headers}\",signature=\"{signature}\""
    ))
}

pub fn sign_post_request_with_body(
    url: &str,
    body_hash: &[u8],
    key: PKey<Private>,
    key_id: String,
) -> anyhow::Result<(String, String)> {
    // (Signature, Digest) -header returns
    let mut hasher = sha::Sha256::new();
    hasher.update(body_hash);
    let hash = general_purpose::STANDARD.encode(hasher.finish());
    let url = Url::parse(url)?;
    let host = match url.host_str() {
        Some(d) => d,
        None => bail!("Host is none"),
    };
    let path = url.path();
    Ok((
        sign_post_request_with_hash(path, host, &hash, key, key_id)?,
        format!("sha-256={hash}"),
    ))
}
