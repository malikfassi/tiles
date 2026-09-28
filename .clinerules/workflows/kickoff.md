Cadrage de : **{{args}}**

Tu es dans une seule conversation Cline (pas d'agents séparés) : à chaque étape, charge le skill de rôle indiqué
et bascule dans le mode Cline qu'il demande avant de continuer. Ne saute pas les validations de Malik entre les phases.

1. **Produit — mode Plan.** Charge `.clinerules/skills/product/SKILL.md`, applique-le à ce périmètre.
   Présente à Malik le résumé et les questions ouvertes. Attends ses réponses, reporte-les dans `docs/product.md`
   (section Entrées, datée), et refais l'étape seulement si ça change le périmètre.
2. **Architecture — mode Plan.** Charge `.clinerules/skills/architect/SKILL.md`. Présente les décisions proposées,
   les ADR à ouvrir et les points à valider. Attends la validation.
3. **Plan de tâches — mode Plan.** Charge `.clinerules/skills/planner/SKILL.md`. Présente le nombre de tâches,
   la première tranche verticale et les bloquants.
4. Mets à jour `docs/STATUS.md` (workflow `handoff` si la session se termine ici).

Entre deux phases, ne continue jamais sans une réponse explicite de Malik.
