//! Real product logos (fetched from each project's site), embedded as base64
//! PNG so the cards carry authentic branding with no runtime dependency.

/// Base64 PNG for a project's logo, if we have one.
pub fn logo(name: &str) -> Option<&'static str> {
    Some(match name {
        "dup" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logos/dup.b64")),
        "cavemode" => {
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logos/cavemode.b64"))
        }
        "cattype" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logos/cattype.b64")),
        "devpon" => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logos/devpon.b64")),
        _ => return None,
    })
}
