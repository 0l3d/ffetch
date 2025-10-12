//! # FFetch - Fast System Information Fetcher
//!
//! A lightweight and fast system information library for Linux systems.
//! This crate provides functions to retrieve various system information
//! such as CPU details, memory usage, GPU information, and more.
//!
//! ## Features
//!
//! - **Hardware Information**: CPU name, architecture, GPU details
//! - **System Information**: OS name, kernel version, hostname, uptime
//! - **Desktop Environment**: DE, compositor, terminal, shell information
//! - **Memory & Storage**: RAM usage, disk usage for specific mount points
//! - **Package Management**: Count packages from various package managers
//! - **Display Information**: Monitor details with resolution and refresh rate
//!
//! ## Quick Start
//!
//! ```rust
//! use ffetch::*;
//!
//! // Get basic system information
//! let os_name = get_os_name();
//! let kernel = get_kernel_version();
//! let username = get_username();
//! let hostname = get_hostname();
//!
//! println!("{}@{}", username, hostname);
//! println!("OS: {}", os_name);
//! println!("Kernel: {}", kernel);
//! ```
//!
//! ## Hardware Information Example
//!
//! ```rust
//! use ffetch::*;
//!
//! // Get hardware details
//! let cpu = get_cpu_name();
//! let gpu = get_gpu();
//! let memory = get_memory();
//! let arch = get_cpu_arch();
//!
//! println!("CPU: {}", cpu);
//! println!("GPU: {}", gpu);
//! println!("Memory: {}", memory);
//! println!("Architecture: {}", arch);
//! ```
//!
//! ## Desktop Environment Example
//!
//! ```rust
//! use ffetch::*;
//!
//! // Get desktop environment information
//! let de = get_desktop_env();
//! let shell = get_shell();
//! let terminal = get_terminal();
//!
//! println!("DE: {}", de);
//! println!("Shell: {}", shell);
//! println!("Terminal: {}", terminal);
//! ```
//!
//! ## Platform Support
//!
//! Currently supports **Linux** systems only. The library reads from:
//! - `/proc/version`, `/proc/cpuinfo`, `/proc/meminfo`
//! - `/etc/os-release`, `/etc/hostname`
//! - Various environment variables
//! - System commands like `lspci`, `df`, `xprop`
//!
//! ## Dependencies
//!
//! This crate requires some system utilities to be installed:
//! - `lspci` for GPU information
//! - `xprop` for terminal detection (X11 environments)
//! - `df` for disk usage information
//! - Package managers for package counting
pub mod ffetch;
pub use ffetch::*;
