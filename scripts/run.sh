#!/bin/bash

package=www

rm -rf "$package/dist"

wasm-pack build && (cd $package && npm run build)
(cd $package && npm run start -- --port 9000)

echo "open http://localhost:9000/"
