# Plan de tâches — tiles

> Produit par le rôle `planner` (`.clinerules/skills/planner/SKILL.md`) à partir des ADR 0001-0003.
> Types : `Agent` (Cline fait seul), `Manuel` (Malik : testnet, wallet, explorateur), `Décision` (Malik tranche).
> Statuts : `à faire` | `en cours` | `fait` | `bloqué` | `attente-manuel`.
> L'avancement réel et la prochaine étape sont dans `docs/STATUS.md`.

## Tableau

| ID | Titre | Type | Statut | Dépend de | Vérif. Plan |
|---|---|---|---|---|---|
| T-001 | Réparer le workspace (`vendor/*` fantôme) et obtenir `cargo test` vert sur l'existant | Agent | fait | — | non |
| T-002 | Inventaire et documenter l'état réel du contrat (module par module) dans `docs/architecture.md` | Agent | fait | T-001 | non |
| T-003 | Portage CosmWasm 2.x : dépendances, entry points, helpers (`to_json_binary`, `Response`, `Uint128`) | Agent | fait | T-002 | **oui** |
| T-004 | Remplacer `sg721-base` par `cw721-base` + extension de collection (`royalty_info`, share ≤ 0.10) | Agent | fait | T-003 | **oui** |
| T-005 | Supprimer le mint de type vending factory/minter et le remplacer par un mint direct du propriétaire | Agent | fait | T-004 | **oui** |
| T-006 | Refonte de l'état : pixels par tuile interrogeables, prix par tuile, timestamps cosmwasm | Agent | fait | T-004 | **oui** |
| T-007 | Validation et erreurs : variantes dédiées, bornes vérifiées dans les handlers (TODO.md §2, §3) | Agent | fait | T-006 | non |
| T-008 | `set_pixel_color` : coloriage ouvert à tous, bail protégé (`PixelLeaseActive`), paiement exact, événement indexable | Agent | fait | T-006 | **oui** |
| T-009 | Répartition des paiements : somme exacte, reste d'arrondi explicite, points de base | Agent | fait | T-008 | **oui** |
| T-010 | Suite de tests `cw-multi-test` : un test par règle et par variante d'erreur | Agent | fait | T-008 | non |
| T-011 | Migration d'état documentée et testée (schéma versionné) | Agent | fait | T-006 | **oui** |
| T-012 | Scripts de déploiement testnet Cosmos Hub (`gaiad`) + constantes (`CHAIN_ID`, `NODE_URL`, denom ATOM) | Agent | fait | T-010 | non |
| T-013 | Vérifier que le wasm du testnet du Hub est déployable librement (note `docs/notes/`) | Agent | fait | — | non |
| T-014 | Déployer le contrat sur le testnet du Hub et minter une tuile | Manuel | fait | T-012, T-013 | — |
| T-015 | ADR production : Studio 2.0 + logique hors chaîne vs proposition de gouvernance | Décision | à faire | T-014 | — |
| T-018 | ADR 0005 : denom de paiement (frais vs royalties, multi-denom à terme) | Décision | fait | T-008 | non |
| T-016 | Orchestration web : lecture des pixels et des événements du contrat | Agent | **attente-malik** | T-010 | non |
| T-017 | Interface : canvas de tuiles et coloriage, réutiliser `mosaic/frontend` | Agent | **attente-malik** | T-016 | non |
| T-019 | Scénario e2e en ligne de commande (`scripts/e2e.sh`) avec assertions on-chain | Agent | fait | T-012 | non |

**T-019 — Scénario e2e en ligne de commande.** Ajouté à la demande de Malik (session du 2026-09-28) : avant toute
partie web, un e2e réel avec le contrat, piloté uniquement par `gaiad`. `scripts/e2e.sh` exécute le parcours
complet et **assertionne chaque fait on-chain**, en nommant la règle vérifiée :

1. prérequis : `gaiad`, `jq`, solde suffisant, contrat déployé et joignable ;
2. instantiation : parts en points de base, adresse de collection, royalties CW721 déclarées ;
3. mint : la tuile a ses 100 pixels, pixel 0 en `#FFFFFF`, propriétaire correct ;
4. **prix** : le devis vient du contrat (`QuotePixelUpdates`), les trois parts s'additionnent exactement au total,
   un **sous-paiement d'une unité est refusé**, le paiement exact passe, et la couleur change réellement on-chain
   (relecture de `tile_pixels`) ;
5. **bail (ADR 0004)** : le titulaire peut recolorier son propre pixel, et son expiration est **prolongée**,
   pas réinitialisée à la même valeur ;
6. **paiement** : le solde du payeur diminue, et l'événement `payment_distribution` reporte exactement le total payé.

Configuration testnet dédiée : `scripts/messages/testnet.json` (chain-id `provider`, RPC polypore, faucet).
Usage : `scripts/e2e.sh testnet`. Sans `gaiad` sur la machine, le script sort en erreur explicitement au lieu
de faire semblant : il est donc sûr à rejouer.

**Ce que ce scénario ne peut pas couvrir** : le cas « un tiers ne peut pas écraser un bail actif » exige un
second signataire, donc une seconde clé financée. Il reste couvert par `cw-multi-test` et les tests unitaires
(`PixelLeaseActive`), et deviendra vérifiable sur chaîne dès que deux clés existeront dans le keyring.


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

**T-008 — Règle de coloriage (ADR 0004).** Aucun contrôle d'ownership sur l'appelant : `SetPixelColor` est ouvert à tous,
seuls le paiement et l'état du bail sont vérifiés. Un pixel dont le bail est encore valide (`expiration_timestamp > env.block.time`)
et détenu par quelqu'un d'autre ne peut pas être écrasé : erreur dédiée `PixelLeaseActive { token_id, pixel_id, expires_at }`.
**Le titulaire du bail (`last_updated_by`) peut prolonger son propre pixel** avant expiration : nouvelle couleur et/ou durée,
tarif normal, expiration recomptée depuis `env.block.time`, sans premium (amendement ADR 0004). À expiration, le pixel redevient libre.
Critères : un test par cas (pixel libre, bail actif par un tiers, bail actif par son titulaire, bail expiré, bail expirant exactement à l'instant du bloc, prolongation qui recompte depuis le bloc courant).

**T-009 — Répartition des paiements.** Fait le 2026-09-28. Les parts sont en **points de base entiers**
(`COLLECTION_SHARE_BPS = 500`, `PLATFORM_SHARE_BPS = 200`, `BPS_DENOMINATOR = 10 000`), plus aucun flottant
dans le chemin de l'argent. `split_payment_bps` floor chaque part une fois et donne le reliquat au propriétaire :
la somme égale toujours le montant reçu. Le reliquat que le propriétaire absorbe est borné à 2 unités
(une par part tronquée), prouvé par balayage de 1 à 2 000 plus quelques grandes valeurs.
`Config` porte `collection_share_bps: u64` / `platform_share_bps: u64` au lieu de `Decimal` : **changement de
schéma d'état à couvrir par la migration T-011**. `split_payment(Decimal)` reste en enveloppe de compatibilité,
avec un test d'équivalence sur 6 montants dont `u128::MAX`.
Tests : `tests/core/validation_money.rs` (unitaire) et `tests/contract/pixel/split.rs` (bout en bout : soldes
réels mouvementés, reliquat non nul vérifié, sous-paiement/sur-paiement/multi-denom/absence de paiement refusés,
prix revérifié après changement de grille, part collection tronquée et jamais arrondie au supérieur).

**T-011 — Migration d'état.** Fait le 2026-09-28. Le contrat n'avait **aucun** entry point `migrate` : il n'était
pas migrable du tout. `cw2` enregistrait déjà la version à l'instantiation, mais rien ne la relisait.
- `src/contract/migrate.rs` : `migrate_handler` applique trois règles strictes. Le nom de contrat stocké doit être
  celui du code (sinon `UnsupportedMigration` : on refuse de toucher le stockage d'un autre contrat) ; migrer depuis
  la version courante est un no-op ; **toute autre version est refusée** plutôt que devinée.
- `MigrateMsg` est vide : la migration ne lit aucun champ de l'appelant, donc rien à authentifier. Une migration
  future qui prendrait des paramètres devrait être protégée (noté dans le doc-comment).
- Événement `migration` (`from_version`, `to_version`) pour qu'un indexeur suive les changements de schéma.
- 7 tests dans `tests/contract/migrate.rs` : version enregistrée à l'instantiation, migration courante acceptée avec
  l'événement, version inconnue refusée, contrat étranger refusé, version intacte après refus, état préservé, et
  **le contrat reste utilisable après migration** (un pixel se colore encore).

**Conséquence à retenir** : le passage des parts en points de base (T-009) a changé le schéma de `Config`
(`collection_share_percent: Decimal` → `collection_share_bps: u64`). Migrer depuis cet ancien format est
**refusé** aujourd'hui, faute de branche de conversion. Aucun contrat n'étant déployé (T-014 à faire), il n'y a
rien à convertir ; le jour où un déploiement pré-T-009 existerait, la conversion s'écrit dans le `match` de
`migrate_handler`, avec son test.

