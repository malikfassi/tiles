# Déployer un contrat custom sur le Cosmos Hub (testnet et mainnet)

Date : 2026-09-28. Question de T-013 : le testnet du Cosmos Hub accepte-t-il un upload de wasm
**librement**, comme le croyait la note d'écosystème ? **Réponse : non.** Sources : dépôt `cosmos/gaia`,
`cosmos/testnets`, docs officielles. **Ne pas refaire cette recherche.**

## Le module wasm existe bien dans Gaia

`app/app.go` de Gaia (v29) importe le module :

```go
"github.com/CosmWasm/wasmd/x/wasm"
wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
```

Donc non : le Cosmos Hub **n'est pas dépourvu de CosmWasm**, contrairement à ce que laisserait croire
le genesis du testnet `provider` (daté de 2023, antérieur à l'ajout du module). C'est une correction de
la note `ecosysteme-cosmos-2026.md`.

## Mais l'upload est réservé à la gouvernance

`app/keepers/keepers.go` construit le keeper avec **l'autorité de gouvernance** :

```go
wasmkeeper.NewKeeper(..., authtypes.NewModuleAddress(govtypes.ModuleName).String(), wasmOpts...)
```

C'est la configuration d'une chaîne **permissionnée** : `store_code` passe par une proposition de
gouvernance, pas par une transaction libre. Le testnet `provider` utilise la **même** application
`gaiad` que le mainnet, donc la même restriction. « Testnet » ne veut pas dire « upload libre ».

## Ce que ça change pour tiles

Un contrat custom comme tiles **ne peut pas** être uploadé librement, ni sur le testnet ni sur le mainnet :
il faut une proposition de gouvernance (mainnet) ou l'accord des coordinateurs du testnet.

Voie de repli, **sans dépendre de personne** : la même application `gaiad` en local via Docker
(chaîne locale à un validateur), ou `wasmd` local. Les scripts `scripts/*.sh` ne changent pas de forme :
ils visent `gaiad`, seul le `CHAIN_ID`/`NODE_URL` change.

## Faits utiles sur le testnet `provider`

| Élément | Valeur |
|---|---|
| Chain-id | `provider` |
| Denom | `uatom` (comme le mainnet : les scripts sont directement réutilisables) |
| Binaire | `gaiad`, Gaia `v28.2.0` (upgrade du 17 septembre 2026) |
| RPC | `https://rpc.provider-sentry-01.hub-testnet.polypore.xyz` |
| Faucet | `faucet.polypore.xyz`, ou Discord `#testnet-faucet` |
| Explorateur | `explorer.polypore.xyz/provider`, Mintscan `ics-testnet-provider` |
| Genesis | `cosmos/testnets`, `provider/provider-genesis.json` |

## Limites connues de cette note

- La permission exacte d'upload du testnet `provider` (paramètre on-chain `code_upload_access`) n'a pas pu
  être lue directement : elle vit dans le state de la chaîne, pas dans le genesis de 2023. L'inférence
  « même binaire, donc même restriction » est solide mais **à confirmer par une requête** une fois `gaiad`
  installé : `gaiad query wasm params --node <RPC>`.
- Les statuts de whitelist Stargaze 2.0 sur le Hub (marketplace, launchpad, studio) restent à confirmer
  auprès de l'équipe : ce sont des décisions de gouvernance, pas de la documentation technique.
