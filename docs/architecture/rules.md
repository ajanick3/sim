# The base rules

The rules the engine implements, taken from the official Pokémon TCG rulebook
(`mew_rulebook_en.pdf`) and reviewed against it. The numbers are stable: the
code cites them, so a rule keeps its number even when the text around it moves.
The operator read them against the rulebook and corrected them.

## Deck construction

1. Exactly 60 cards.
2. Max 4 copies of any card with the same name — except Basic Energy (unlimited).
3. Max 1 ACE SPEC card **total** in the deck (not 1 per name).
4. _(Not in rulebook)_ Legality by regulation mark + the Banned Card List.

## Setup

5. Coin flip; **winner chooses** who goes first.
6. Each player shuffles and draws 7.
7. Mulligan: no Basic in hand → reveal hand, shuffle back, draw 7. Repeat.
8. Per mulligan the opponent took, you **may** draw 1 extra card.
9. Place 1 Basic face down as Active; up to 5 more face down on the Bench.
10. Top 6 cards aside face down as Prizes.
11. Both players flip Pokémon face up; game begins.

## Turn structure — 3 parts, in order

12. **Draw a card.** Deck empty and cannot draw → **you lose**.
13. **Do any of these, in any order:**
    - Put Basic Pokémon from hand onto Bench — any number (Bench cap 5)
    - Evolve any number of Pokémon, each once per turn
    - Attach an Energy from hand — **once per turn**
    - Play Trainers — Items any number, Tools any number, **1 Supporter**, **1 Stadium**
    - Retreat — **once per turn**
    - Use Abilities — any number, from Active _and_ Bench
14. **Attack, then the turn ends.** Cannot return to step 13.

## First-turn restrictions (player going first)

15. They **do** draw a card.
16. They **cannot play a Supporter** on their first turn.
17. They **skip the attack step** on their first turn.
18. **Neither player** can evolve on their first turn (unless a card says so).

## Evolution

19. Played on top of the Pokémon it evolves from, which must have been in play
    **since the beginning of your turn**.
20. Cannot evolve a Pokémon the turn it was played; cannot evolve the same
    Pokémon twice in one turn.
21. Works on Active **or** Benched Pokémon.
22. Keeps attached cards and damage counters; **clears all Special Conditions**
    and other effects.

## Retreat

23. Once per turn. Discard Energy equal to the Retreat Cost (free if none).
24. Requires at least one Benched Pokémon.
25. **Asleep and Paralyzed prevent retreating.** Confused does not.
26. Damage counters and attachments travel with the Pokémon.
27. Moving to the Bench **removes all Special Conditions**.
28. You **may still attack** after retreating, with the new Active.

## Attacking — damage order (the rulebook's explicit sequence)

29. Check the **attacking** Pokémon has the required Energy. (Normally the
    Active — but see Alakazam ex below.)
30. Do what the attack requires (coin flips). Confused's flip happens **before**.
31. Start from base damage, applying the attack's own text.
32. Apply effects on **your** Pokémon (before Weakness/Resistance).
    **Stop if damage is 0.**
33. Apply **Weakness** (increase), then **Resistance** (decrease).
34. Apply effects on the **defending** Pokémon (after Weakness/Resistance).
35. 1 damage counter per 10 final damage. **0 or less → no counters.**
36. **Weakness/Resistance never apply to Benched Pokémon.**
37. Effects that say "put damage counters" bypass all of the above.

## Knockout & prizes

38. Damage >= HP → Knocked Out; the Pokémon and all attached cards go to its
    owner's discard.
39. The **opponent of the KO'd player takes 1 Prize** (or as many as specified).
40. The player whose Active was KO'd chooses a new Active from their Bench.

## Win conditions

41. Take all your Prize cards.
42. Opponent has no Pokémon in play when they must choose a new Active.
43. Opponent cannot draw at the start of their turn.
44. If 41 and 42 would trigger at once, **you still win**.

## Pokémon Checkup (between turns)

45. Happens after a turn ends, before the next begins.
46. Special Conditions first, then other between-turn effects.
47. You choose the order of your own effects within it.
48. Anything at 0 HP after checkup is KO'd — new Active chosen, Prize taken —
    **then** the next turn starts.

## Special Conditions

49. Only the **Active** Pokémon can have them.
50. **Asleep** — cannot attack or retreat. Checkup: flip; heads recovers.
51. **Paralyzed** — cannot attack or retreat. Recovers at the checkup **after
    its owner's next turn**.
52. **Confused** — flip before attacking; tails = attack doesn't happen and
    **3 damage counters on your own Pokémon**.
53. **Burned** — checkup: **2 damage counters**, then flip; heads removes it.
54. **Poisoned** — checkup: **1 damage counter**.
55. **Asleep / Confused / Paralyzed are mutually exclusive** — the most recent
    replaces the others (they all rotate the card).
56. **Burned and Poisoned are independent** — can coexist with each other and
    with a rotation condition.
57. A second Burn or Poison **replaces** the existing one rather than stacking.

## Stadiums & Tools

58. A Stadium stays in play; only one at a time; a new one discards the old.
59. **Cannot** play a Stadium with the same name as one already in play.
60. One Stadium played per turn.
61. **1 Pokémon Tool per Pokémon by default**, but card abilities raise the cap:
    - _Self-scoped:_ `Garbodor VMAX` "Rubbish Collecting", `Genesect GX`
      "Double Drive" — this Pokémon may have up to 2.
    - _Team-scoped:_ `Rotom ex` (me02-029) "Multi Adapter" — each of your
      Pokémon with "Rotom" in its name may have up to 2.
    - Every one carries: _"If this Ability goes away, discard Pokémon Tools
      until only 1 remains."_ So the limit is a **continuous invariant
      re-checked when an ability is lost**, not an attach-time check.

## Cards that break a naive engine

Design the engine so these stay expressible.

Design the engine so these are expressible from day one.

- **`Alakazam ex` (`sv03.5-065`, reg G) — "Dimensional Hand"**:
  _"This attack can be used even if this Pokémon is on the Bench."_
  Breaks the assumption that the attacker is the Active Pokémon (rule 29).
  Exactly 5 prints in the SV pool — rare enough to design around by accident
  and then be wrong. Open questions for the Compendium: does a Benched attacker
  apply Weakness/Resistance (rule 36 says W/R never apply to Bench)? Can it
  attack while Asleep/Paralyzed, given those are Active-only?
- **Tool limits are ability-derived and dynamic** — see rule 61.
- **Prize count is computed at KO time** — see §4.
- **Mega Evolution is NOT a new stage.** In XY, `M Alakazam EX` had
  `stage = MEGA`. In the ME era, `Mega Venusaur ex` is `Stage2` evolving from
  `Ivysaur`; `Mega Zeraora ex` and `Mega Darkrai ex` are **Basic** with 270-280
  HP. There is no `MEGA` stage in reg J. Good news for the engine: "Mega" is
  naming, not mechanics — the rulebook's "all the normal rules for Evolution
  apply" is literal.
