# Example Token Bridge UI

## Prerequisites

- Docker
- NodeJS v14+
- NPM v7.18+

Run the following from the root of this repo

```bash
DOCKER_BUILDKIT=1 docker build --target node-export -f Dockerfile.proto -o type=local,dest=. .
DOCKER_BUILDKIT=1 docker build -f solana/Dockerfile.wasm -o type=local,dest=. solana
npm ci --prefix ethereum
npm ci --prefix sdk/js
npm run build --prefix sdk/js
```

The remaining steps can be run from this folder

## Install

```bash
npm ci
```

## Develop

```bash
npm start
```

## Build for local tilt network

```bash
npm run build
```

## Build for testnet

```bash
REACT_APP_CLUSTER=testnet npm run build
```

## Build for mainnet

```bash
REACT_APP_CLUSTER=mainnet REACT_APP_COVALENT_API_KEY=YOUR_API_KEY REACT_APP_SOLANA_API_URL=YOUR_CUSTOM_RPC npm run build
```

## Test Server

```bash
npx serve -s build
```

## Environment Variables (optional)

Create `.env` from the sample file, then add your Covalent API key:

```bash
cp .env.sample .env
```

## Run Project
```bash
npm i
npm start
```

### Custom Design And Text Changes Example on .env

- REACT_APP_PRIMARY_COLOR: Env for used in button and tab.
- REACT_APP_SECONDARY_COLOR: Env for used in div.
- REACT_APP_BODY_COLOR: Env for used in body color.
- REACT_APP_TEXT_COLOR: Env for used in text color.
- REACT_APP_LOGO: Env for logo image.
- REACT_APP_TITLE: Env for main title.
- REACT_APP_SUBTITLE: Env for subtitle.
- REACT_APP_LINK_NAME: Env for link name.
- REACT_APP_LINK_ADDRESS: Env for link address.

```bash
REACT_APP_PRIMARY_COLOR="#2abfff"
REACT_APP_SECONDARY_COLOR="#ffffff12"
REACT_APP_BODY_COLOR="#16171b"
REACT_APP_TEXT_COLOR="#ffffff"
REACT_APP_LOGO="cloud image link"
REACT_APP_TITLE="Token Bridge"
REACT_APP_SUBTITLE="Token Bridge"
REACT_APP_LINK_NAME=""
REACT_APP_LINK_ADDRESS=""
```

###Add Custom Allowed Chains
You can add custom chains using chains fullname(ex: chain1-name:token-address)

```bash
REACT_APP_ALLOWED_CHAINS="Ethereum:0x56a86d648c435dc707c8405b78e2ae8eb4e60ba4,Binance Smart Chain:0x6855f7bb6287f94ddcc8915e37e73a3c9fee5cf3,Polygon:0x980111ae1b84e50222c8843e3a7a038f36fecd2b"
```

The supported chains listed below here:

```bash
Aurora
Avalanche
Binance Smart Chain
Ethereum
Fantom
Oasis
Polygon
Solana
Terra
```

### Docker Build Locally

```bash
docker build -t stackos-token-bridge:tag .
```

### Docker Run Image with env variable

```bash
docker run -it -p 3001:3000 -e REACT_APP_PRIMARY_COLOR="#2abfff" -e REACT_APP_ALLOWED_CHAINS='Ethereum:0x56a86d648c435dc707c8405b78e2ae8eb4e60ba4,Binance Smart Chain:0x6855f7bb6287f94ddcc8915e37e73a3c9fee5cf3,Polygon:0x980111ae1b84e50222c8843e3a7a038f36fecd2b' stackos-token-bridge:v0.0.3