# portal-agent

The in-guest agent for [Portal](https://github.com/portal-project/portal), which runs Linux
distros on Apple Silicon Macs via Apple's Virtualization.framework.

portal-agent runs inside the Linux VM and communicates with the macOS host over a vsock
channel using JSON-RPC 2.0 (a single channel, no networking required). It is responsible for:

- clipboard sync between guest and host
- display resolution and DPI handling
- bridging Wayland guest windows to native macOS windows
- mounting shared folders exposed by the host
- reporting guest health back to the host

portal-agent is built as a static binary so the same build runs unmodified across the curated
distros Portal supports: Arch ARM, Fedora, Ubuntu, Debian, and NixOS.

## License

Apache License 2.0. See [LICENSE](LICENSE).
