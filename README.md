# ethereum-transactions-crawler

**Note: a little edit has been added, I forgot to update some files when I uploaded them**

**The output should look like this:**

![Screenshot_20230630_153954](https://github.com/qalassa/ethereum-transactions-crawler/assets/109701506/ac952a84-516a-45bd-be0e-a370b436cd0b)

Pre-requisites: Rust and Node.js.

Also, install 'http-server' globally via NPM by running:
```
npm install --global http-server
```

The backend, frontend, and static directories need to be run simultaneously, and in different terminal sessions.

A. Regarding the backend:
Navigate to it from the project root via
```
cd backend
```
and run through:
```
cargo run RUST_LOG=info
```
this will run the backend on the 8080 port.

B. Regarding the frontend:
Navigate to it from the project root via
```
cd frontend
```
and run through:
```
cargo run RUST_LOG=info
```
this will run the frontend on the 3030 port

C. Regarding static:
Navigate to it from the project root via:
```
cd static
```
and run through:
```
http-server . -p <port number>
```
Replace <port number> with any port number other than 8080 and 3030.
Now, click any of the two generated URLs in order to get the app running in the browser.

