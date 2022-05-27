docker run -it -p 3002:3000 -e REACT_APP_PRIMARY_COLOR='#b84aff' -e REACT_APP_SECONDARY_COLOR='#ffffff12' -e REACT_APP_BODY_COLOR='#16171b' -e REACT_APP_TEXT_COLOR='#ffffff'-e REACT_APP_ALLOWED_CHAINS='Ethereum:0x56a86d648c435dc707c8405b78e2ae8eb4e60ba4,Polygon:0x980111ae1b84e50222c8843e3a7a038f36fecd2b' 0612020/stackos-token-bridge:v0.0.3


docker run -it -p 3002:3000 -e REACT_APP_PRIMARY_COLOR="#b84aff" -e REACT_APP_SECONDARY_COLOR="#ffffff12" -e REACT_APP_BODY_COLOR="#16171b" -e REACT_APP_TEXT_COLOR="#ffffff" -e REACT_APP_ALLOWED_CHAINS="Ethereum:0x56a86d648c435dc707c8405b78e2ae8eb4e60ba4,Polygon:0x980111ae1b84e50222c8843e3a7a038f36fecd2b" 0612020/stackos-token-bridge:v0.0.3



docker run -it -p 3004:3000 -e REACT_APP_PRIMARY_COLOR="#b84aff" -e REACT_APP_SECONDARY_COLOR="#ffffff12" -e REACT_APP_BODY_COLOR="#16171b" -e REACT_APP_TEXT_COLOR="#ffffff" -e REACT_APP_LOGO="https://www.linkpicture.com/q/logo_12.svg" -e REACT_APP_TITLE="Token Bridge" -e REACT_APP_SUBTITLE="Portal is a bridge that" -e REACT_APP_LINK_NAME="StackOS" -e REACT_APP_LINK_ADDRESS="https://www.home.stackos.io/" -e REACT_APP_CLUSTER="mainnet" -e REACT_APP_ALLOWED_CHAINS="Ethereum:0x56a86d648c435dc707c8405b78e2ae8eb4e60ba4,Polygon:0x980111ae1b84e50222c8843e3a7a038f36fecd2b" -e REACT_APP_COVALENT_API_KEY="ckey_ac8f912a507e48b2bdd57a6c795" 0612020/stackos-token-bridge:v0.0.4