#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# Installation Dashboard

# Hardware Summary
lscpu | grep 'Model name'
lsblk -o NAME,SIZE,TYPE,MOUNTPOINT

# Feature Recommendations
grep 'Recommended Features' /var/log/anya-install.log

# Real-time Status
systemctl status anya-core --no-pager
