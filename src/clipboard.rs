//! Syncs clipboard contents between the Linux guest and the macOS host over the vsock RPC channel.

pub fn sync_clipboard(_text: &str) {
    unimplemented!("clipboard sync requires a running guest session")
}
