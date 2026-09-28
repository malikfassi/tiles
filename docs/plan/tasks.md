# Plan de tâches — tiles

> Produit par le rôle `planner` (`.clinerules/skills/planner/SKILL.md`) à partir des ADR 0001-0003.
> Types : `Agent` (Cline fait seul), `Manuel` (Malik : testnet, wallet, explorateur), `Décision` (Malik tranche).
> Statuts : `à faire` | `en cours` | `fait` | `bloqué` | `attente-manuel`.
> L'avancement réel et la prochaine étape sont dans `docs/STATUS.md`.

## Tableau

| ID | Titre | Type | Statut | Dépend de | Vérif. Plan |
|---|---|---|---|---|---|
| T-001 | Réparer le workspace (`vendor/*` fantôme) et obtenir `cargo test` vert sur l'existant | Agent | fait | — | non |
| T-002 | Inventaire et documenter l'état réel du contrat (module par module) dans `docs/architecture.md` | Agent | à faire | T-001 | non |
| T-003 | Portage CosmWasm 2.x : dépendances, entry points, helpers (`to_json_binary`, `Response`, `Uint128`) | Agent | à faire | T-002 | **oui** |
| T-004 | Remplacer `sg721-base` par `cw721-base` + extension de collection (`royalty_info`, share ≤ 0.10) | Agent | à faire | T-003 | **oui** |
| T-005 | Supprimer le mint de type vending factory/minter et le remplacer par un mint direct du propriétaire | Agent | à faire | T-004 | **oui** |
| T-006 | Refonte de l'état : pixels par tuile interrogeables, prix par tuile, timestamps cosmwasm | Agent | à faire | T-004 | **oui** |
| T-007 | Validation et erreurs : variantes dédiées, bornes vérifiées dans les handlers (TODO.md §2, §3) | Agent | à faire | T-006 | non |
| T-008 | `set_pixel_color` : ownership, expiration, paiement exact, événement indexable | Agent | à faire | T-006 | **oui** |
| T-009 | Répartition des paiements : somme exacte, reste d'arrondi explicite, points de base | Agent | à faire | T-008 | **oui** |
| T-010 | Suite de tests `cw-multi-test` : un test par règle et par variante d'erreur | Agent | à faire | T-008 | non |
| T-011 | Migration d'état documentée et testée (schéma versionné) | Agent | à faire | T-006 | **oui** |
| T-012 | Scripts de déploiement testnet Cosmos Hub (`gaiad`) + constantes (`CHAIN_ID`, `NODE_URL`, denom ATOM) | Agent | à faire | T-010 | non |
| T-013 | Vérifier que le wasm du testnet du Hub est déployable librement (note `docs/notes/`) | Agent | à faire | — | non |
| T-014 | Déployer le contrat sur le testnet du Hub et minter une tuile | Manuel | à faire | T-012, T-013 | — |
| T-015 | ADR production : Studio 2.0 + logique hors chaîne vs proposition de gouvernance | Décision | à faire | T-014 | — |
| T-016 | Orchestration web : lecture des pixels et des événements du contrat | Agent | à faire | T-010 | non |
| T-017 | Interface : canvas de tuiles et coloriage, réutiliser `mosaic/frontend` | Agent | à faire | T-016 | non |

## Détail des tâches à venir

**T-001 — Réparer le workspace.** Retirer du `Cargo.toml` les membres `vendor/vending-minter` et `vendor/vending-factory`
(le dossier `vendor/` n'existe pas), vérifier `build.rs`, lancer `cargo build` puis `cargo test`.
Critères : `cargo metadata` sans erreur ; `cargo test` se termine et son résultat est consigné tel quel (vert ou liste d'échecs).

**T-002 — État réel du contrat.** Lire `src/` et `tests/` et écrire `docs/architecture.md` : modules, état stocké,
messages expose/query, événements, ce qui dépend de sg721. Aucun code modifié.
Critères : `docs/architecture.md` couvre chaque module de `src/` ; la liste des dépendances Stargaze à retirer est explicite.

**T-013 — Vérifier le statut permissionless du testnet du Hub.** Note `docs/notes/testnet-cosmos-hub.md` :
voyons si un utilisateur peut y uploader un code ID sans proposition. Si non, le repli est un nœud `wasmd` local en Docker.
Critères : la note conclut par oui/non avec la source officielle.
