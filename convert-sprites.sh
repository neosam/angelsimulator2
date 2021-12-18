#!/usr/bin/env bash

mkdir assets/sprites/
cd resources/sprites
for file in *.svg; do convert -background none -depth 8 $file ../../assets/sprites/"`basename $file .svg`.png"; done
cd ../..
