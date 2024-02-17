
![calib_1](https://github.com/keeglunn/experimentsRS/assets/3039282/ea1b72b5-fbd3-4552-a11b-d23e927cd685)


### Examples
`cargo run --example crunchify_1`

### ffmpeg
ffmpeg -framerate 30 -pattern_type glob -i '*.png' -c:v libx264 -pix_fmt yuv420p out.mp4