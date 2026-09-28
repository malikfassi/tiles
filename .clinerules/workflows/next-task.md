Tâche demandée : {{args}} (si vide : la première tâche **exécutable par un agent** de `docs/plan/tasks.md` dont les dépendances sont `fait`).

Ce workflow tourne dans le worktree Git courant. `docs/plan/tasks.md` local peut être en retard sur `master`
si d'autres sessions ont fusionné du travail entre-temps : toujours se resynchroniser avant de choisir une tâche,
et toujours fusionner à la fin, sinon le travail reste invisible et une tâche finie peut être refaite en double.

Cline n'a pas d'agents séparés : chaque étape charge un skill de rôle et demande un mode Cline précis
(voir `.clinerules/01-mode-routing.md`). Bascule de mode avant chaque étape qui le demande.

## Quelle tâche ce workflow choisit

- Il ne prend que des tâches **exécutables sans Malik** : tout type **sauf** `Manuel`.
- Il **ne prend jamais** une tâche `attente-manuel`.
- Une tâche `Manuel` — `à faire`, `attente-manuel` ou `en cours` — **ne bloque jamais** une tâche exécutable sans lien avec elle.
  Ne traite comme dépendance que ce qui est écrit dans la colonne « Dépend de » : si la dépendance n'est pas `fait`,
  la tâche dépendante est **différée**, pas bloquée pour autant — passe à la suivante de la liste.
- On ne s'arrête que s'il ne reste **que** des tâches `Manuel`/`attente-manuel`, ou qu'un vrai choix revient à Malik.

## Décisions qui reviennent à Malik

- Une question produit, architecture ou implémentation peut surgir pendant n'importe quelle tâche.
- Dans ce cas : **pose la question à Malik et mets la tâche en attente de sa réponse**. N'invente jamais la réponse,
  ne choisis pas « au plus raisonnable » pour continuer, et n'ouvre pas un ADR à sa place.
- C'est un arrêt **sur cette tâche seulement** : tu peux enchaîner sur une autre tâche exécutable indépendante.
- Si la décision nécessite un ADR : la décision se prend d'abord (mode Plan, skill `architect`), l'ADR se remplit ensuite (workflow `adr`).

## Étapes

0. **Resynchronisation** (Act) : `git fetch origin 2>/dev/null || true` puis `git merge --ff-only origin/master` sur la branche courante.
   En cas de conflit ou d'avance impossible en fast-forward, le signaler et demander à Malik plutôt que de forcer.
1. **Choix de la tâche** (Act) : relis `docs/plan/tasks.md` après resynchronisation. Choisis la première tâche `à faire`
   de type autre que `Manuel` dont toutes les dépendances sont `fait`. Passe-la à `en cours` et commit ce changement de statut seul.
2. **Implémentation — mode Act.** Charge `.clinerules/skills/rust-dev/SKILL.md` et implémente la tâche.
3. **Relecture — mode Plan.** Charge `.clinerules/skills/reviewer/SKILL.md`. Si constats graves : reviens à l'étape 2, puis relis une fois.
   Deux allers-retours au maximum, sinon demande à Malik.
4. **Build et tests — mode Act.** Charge `.clinerules/skills/qa/SKILL.md` (`cargo fmt --check`, `cargo clippy`, `cargo test`).
5. Si tout passe : statut `fait`, commit. Sinon : `bloqué` avec la raison en une ligne, commit, et ne pas fusionner.
   - **Cas particulier** : si la tâche est finie côté agent mais exige une action de Malik (déploiement testnet, signature,
     vérification explorateur), marque-la `attente-manuel` et indique la commande à lancer.
6. **Fin de tâche** : si la tâche est `fait`, ne fusionne pas automatiquement. Malik décide quand intégrer la branche.
7. Rapport à Malik en 5 lignes : ce qui marche, ce qui reste, prochaine tâche, branche/commit créés, prêt à fusionner ou non.
   Si une décision attend Malik, la poser en premier.
