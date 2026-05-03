// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.0;

contract Build {
    string public constant command = "cargo build --locked";
    string public constant cargoTargetDir = "target";
}
