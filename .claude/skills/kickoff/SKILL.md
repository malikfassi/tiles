---
name: kickoff
description: Lance le cadrage complet d'un périmètre - produit, puis architecture, puis plan de tâches - avec validation de Malik entre chaque phase.
argument-hint: "[périmètre]"
disable-model-invocation: true
---

Cadrage de : **$ARGUMENTS**

Tu orchestres ; tu n'écris pas les specs toi-même. Les rôles lisent les docs eux-mêmes : ne leur recopie pas leur contenu, donne-leur le périmètre et ce qui a changé.
Les rôles vivent dans `.clinerules/skills/` ; en session Claude Code, applique-les directement dans l'ordre.

1. **Produit.** Applique le rôle `product` (`.clinerules/skills/product/SKILL.md`) sur ce périmètre. Présente le résumé et les questions ouvertes à Malik. Attends ses réponses, reporte-les dans `docs/product.md` (section Entrées, datée).
2. **Architecture.** Applique le rôle `architect`. Présente les décisions proposées, les ADR à ouvrir et les points à valider. Attends la validation.
3. **Plan.** Applique le rôle `planner`. Présente le nombre de tâches, la première tranche verticale et les bloquants.
4. Mets à jour `docs/STATUS.md` : phase atteinte, décisions validées, prochaine étape.

Entre deux phases, ne continue jamais sans une réponse explicite de Malik.
