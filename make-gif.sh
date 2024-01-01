#!/bin/sh

set -xe

# Step 1: Convert frames to a video
ffmpeg -y -i data/frame%d.ppm -vf "fps=30,scale=640:-1:flags=lanczos" -c:v libvpx-vp9 -b:v 2M -pix_fmt yuv420p data/demo.mp4

# Step 2: Generate a palette from the video
ffmpeg -y -i data/demo.mp4 -vf "fps=30,scale=640:-1:flags=lanczos" -c:v libvpx-vp9 -b:v 0 -crf 30 -pix_fmt yuv420p data/palette.png

# Step 3: Create the GIF using the generated palette
ffmpeg -y -i data/demo.mp4 -i data/palette.png -filter_complex "fps=30,scale=640:-1:flags=lanczos[x];[x][1:v]paletteuse" data/demo.gif

