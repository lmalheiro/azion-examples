# emcc -s ASYNCIFY -lembind src/hello.cpp -s SINGLE_FILE=1 -o dist/hello.js --extern-post-js js/bootstrap.js
emcc -s SIDE_MODULE -s ASYNCIFY -lembind src/hello.cpp -s SINGLE_FILE=1 -o dist/a.out.wasm 

