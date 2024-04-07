-- solana logs
-- solana-test-validator -r

rm -r node_modules
rm -r target

nvm use 16
yarn install

anchor build
anchor run test