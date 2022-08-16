#!/bin/bash
set -euv
sea generate entity -o src/models --with-serde both
chown 1000:1000 -R src/models
