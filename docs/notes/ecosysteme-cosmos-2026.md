# Écosystème Cosmos / Stargaze — état septembre 2026

Date : 2026-09-28. Sources vérifiées dans la session de reprise du projet. **Ne pas refaire cette recherche.**

## Stargaze : L1 morte, application sur le Cosmos Hub

- Stargaze a migré de sa chaîne propre (`stargaze-1`, testnet `elgafar-1`) vers le **Cosmos Hub** (`cosmoshub-4`).
  Timeline officielle : snapshot NFT le **17 mars 2026**, déploiement des apps sur le Hub juste après, claims NFT ouverts,
  migration des STARS en Q2, extinction de la L1 (docs.stargaze.zone/migration/whats-happening).
- Le **testnet `elgafar-1` n'existe plus**. Les code IDs 5008/5009/5010 et les contrats du dépôt `scripts/state/`
  sont historiques et définitivement inutilisables.
- Le dépôt `github.com/public-awesome/stargaze` est **archivé depuis le 11 juin 2026** (lecture seule).
- Conséquence directe : `sg721`, `sg721-base`, `sg-std`, `sg1/2/4`, `vending-minter`, `vending-factory` ne sont plus des cibles.

## Ce qu'est Stargaze 2.0 (docs officielles)

- Une app sur le Cosmos Hub : gas en **ATOM**, plus de staking STARS, wallets inchangés (`stars1…` → `cosmos1…`, mêmes clés).
- Toutes les collections sont des contrats **CW721** avec extension de collection :
  `collection_info_extension` → `royalty_info` (`payment_address`, `share` décimal, **maximum 10 %**).
- Royaumes exécutables référencés : transfer/send_nft/approve/revoke/burn, `update_collection_info`,
  queries `owner_of`, `nft_info`, `all_nft_info`, `tokens`, `all_tokens`, `num_tokens`, `get_collection_info_and_extension`.
- Création de collection : **Stargaze Studio 2.0**, wizard sans code, trois types (Vending/génératif, Open Edition, 1/1).
  Stockage **Arweave** permanent payé une fois (pas de testnet Arweave), répétition on-chain gratuite sur le testnet du Hub
  (le faucet ATOM est versé automatiquement), puis « promote to mainnet ».
- Les APIs/indexer Stargaze **ne sont pas publics** : pour des données indexées, passer par des events de contrat ou par l'équipe.
- Doc développeurs : `docs.stargaze.zone/developers/overview` et `/developers/cw721-reference`.

## Déployer un contrat custom sur le Cosmos Hub : permissionné

- Le module wasm du Hub est **permissionné** : uploader un code ID passe par une **proposition de gouvernance**
  (StoreCode puis Instantiate), avec dépôt en ATOM et campagne communautaire.
- Précédents de contrats custom acceptés (projets non-Stargaze) :
  - **ICNS** — service de noms `.cosmos` (2023) ;
  - **Electris** — jeu arcade 100 % on-chain (mi-2024) ;
  - **Epocha** — marchés de prédiction portés par Cryptocito (fin 2024) ;
  - **Stargaze 2.0 lui-même** — CW721 + marketplace + launchpad + studio whitelistés courant 2026.
  - ⚠️ Numéros de propositions exacts et dates à confirmer un jour sur Mintscan : les explorateurs web étaient inaccessibles
    pendant la rédaction de cette note.
- Osmosis fonctionne sur le même modèle (whitelist par gouvernance), donc ce n'est pas une spécificité du Hub.
- ⚠️ **Correction (T-013 puis T-014)** : Gaia embarque le module wasm. J'avais d'abord conclu du
  code source que l'upload était **réservé à la gouvernance, y compris sur testnet** — c'était faux.
  La chaîne dit : `gaiad query wasm params` → `code_upload_access: Everybody` sur le testnet `provider`.
  Un contrat custom s'y déploie **sans gouvernance** (fait le 2026-09-29 : code ID 738, contrat
  `cosmos15t68c2h8qkvwkmkvksy9yn3fg2pyj6ec8cenad9svgjc2q96cjcq538m53`). Le mainnet reste à vérifier.
  Détail et contraintes : `docs/notes/testnet-cosmos-hub.md`.
- Binaire : `gaiad`. Testnet : `provider` (denom `uatom`, faucet `faucet.polypore.xyz`).
  Repo des testnets : `github.com/cosmos/testnets` (répertoire `provider`).

## Autres chaînes (pour mémoire, non retenues)

- **Juno** — vivante, permissionless, « home of CosmWasm » ; pivot 2026 vers les agents autonomes. Aucune intégration au marketplace Stargaze.
- **Neutron** — la fondation a annoncé le passage en **support long terme** et la dépréciation de ses frontends (juin 2026). À éviter comme cible.
- **Osmosis / Kujira / Terra** — permissionless pour les contrats, mais pas d'exposition au marché Stargaze.

## Conséquence pour tiles

Le projet vise Stargaze 2.0 sur le Cosmos Hub (ADR 0001). Le contrat se développe et se teste en local
(`cw-multi-test`), puis sur le testnet du Hub. La production (Studio 2.0 vs proposition de gouvernance) est
tranchée par un ADR ultérieur, quand le produit sera complet et utilisable.
