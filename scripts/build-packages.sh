#!/bin/sh
set -e
mkdir -p /out
nfpm pkg --packager deb --target /out/griffon-0.3.2.deb
nfpm pkg --packager rpm --target /out/griffon-0.3.2.rpm