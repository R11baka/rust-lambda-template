# Template repository with AWS Lambda and rust

## Prerequisites

## Install nodejs
```shell
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
n install 20
```
### Install rust
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```
### Install cargo lambda (need for local testing)
Go to https://www.cargo-lambda.info/guide/getting-started.html
and for local development you can run
```shell
cargo lambda watch
```
in root directory

# How to deploy  ?
Clone this repo
and install dev dependencies
```shell
npm install
```
Now you can use sls in command line, so you can deploy lambda with command
```shell
npx sls deploy
```

### Why not just use cargo lamda ?
For me sls framework has more abilities than https://www.cargo-lambda.info/commands/deploy.html