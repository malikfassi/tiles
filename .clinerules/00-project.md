# Tiles — instructions Cline

Plateforme de pixel art collaborative en NFT. Chaque **tuile** est un NFT (CW721) contenant 100 pixels ;
son propriétaire peut changer la couleur d'un pixel pendant une durée limitée, contre paiement
(prix qui évolue avec le remplissage de la tuile, revenus répartis entre propriétaire, royalties et plateforme).

Cible : **Stargaze 2.0 sur le Cosmos Hub** (`cosmoshub-4`, gas en ATOM, standard CW721). Voir `docs/notes/ecosysteme-cosmos-2026.md`
et l'ADR 0001 pour le pourquoi (l'ancienne chaîne Stargaze L1 et son testnet `elgafar-1` sont morts en 2026).

Cette configuration `.clinerules/` vient du projet runassist. `.claude/skills/` contient le miroir pour Claude Code :
si tu modifies un workflow ou un rôle, répercute le changement dans les deux dossiers.

## Lire en premier
`docs/STATUS.md` : où on en est, prochaine étape. Ne pas ré-explorer ce qui y est déjà écrit.

## Où est quoi
- `docs/product.md` : vision, périmètre, user stories, questions ouvertes
- `docs/architecture.md`, `docs/adr/` : structure technique, décisions
- `docs/plan/tasks.md` : tâches, statut
- `docs/notes/` : recherches déjà faites (écosystème Cosmos, CW721…) ; les lire avant de rechercher à nouveau
- `src/` : le contrat CosmWasm (`contract/`, `core/`, `events/`, `defaults/`)
- `tests/` : tests d'intégration `cw-multi-test`
- `scripts/` : build et déploiement (encore écrits pour Stargaze L1 : à porter)
- `.clinerules/skills/` : un dossier par rôle (product, architect, planner, rust-dev, reviewer, qa, researcher) + connaissances (`rust-conventions`, `cosmwasm-domain`)
- `.clinerules/workflows/` : procédures (`kickoff`, `next-task`, `adr`, `handoff`)

## Trois sortes de blocage (ne pas les confondre)
Toute tâche de `docs/plan/tasks.md` porte un `Type` :

1. **Travail d'agent** — recherche, architecture, implémentation, relecture, QA. Cline le fait seul. `/next-task` ne prend que ces tâches.
2. **Action hors agent** (`Manuel`) — déployer sur un testnet, manipuler un wallet, vérifier sur un explorateur, signer une transaction.
   Aucun agent ne peut le faire. La tâche passe à `attente-manuel` quand tout le travail d'agent est fini.
3. **Décision qui revient à Malik** — question produit, architecture ou implémentation. Cline pose la question et met la tâche en attente,
   sans inventer la réponse ni ouvrir un ADR à sa place.

**Règle clé : un blocage sur (2) ou (3) n'arrête jamais tout le projet.** Cline enchaîne sur les autres tâches exécutables indépendantes.

## Règles
- Rust, CosmWasm **2.x**, CW721 standard. Le contrat doit rester **portable** : aucune dépendance spécifique à une chaîne
  (`sg721`, `sg-std`, `vending-*` sont interdits — ils sont morts avec Stargaze L1).
- Code, commentaires et identifiants en anglais ; documentation et échanges avec Malik en français.
- Pas de contournement ni de dette technique : on trouve la cause et on fait propre. Si ce n'est pas possible, on s'arrête et on demande à Malik.
- Le contrat ne dépend d'aucun service externe pour sa logique. L'orchestration web/indexation vit hors chaîne (ADR à venir).
- Toute la logique testable doit l'être sans nœud : `cargo test` (cw-multi-test) est la référence.
- Une décision = un ADR court (workflow `adr`). Une décision acceptée ne se re-débat pas sans nouvel élément.
- Petite modif : directement, en Act. Tâche du plan : workflow `next-task`. Fin de session : workflow `handoff`.

## Modes Cline (voir `01-mode-routing.md`)
Un seul modèle actif par mode, Plan et Act. Charge le skill du rôle qui correspond à la tâche
(`.clinerules/skills/<rôle>/SKILL.md`) et reste dans le mode indiqué par ce skill pour tout le travail de ce rôle.
