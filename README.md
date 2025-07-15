# daily game blurb reducer

A simple tool that combines multiple daily game blurbs into a forum post.

## Building for itch.io

In the `app` folder, run `trunk build --release --no-sri`; then go into the generated `dist` folder and change all the paths to the WASM and JS file to relative ones (starting `./` instead of `/`). Zip the contents of the folder up, and upload them.
