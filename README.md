# ethereum-transactions-crawler

Pre-requisites: Rust and Node.js
Also install 'http-server' globally via NPM by running "npm install --global http-server" in your terminal.

The backend, frontend, and static directories need to be run simultaneously, and in a different terminal sessions.

A. Regarding the backend:
Navigat to it from the project root via 'cd backend' and run through 'cargo run RUST_LOG=info'
this will run the backend on 8080 port

B. Regarding the frontend:
Navigat to it from the project root via 'cd frontend' and run through 'cargo run RUST_LOG=info'
this will run the frontend on 3030 port

C. Regarding static:
Navigat to it from the project root via 'cd static' and run through 'http-server . -p <port number>'. Replace <port number> with any port number other than 8080 and 3030.
Now, click any of the two generated URLs in order to get the app running into the browser.

