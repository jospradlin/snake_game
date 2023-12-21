## Rust/WebAssembly-based Snake Game

The following code is a Rust/WebAssembly-based Snake Game used as a learning tool for the new technology.  This code can be build, containerized, and further "distroless" containerized and deployed to Docker, GCP CloudRun, AWS Lambda, etc.

## Pre-Requirements

1.  Ensure that you've properly installed Rust, Cargo, and other standard Rust tools.
2.  Ensure NPM and NodeJS (I used 20.10 in this project).
3.  Ensure Rust-specific WebAssembly tools have been installed.
4.  Ensure Docker Tools are installed

## Run in Development Mode

1.  Clone the repository.
2.  cd into the /www directory.
3.  $  npm install
4.  $  wasm-pack build --target web  // This builds the Rust code to WebAssembly found in the ./pkg sub-folder
5.  $  npm run dev // Runs the Development server

## Build for Production
1.  Clone the repository.
2.  cd into the /www directory.
3.  $  npm install
4.  $  wasm-pack build --target web  // This builds the Rust code to WebAssembly found in the ./pkg sub-folder
5.  $  cd back to /(root) directory
6.  $  npm run prebuild
6.  $  npm run build
7.  $  npm install


## Post-Build Start Server/Application
1.  $  npm start
2.  Point your web browser to http://localhost:3000


## Build the Docker Container
1.  Clone the repository.
2.  docker build -t <some repository host or prefix>snakegame .
3.  docker tag snakegame snakegame:<some tag>


Enjoy!