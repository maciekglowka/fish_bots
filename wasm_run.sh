set -e
./wasm_build.sh
cd output/wasm
python3 -m http.server
