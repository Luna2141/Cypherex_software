Cypherex

A secure, decentralized, and offline-first password manager built on the Polkadot network. This open-source project enables users to manage passwords securely by leveraging blockchain technology, designed for use with Raspberry Pi Zero boards or USB drives.
----------
**Features**

Blockchain Integration: Store passwords securely on Polkadot's blockchain with minimal transaction fees.
Offline First: Operates as a standalone system on Raspberry Pi Zero boards or USB drives, ensuring privacy and security.
Rust-Powered: Built with Rust for performance, reliability, and safety.
Open Source: Free to use and modify under specific terms.
Tipping Supported: Show appreciation by tipping the developer for continued improvements and support.
----------
**How It Works**
Password Storage on Blockchain:
Passwords are encrypted and stored on the Polkadot blockchain, requiring a small fee slightly above the network’s base fee for each transaction. This ensures a secure and immutable storage solution.
----------

**Installation**
----------
Prerequisites
----------
A Raspberry Pi Zero board or a USB thumb drive.
Rust development environment installed.
Install Rust: ``https://www.rust-lang.org/tools/install``
----------
**Steps for Raspberry Pi Zero**
----------
Clone the repository to your Raspberry Pi:
bash
``git clone https://github.com/your-username/crypto-password-manager.git``
Navigate to the project directory:
``cd crypto-password-manager``
Build the project:
``cargo build --release``
Run the application:
``./target/release/crypto-password-manager``
----------
**Steps for USB Thumb Drive**
Clone the repository on your computer:
``git clone https://github.com/your-username/crypto-password-manager.git``
Compile the project for your target device:
``cargo build --release``
Transfer the compiled binaries to your USB drive:
``cp ./target/release/crypto-password-manager /path/to/usb/``
Plug the USB into your device and run the application.
----------
**Disclaimer**
Passwords are encrypted using various cryptography methods before being stored on the Polkadot blockchain. Users must have a Polkadot wallet and sufficient funds to cover the small transaction fees for storing passwords.
----------

**Support**
----------
If you find this project useful, consider tipping the developer:
Polkadot Address: your-polkadot-wallet-address

Your support helps keep the project alive and improves future updates!

**License**
----------
This project is licensed under the Creative Commons Attribution-NonCommercial 4.0 International License.
You are free to use and share this software for personal and non-commercial purposes.

For inquiries regarding commercial use or redistribution, please contact the author @luna24:matrix.org 