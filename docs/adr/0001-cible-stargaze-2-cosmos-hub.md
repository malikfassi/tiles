# 0001 — Cible de déploiement : Stargaze 2.0 sur le Cosmos Hub

- Statut : accepté
- Date : 2026-09-28

## Contexte
Le projet a été conçu et déployé en décembre 2024 sur le testnet Stargaze `elgafar-1`, avec une collection sg721.
Entre-temps, Stargaze L1 a été arrêté : le réseau a migré vers le Cosmos Hub (`cosmoshub-4`), le testnet `elgafar-1`
n'existe plus et le dépôt `public-awesome/stargaze` est archivé depuis le 11 juin 2026. Toute collection Stargaze 2.0
est un contrat CW721 sur le Cosmos Hub, payable en ATOM. Le code existant (sg721-base, scripts `starsd`) ne peut plus être déployé tel quel.

## Décision
Le contrat tiles est porté sur **CW721 standard + extension de collection** (royalties ≤ 10 %) et vise
**Stargaze 2.0 sur le Cosmos Hub** comme cible de production. Le développement se fait d'abord contre
`cw-multi-test`, puis sur un testnet du Cosmos Hub ; le mainnet dépend d'un ADR ultérieur (Studio 2.0 ou
proposition de gouvernance pour whitelister un contrat custom).

## Conséquences
- Suppression de toute dépendance Stargaze : `sg721`, `sg721-base`, `sg-std`, `sg1/2/4`, `vending-*`, `sg-multi-test`.
- Le contrat devient portable : il fonctionne sur toute chaîne CosmWasm 2.x, ce qui préserve les options.
- Les scripts de déploiement et les `.state` d'`elgafar-1` sont historiques et ne servent plus de référence.
- Le déploiement en mainnet n'est pas acquis : le wasm du Cosmos Hub est permissionné (gouvernance) — à traiter en temps voulu.

## Alternatives écartées
- Rester sur Stargaze L1 / `elgafar-1` : impossible, la chaîne est éteinte et le testnet supprimé.
- Autre chaîne permissionless (Juno, Kujira) : déploiement libre, mais aucune intégration au marketplace Stargaze 2.0.
- Collection via Stargaze Studio 2.0 sans contrat custom : supprimerait la logique de pixels et d'expiration on-chain, qui est le cœur du produit.
- Neutron : la fondation a annoncé le passage en support long terme et le retrait de ses frontends (2026).
