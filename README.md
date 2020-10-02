# Orthrus

Orthrus is an endpoint for [Cerberus](https://github.com/elysium-suite/cerberus) that sends Aeacus releases to an authenticated client. It's written in Rust with the Rocket.rs crate.

## How to use

- Replace `aeacus-linux.zip` and `aeacus-win32.zip` with actual zipped production binaries of aeacus.
- Set the password required to download the engine inside `.env.example` by replacing `my_secure_key` with your chosen password.
