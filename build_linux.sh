docker run --rm \
  -v $(pwd):/app \
  -v cargo_index:/usr/local/cargo \
  -e ROGALIK_ASSETS=/app/assets \
  --user $(id -u):$(id -g) \
  -t rust_linux
