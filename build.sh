#!/bin/sh
set -e



cd addition && ./build.sh && cd ..
cd multiplication && ./build.sh && cd ..
cd Button-calling && ./build.sh && cd ..
