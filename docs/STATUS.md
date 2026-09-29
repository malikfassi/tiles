# STATUS — 2026-09-28

## Où on en est
Reprise du projet après abandon (dernier commit : 2025-01-03). Objectif de session : retrouver le fil et remettre le projet sur les rails.

**Fait :**
- État des lieux complet du code et de l'écosystème ; comparaison avec le projet voisin `mosaic` (à `/Users/malikfassifihri/dev/mosaic`).
- Décision de cible : **Stargaze 2.0 sur le Cosmos Hub** (ADR 0001), standard **CW721** (ADR 0002), portage **CosmWasm 2.x** (ADR 0003).
- Pilotage installé : `.clinerules/` (rôles, workflows, conventions) + miroir `.claude/skills/`, `docs/` (product, architecture, ADR, plan, notes).
- Note d'écosystème sourcée : `docs/notes/ecosysteme-cosmos-2026.md` (Stargaze L1 morte, Hub permissionné, précédents de gouvernance).
- **T-001 fait** : workspace réparé (membres `vendor/*` fantômes retirés, dépendance `sg_std` retirée de `build.rs`, denom `uatom` en constante locale).
- **T-002 fait** : inventaire complet du contrat dans `docs/architecture.md` (structure, messages, problèmes identifiés et classés par gravité).
- **ADR 0004 accepté** : coloriage ouvert à tous (canvas collaboratif) et bail de couleur protégé jusqu'à expiration. Pitch, architecture et critères de T-008 mis à jour en conséquence.
- **T-003 + T-004 + T-005 faits** : portage complet en CW721 0.22 / CosmWasm 2.x. `sg721`, `sg-std`, `vending-*`, `sg-multi-test` **entièrement retirés** du projet.
- **T-006 fait** : état et temps revus. `Timestamp` de cosmwasm partout (plus de `u64` en secondes), prix calculé par une **source unique** (`core/quote.rs`) partagée entre l'exécution et la nouvelle query `QuotePixelUpdates`, `interpolate()` sans `unwrap()`, événement enrichi (`leased_pixels`).
- **T-007 fait** : un test unitaire par règle et par variante d'erreur (`tests/core/validation_input.rs`, `validation_lease.rs`, `validation_money.rs`).
- **T-008 fait** : `set_pixel_color` couvert de bout en bout. **Un bug mort retrouvé dans `single_payment`** (`!is_valid_hex_color("#FFFFFF") && info.funds.is_empty()` : le premier bloc ne s'exécutait jamais) → supprimé. Six nouveaux tests de paiement : montant exact accepté, sous-paiement refusé, sur-paiement refusé, denom inconnu refusé, paiement absent refusé, multi-denom refusé.
- **T-009 fait** : répartition des paiements en **points de base entiers** (500 / 200 sur 10 000), plus aucun flottant dans le chemin de l'argent. `split_payment_bps` floor chaque part une fois et donne le reliquat au propriétaire : la somme égale toujours le montant reçu, et le reliquat est borné à 2 unités (prouvé par balayage de 1 à 2 000). `Config` passe de `Decimal` à `u64`.
- **T-010 fait** : `tests/contract/errors.rs`, **15 tests, un par variante d'erreur atteignable**, chacun asserant la variante exacte. Deux corrections de fond au passage : `tests/mod.rs` ne chargeait pas les fichiers `mod.rs` (les tests d'intégration écrits en T-009 n'étaient **jamais compilés**), et `cw-multi-test` ne transmet pas l'erreur du contrat — les variantes se testent donc en appelant les handlers avec `mock_dependencies`.
- **T-011 fait** : le contrat n'avait **aucun entry point `migrate`** — impossible à mettre à jour en place. `migrate_handler` + événement `migration`, avec 7 tests. Règle retenue : le nom de contrat doit correspondre, migrer depuis la version courante est un no-op, **toute autre version est refusée** plutôt que devinée.
- **T-012 fait** : scripts de déploiement portés de Stargaze L1 vers le Cosmos Hub. `starsd`, `ustars`, `vending-*` et `sg721` **entièrement retirés** ; tout passe par `gaiad` et `uatom`. Scripts : `01_build`, `02_deploy`, `03_mint`, `04_set_pixel_color`, `query_contract`, `query_tx`, plus 4 tests qui font le pont script↔contrat. **Trois bugs corrigés au passage** : `build.rs` corrompait toutes les constantes exportées (`MIN_PIXEL_PRICE` valait `128100000`), `00_load_constants.sh` cassait sur tout espace (la description de collection était tronquée à `A`), et le message d'instantiation était rejeté par la chaîne (`share` doit être une chaîne, pas un nombre JSON).

- **T-013 fait** : note `docs/notes/testnet-cosmos-hub.md`. **La croyance « le testnet du Hub accepte un upload libre » est fausse.** Gaia embarque le module wasm, mais l'upload est réservé à la gouvernance (`NewKeeper(..., govtypes.ModuleName, ...)`), et le testnet `provider` utilise la même application. Note d'écosystème corrigée.

## ⚠️ Décisions en attente pour Malik
- **T-016 / T-017 (orchestration web et interface)** : mis en `attente-malik`. Le choix de **où vivent la lecture des
  événements et l'indexation** (client RPC direct, indexer maison, indexer tiers, ou décision reportée) engage
  l'architecture et conditionne les deux tâches. Question posée, réponse en attente.
- **T-014 (déploiement testnet)** : de type `Manuel`. Le scénario est prêt à être joué (voir T-019) ; il faut une clé
  financée sur le testnet `provider` et `gaiad` installé.

## Où en est le produit, concrètement
**11 tâches sur 19 sont faites** (T-001 à T-013, T-018, T-019). Aucune tâche d'agent n'est bloquée par une autre :
les restantes sont soit manuelles (T-014), soit des décisions (T-015), soit en attente de Malik (T-016, T-017).

**Ce qui marche vraiment (testé le 2026-09-28) :** `cargo test` **115/115 verts** (vérifié avec `-- --list`),
`cargo fmt --check` propre, `cargo clippy --all-targets` sans erreur nouvelle, tous les scripts passent `bash -n`.
Toolchain : Rust stable 1.98.1 (exigé ≥ 1.86 par cw721-base 0.22).

## À traiter par la suite (non bloquant)
- **`src/contract/contract.rs` porte le même nom que son module parent** : lint clippy `module_inception`, **préexistant** à T-009. Signale un vrai défaut de structure (conventions : pas de `contract/contract/`). À corriger dans une tâche dédiée — renommer touche `lib.rs` et les entry points.
- **T-011 (migration d'état) est fait** : `Config` a changé de schéma en T-009 (`Decimal` → `u64`), et c'est désormais couvert par un entry point `migrate` testé. Un déploiement pré-T-009 ne pourrait toutefois pas être migré (conversion non écrite, car aucun contrat n'est déployé).
- **`tests/contract/pixel/validation.rs` n'assertent que `is_err()`** : ils passent si le contrat renvoie la mauvaise erreur. Doublés par `tests/contract/errors.rs`, mais à nettoyer un jour.

## Ordre de validation du contrat (établi par T-010, à ne pas contredire)
1. Forme du message (`InvalidPixelId`, `InvalidColorFormat`, `DuplicatePixelId`, `EmptyUpdates`, bornes de bail).
2. Règles dépendant de l'état, une fois la tuile chargée : `TileNotFound` avant tout examen des fonds.
3. **Paiement** (`InvalidPayment`) — avant le contrôle du hash de métadonnées.
4. `MetadataHashMismatch` (verrou optimiste).
5. `PixelLeaseActive`, puis écriture d'état.

## Piège de structure des tests (T-009/T-010)
`tests/mod.rs` est le **seul** point d'entrée de la suite d'intégration. Il déclare `pub mod contract;` et `pub mod core;`,
qui délèguent aux fichiers `mod.rs`. Un fichier de test ajouté dans un dossier **sans** être déclaré dans le
`mod.rs` correspondant est silencieusement ignoré : `cargo test` reste vert. Vérifier la présence du test avec
`cargo test -- --list` avant de conclure qu'il passe.

## ✅ Décision tranchée : ADR 0005 — denom de paiement (T-018, fait)
Recherche faite dans `docs/notes/stargaze-2-denoms-paiement.md` (sources : `paying-with-different-tokens`,
`collect/fees`, `collect/minting-and-trading`). **Conclusion : `uatom` est le bon denom, sans configuration.**

- ATOM est le token **« primary »** de Stargaze 2.0, et l'exemple officiel de répartition des frais est
  libellé en ATOM (vente de 100 ATOM → 92 vendeur, 5 créateur, 2 marketplace). La crainte « la part
  collection se confond avec les frais » était fausse : le gas est un flux **séparé**, payé en plus du prix.
- Le multi-token de Stargaze 2.0 (ATOM, TIA, BTC, STARS, USDC) est une **orchestration frontend**
  (swap Skip), pas une capacité du CW721. Un contrat autonome ne peut pas la reproduire sans dépendre
  d'un routeur externe — contraire aux règles du projet.
- **Option B retenue** : `uatom` en dur, zéro changement de code. Un test verrouille la décision
  (`the_only_accepted_denom_is_uatom`) pour qu'une régression soit détectée si quelqu'un y touche.

## Prochaine étape
`/next-task T-009` — répartition des paiements (somme exacte, reliquat explicite, points de base), puis T-010 (suite de tests) et T-011 (migration d'état).


**Point clé de T-006 — le devis :** la query `QuotePixelUpdates { token_id, updates }` renvoie ce que les écritures coûteront (`total`, les trois parts, les expirations). Elle passe par **le même `quote()`** que `SetPixelColor`, donc ce qui est affiché est exactement ce qui est facturé. C'est indispensable pour le frontend et ça élimine toute divergence prix annoncé / prix payé.

**Règles du bail, désormais testées une par une :** pixel libre ouvert à tous ; bail expiré libéré ; un tiers refusé sous un bail actif ; **le titulaire peut prolonger** son propre pixel ; un bail qui expire exactement à l'instant du bloc est libre ; un pixel protégé fait refuser tout le lot.

## Prochaine étape
`/next-task T-008` — finaliser `set_pixel_color` (cœur du produit), puis T-009 (répartition avancée) et T-011 (migration). Vérification en Plan recommandée.


## Décisions en attente de Malik
1. **Mode de production** (à trancher plus tard, quand le contrat sera complet) : Studio 2.0 + logique hors chaîne vs proposition de gouvernance — ADR à ouvrir.

(Toutes les règles de coloriage sont tranchées : ADR 0004, y compris la prolongation d'un bail par son titulaire.)



## Pièges connus (ne pas refaire)
- **Shell : pas de heredocs.** `cat <<'EOF' … EOF` et `python3 - <<'PYEOF'` laissent le terminal en mode `heredoc>` et mélangent la sortie : plusieurs commandes ont été perdues en session. Utiliser l'outil d'édition de fichiers pour créer/modifier un fichier, ou `python3 -c "..."` sur une seule ligne.
- **Shell : jamais de pager.** Toujours `git --no-pager log --oneline -N` (jamais `git log` nu, il ouvre un pager interactif qui bloque). Idem `git --no-pager diff` / `show`.
- **Shell : borner les commandes longues.** `(timeout N <cmd> > /tmp/x.log 2>&1); grep … /tmp/x.log` puis lecture ciblée, jamais de sortie brute non bornée.
- **Ne plus chercher Stargaze L1** : `elgafar-1` est mort, `starsd` n'est plus la CLI, le repo `public-awesome/stargaze` est archivé. Tout le dossier `launchpad/` est une copie morte du projet Stargaze.
- **Ne pas réintroduire `sg721`/`sg-std`/`vending-*`** : ce sont ces dépendances qui rendent le contrat impossible à déployer (ADR 0002). Le portage est fait, ne pas le défaire.
- **Toolchain** : `cw721-base` 0.22 exige **Rust ≥ 1.86**. La machine a deux toolchains ; les commandes du projet passent par `rustup run stable cargo …` (stable est en 1.98.1), le toolchain par défaut étant un nightly 1.85 trop ancien.
- **Adresses en test** : CW721 0.22 valide les adresses en écriture. Les acteurs de test doivent être de vrais bech32 préfixés `cosmwasm` (le préfixe de `cw-multi-test`), pas des étiquettes comme `"buyer"`.
- **Ne pas perdre de temps sur les explorateurs web** (Mintscan, forum Discourse, DuckDuckGo) : SPA/captchas. Les docs officielles passent par GitBook et sont lisibles avec `.md` (ex. `docs.stargaze.zone/developers/overview.md`).
- `mosaic/` (décembre 2024) est un **prédécesseur**, pas un doublon : le garder comme archive pour le frontend (`mosaic/frontend`, spec dans `mosaic/TECHNICAL_SPEC.md`), sans y travailler.

