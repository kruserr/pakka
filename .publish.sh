#!/usr/bin/env bash

set -Eeuo pipefail

cargo publish -p pakka
cargo publish -p pakka-gui
