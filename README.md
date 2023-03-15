# TFchain &middot; ![Build & Tests](https://github.com/threefoldtech/tfchain/actions/workflows/build_test.yaml/badge.svg)

<p align="center">
  <img height="50%" width="50%" src="./.maintain/header.png">
</p>

🚧 Tfchain 3 is a new version of [tfchain](https://github.com/threefoldtech/tfchain) - caution before using 🚧

Threefold blockchain (3) serves as a registry for Nodes, Farms, Digital Twins and Deployment contracts.
It is the backbone of [ZOS](https://github.com/threefoldtech/zos) and other components.

## Docs

see [docs](./docs/readme.md) for more information on how to work with this component.

## Modules list

- [Tfgrid Module](./pallets/pallet-tfgrid/readme.md): registry for Nodes / Farms / Twins
- [Smart Contract Module](./pallets/pallet-grid-contracts/readme.md): node and rent contracts
- [Dao Module](<(./pallets/pallet-dao/readme.md)>): voting on proposals that impact the system for farmers. See [dao](./docs/misc/minimal_DAO.md) for more info.
- [Kvstore Module](./pallets/pallet-kvstore/readme.md): key value store for deployment information
- [TFT Price Module](./pallets/pallet-tft-price/readme.md): TFT price oracle. See [price](./docs/misc/price.md) for more info.
- other less mentionable modules..

## Deployed instances

None yet
