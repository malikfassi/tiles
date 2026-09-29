# Déploiement sur le Cosmos Hub — vérifié en pratique (T-014)

Date : 2026-09-29. **Mis à jour après un déploiement réel sur le testnet.** La version
précédente de cette note concluait, à partir du code source de Gaia, que l'upload était
réservé à la gouvernance. **La pratique l'a contredite.** Ne pas refaire cette recherche.

## Réponse : oui, on peut uploader un wasm sur le testnet, sans gouvernance

Preuve lue directement sur la chaîne, pas déduite :

```
$ gaiad query wasm params --node <rpc provider>
{"code_upload_access":{"permission":"Everybody","addresses":[]},
 "instantiate_default_permission":"Everybody"}
```

`code_upload_access: Everybody` sur le testnet `provider`. Un contrat custom s'y déploie
avec deux transactions ordinaires, **sans proposition de gouvernance**.

Déploiement réalisé le 2026-09-29 :

| Élément | Valeur |
|---|---|
| Code ID | **738** |
| Checksum | `424d77883143430d490763063b03f392fa02a028939d69596753a52af8a05f8c` |
| Contrat | `cosmos15t68c2h8qkvwkmkvksy9yn3fg2pyj6ec8cenad9svgjc2q96cjcq538m53` |
| Tx store | `B760B8E3D90D6110A50F345DDE0393D83F09C009118A26BDA50A6349F3A780C3` (gas 5 810 504) |
| Tx instantiate | `11414A6B4805ABEF05E022F7F33E392C7FD320603F0EA980F427D919E7F82B44` |
| Registre | `scripts/state/02_deploy_contracts.state` |

## Pourquoi mon raisonnement précédent était faux

Gaia construit son keeper wasm avec l'autorité de gouvernance
(`NewKeeper(..., govtypes.ModuleName, ...)`), ce qui **permet** la permission gouvernée mais
ne l'impose pas : le paramètre on-chain décide. Le mainnet du Hub a effectivement
`code_upload_access` restreint ; **le testnet, non**. J'ai généralisé du code au
comportement d'une chaîne, sans interroger la chaîne.

Leçon retenue : sur une question de permission on-chain, `gaiad query wasm params` sur la
chaîne visée est la seule source qui compte.

## Contraintes réelles du testnet `provider`

| Élément | Valeur |
|---|---|
| Chain-id | `provider` |
| Denom | `uatom` (comme le mainnet) |
| Binaire | `gaiad` ; testnet en Gaia v28.2.0 |
| RPC | `https://rpc.provider-sentry-01.hub-testnet.polypore.xyz` |
| Faucet | `https://faucet.polypore.xyz/request?address=<addr>&chain=provider` (10 ATOM/tx) |
| Explorateur | `explorer.polypore.xyz/provider`, Mintscan `ics-testnet-provider` |

## Contraintes qui ont réellement bloqué, et leurs solutions

1. **Pas de binaire arm64 après Gaia v27.3.0**, et les binaires darwin-amd64 refusent de se
   charger sur macOS 27 (`__DATA_CONST segment missing SG_READ_ONLY flag`). Solution : image
   Docker `ghcr.io/cosmos/gaia:v28.0.0` en `linux/amd64`. Voir `scripts/gaiad-docker.sh`
   et le shim `~/.local/bin/gaiad` : le conteneur tourne avec l'UID de l'hôte (sinon le
   montage est en lecture seule et les clés créées sont perdues), monte `~/.gaia` (keyring)
   et le répertoire courant (pour `tx wasm store <fichier>`).
2. **Taille du wasm.** Le RPC refuse un corps de requête trop gros : un wasm non optimisé de
   2,68 Mo échoue avec `http: request body too large`. Solution :
   `cosmwasm/rust-optimizer:0.17.0` (pas 0.16.1, qui embarque Cargo 1.81 et échoue sur
   `zeroize_derive 1.5.0` avec `edition2024`). 2,68 Mo → **850 Ko**.
3. Le `Cargo.toml` doit **ne pas** contenir `[workspace]` sans `members` : l'optimiseur
   refuse alors de builder (`Cargo.toml contains a workspace key but has no workspace
   members`) et recopie silencieusement le wasm existant, ce qui donne un faux succès
   (même checksum en entrée et en sortie).
4. `strip = true` dans le profil release produit un wasm rejeté à l'upload
   (`Wasm bytecode could not be deserialized: zero byte expected`). Garder `strip = false`.
5. Le linker wasm refuse les symboles d'hôte non résolus depuis Rust 1.87 : il faut
   `-C link-arg=--allow-undefined` (dans `.cargo/config.toml`) et les features
   `iterator` + `cosmwasm_2_2` sur `cosmwasm-std`.

## Limites connues

- Le **mainnet** (`cosmoshub-4`) reste à vérifier : sa permission d'upload peut différer du
  testnet. Contrôler `gaiad query wasm params` sur un nœud mainnet avant de conclure.
- Le détail des whitelist Stargaze 2.0 (marketplace, launchpad, studio) n'est pas couvert.
