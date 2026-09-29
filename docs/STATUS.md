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

**Ce qui marche vraiment (testé le 2026-09-28) :** `cargo test` **48/48 verts**, `cargo build` OK, `cargo fmt --check` propre, `cargo clippy --all-targets` sans erreur (2 warnings de nommage mineurs). Toolchain : Rust stable 1.98.1 (exigé ≥ 1.86 par cw721-base 0.22).

**Point clé de T-006 — le devis :** la query `QuotePixelUpdates { token_id, updates }` renvoie ce que les écritures coûteront (`total`, les trois parts, les expirations). Elle passe par **le même `quote()`** que `SetPixelColor`, donc ce qui est affiché est exactement ce qui est facturé. C'est indispensable pour le frontend et ça élimine toute divergence prix annoncé / prix payé.

## Prochaine étape
`/next-task T-007` — validation et erreurs : finir de déplacer les règles dans les handlers et couvrir chaque variante d'erreur. Puis T-008 (finalisation du bail) et T-009 (répartition avancée).

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

