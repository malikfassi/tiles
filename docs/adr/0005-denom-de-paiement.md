# 0005 — Denom de paiement des pixels

- Statut : proposé
- Date : 2026-09-28

## Contexte
`NATIVE_DENOM` vaut `uatom` et c'est aussi le denom dans lequel le contrat paie ses trois parts.
Or sur le Cosmos Hub, `uatom` est le denom des frais de transaction : la part « collection »
(5 % aujourd'hui) serait donc versée dans le denom qui finance les frais, ce qui la rend
structurellement ambiguë — elle finance l'exécution avant d'être une recette.

Second problème : le contrat n'accepte qu'un seul denom. Le prix est une constante en `uatom`.
Toute évolution vers un stablecoin (prix lisible en USD, volatilité nulle) obligerait à redéployer.

Le bug a été révélé par T-008 : un test qui payait en « mauvais » denom a été accepté,
car ce mauvais denom était en réalité le seul denom reconnu.

## Décision
À trancher par Malik. Les trois options ci-dessous sont argumentées, aucune n'est retenue ici.

**Option A — denom de paiement explicite, distinct des frais.**
Un champ `payment_denom` dans `Config`, choisi à l'instantiation (par exemple `uusdc` sur le Hub),
distinct du denom des frais. Le prix, la part collection et la part plateforme sont exprimés dans ce denom.

**Option B — `uatom` assumé comme denom de paiement.**
On assume que la part collection finance les frais, et on le documente. Aucun changement de code.

**Option C — plusieurs denoms acceptés, avec prix par denom.**
`Config` porte une table denom → prix. Un payeur choisit son denom. Plus riche, plus lourd :
table de prix, oracle ou prix administrés, un test par denom.

## Conséquences
- Quelle que soit l'option, **le contrat doit refuser tout denom qu'il ne reconnaît pas**. C'est déjà
  le comportement actuel et il est testé.
- L'option A est la seule qui rend la part collection économiquement lisible. Elle ajoute un champ
  à `Config`, donc une migration d'état (T-011) et un impact sur les scripts de déploiement.
- L'option C multiplie les chemins de test et introduit une source de vérité pour les prix
  hors chaîne (ou une administration on-chain) : contraire pour l'instant à « pas de dépendance externe ».
- Décider tard coûte une migration ; décider tôt avec le mauvais denom coûte un redéploiement.

## Alternatives écartées
- **Fixer le denom du load-balancing côté distributeur** : hors du contrat, ne règle rien on-chain.
- **Ne pas décider et laisser `uatom` implicite** : c'est l'état actuel, et il est ambigu (part collection = frais).

## Question ouverte pour Malik
Le produit vend-il de la visibilité à un prix **lisible** (stable, option A/C) ou **natif** (option B, simple) ?
