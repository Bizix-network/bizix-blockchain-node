module.exports = {
  apps : [{
    name: "bizix-node",
    script: "/home/bizix/bizix-blockchain-node/target/release/bizix-node",
    args: "--dev --rpc-external --rpc-cors all --pruning=archive",
    interpreter: "none",
    autorestart: true,
    watch: false,
    max_memory_restart: "1G",
    env: {
      NODE_ENV: "development",
    },
  }]
}
