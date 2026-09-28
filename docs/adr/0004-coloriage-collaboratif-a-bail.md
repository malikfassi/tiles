# 0004 — Coloriage collaboratif à bail

- Statut : accepté
- Date : 2026-09-28

## Contexte
Le contrat permettait à quiconque de payer pour colorier un pixel, mais sans règle explicite sur l'autorisation
ni sur la durée de validité d'une couleur. Le code ne faisait d'ailleurs pas ce qu'il annonçait :
`validate_for_tile` retournait toujours `Ok(())`, donc l'expiration n'était jamais appliquée.
Ces deux questions décident de la nature du produit : canvas collaboratif ouvert, ou œuvre privée.

## Décision
1. **Colorier est ouvert à tous.** N'importe qui peut payer pour écrire la couleur d'un pixel de n'importe quelle tuile.
   Le propriétaire de la tuile n'a pas à autoriser l'opération : il encaisse sa part automatiquement.
2. **Le bail est protégé.** Une fois payée, une couleur est inaltérable jusqu'à son expiration
   (`expiration_timestamp > block.time`). Tant que le bail court, personne ne peut écraser le pixel — pas même
   le propriétaire de la tuile. À expiration, le pixel redevient libre et coloriable par le premier payeur.

## Conséquences
- Aucun contrôle d'ownership sur l'appelant de `SetPixelColor` : seul le paiement est vérifié. Un utilisateur peut
  donc colorier une tuile qu'il ne possède pas, ce qui est le comportement voulu.
- La durée d'achat devient un achat de visibilité : payer plus longtemps = rester affiché plus longtemps.
- L'erreur `PixelLeaseActive` est levée quand le pixel visé a un bail encore valide (c'était le bug de T-002).
- La protection du bail implique qu'aucun mécanisme de modération/correction on-chain n'est possible : une fois
  payée, une couleur reste jusqu'au terme. C'est assumé (décentralisation).
- Point ouvert, à trancher en T-008 : le titulaire d'un bail peut-il renouveler ou étendre son **propre** pixel avant
  expiration ? Par défaut non (application littérale du bail) ; recommandation de l'architecte : l'autoriser pour son
  propre bail, sinon l'expérience « je prolonge ma couleur » est impossible.

## Alternatives écartées
- Colorier réservé au propriétaire de la tuile : supprime le canvas collaboratif, qui est l'intérêt du produit.
- Écrasement libre même pendant un bail actif : rendrait l'achat de durée trompeur (on paierait sans garantie).
- Écrasement avec premium (×2) sur un bail actif : protège l'acheteur sans figer le canvas, mais ajoute une règle de
  prix, des arrondis et des tests pour un bénéfice incertain ; écarté pour rester simple, révisable si besoin.
