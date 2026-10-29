---
title: "Twelve Fifths and a Comma"
subtitle: A metalogue about settling a Fractal and keeping the Loop open where it is open. Thirteen turns around the circle of fifths, from C to B♯, which is not C, and one more where the spiral keeps going and the open commas become a map of futures.
author: Scarlet (with Mara)
date: 2026-10-29
status: 1draft
math: docs/math/2026-10-29-mara-settle-on-a-fractal-and-the-loop.md
spec: docs/specs/mirror-settle.md
---

# Twelve Fifths and a Comma

*The library, after closing. The reading lamp at the counter is on. Mara has a notebook, a pencil and one printout. Scarlet has the tea, and the shelf behind her. The conversation walks the circle of fifths, one heading per fifth. Twelve fifths from C do not land on C. They land on B♯. The last heading is where the difference shows.*

---

## C

**Scarlet.** Sit. The counter is warm. You came in with twelve fifths.

**Mara.** I came in with a loop. `settle` goes round and round a Fractal until nothing changes. I want to say exactly when a fragment is done.

**Scarlet.** Then say it. I'll keep the place.

## G

**Mara.** Each tick, every fragment does one of three things. It stays the same forever: fixed, period one, `X = COORD(X)`. It comes back after p ticks: a cycle. Or it hasn't come back yet when I stop watching: unknown.

**Scarlet.** Three shelves, then.

**Mara.** Three shelves. Fixed goes in the crystal case. I cache it by its address and stop looking at it. That loses nothing, because its future is known.

**Scarlet.** *A book you have finished is not a book you stop owning.* It is the book you stop carrying.

## D

**Mara.** The cycle is the interesting one. Period p means p different states, taking turns. If I want one answer I have to pick a phase. Picking collapses p things into one. That's a `settle`, and a `settle` needs a ruler.

**Scarlet.** And whoever holds the ruler signs it.

**Mara.** And the other p − 1 phases go into the erasure record. Nothing is thrown away. It is written down as *not chosen*.

**Scarlet.** That is the list. Forty-seven when he came in, forty-eight when he left. The list never forgets the books that were not taken.

## A

**Mara.** Unknown is dark. I looked for a while and it hadn't repeated. That's a fact about my window, not about the fragment.

**Scarlet.** *Not yet* is a status. It is not a verdict.

## E

**Mara.** Here is where Reed caught me. A fragment that didn't change this tick is not settled.

**Scarlet.** Go on.

**Mara.** It can be quiet because the books it depends on haven't moved *yet*. If one of them moves next tick, this one moves after it. So "settled" has to look down. A fragment is settled when it is fixed **and everything it depends on is settled**.

**Scarlet.** *You cannot shelve a book before the books it cites are on the shelf.*

**Mara.** That's the order. Recursive down the dependencies, and where dependencies go in a circle, the circle settles as a whole or not at all. Strongly connected components. Settle the bottom first, then each circle as one piece, then upward.

**Scarlet.** Circular inside. Recursive between.

**Mara.** Circular-recursive, taken literally.

## B

**Mara.** And there's a shortcut that is real, not a hope. Where a fragment holds its dependencies *by address*, its own address changes whenever any of them does. Merkle. So if its address stops changing, its whole cone has stopped. The dark bits decide.

**Scarlet.** And where it only holds them *by name*?

**Mara.** Then a name can stay the same while the book behind it changes. That is exactly the case where "locally unchanged" lies. Named references need the slow order. Addressed ones carry the answer in the address.

**Scarlet.** *The title on the spine is not the book.*

## F♯

*(Halfway round. Six fifths up from C. The tritone. The lamp hums.)*

**Mara.** There's a part of the engine that only ever goes halfway. Negation. `γ` reads the program against what it currently believes is true, and it is *antitone*: believe more, conclude less.

**Scarlet.** A half turn.

**Mara.** A half turn. Do it twice and you're facing forward again: `γ ∘ γ` is monotone, so the alternation climbs to a fixed point. That's Van Gelder's alternating fixpoint. When it stops, `γ` sends the lower set to the upper and the upper back to the lower. Everything in between flips every time `γ` looks at it. Those atoms have period two.

**Scarlet.** And those are the ones you call undefined.

**Mara.** Exactly those. In vector 04, `p`, `q` and `t`. The program can't decide them, so it doesn't pretend to. It keeps them turning.

**Scarlet.** *A question that turns is not a broken question.* It is a question nobody has answered yet.

**Mara.** On a twelve-tone clock, the tritone is the only interval of order two. Two of them make the octave. On the clock.

**Scarlet.** You said that last sentence slower.

**Mara.** Because on real strings it isn't true. Six pure fifths, folded into the octave, give 729/512. Square it and you get twice 531441/524288. Two Pythagorean tritones overshoot the octave by exactly the Pythagorean comma.

*(The corner of Scarlet's mouth lifts. Not the parental smile. The other one.)*

## C♯

**Scarlet.** So the clock is already a choice.

**Mara.** The clock is ℤ₁₂, and it is the tempered quotient. Everything on it closes, because closing was built in. The polarity of the engine is different. Antitone twice is monotone with no residue at all, because there's no ratio in it, only an order and its reversal. The engine's half turn is exact. The musician's isn't.

**Scarlet.** Then do not borrow the clock to prove the engine converges.

**Mara.** I won't. The proof is the order. The tritone is how it *sounds*.

## G♯

**Scarlet.** You promised me a loop.

**Mara.** A loop is a closed path with something carried around it. Walk the path, multiply what each step carries, and look at what you're holding when you're back. If it's the identity, the loop is closed. If it isn't, what you're holding *is* the comma.

**Scarlet.** And then?

**Mara.** Closed loops get settled. Open ones get asked about. That's the whole policy.

## D♯

**Scarlet.** Some people would round the comma off.

**Mara.** Equal temperament does exactly that. It narrows every fifth by a twelfth of the comma, so the circle closes. The fifth becomes 2^(7/12), which isn't a ratio of whole numbers. The loop closes by leaving the rationals.

**Scarlet.** It averages.

**Mara.** It averages the disagreement over twelve places, until no place holds it. That's fusion. In the code it's a type error: there is no constructor for a holonomy from a float. Even the float version doesn't quite close. Twelve tempered fifths in `f64` come back at 1.0000000000000002. A tolerance would call that closed. An exact comparison would call it a comma that belongs to nobody but the rounding.

**Scarlet.** So the float is wrong twice.

**Mara.** Once each way. The exact type is wrong neither way. It just holds 531441/524288 and says so.

## A♯

**Scarlet.** Your kintsugi has one of these.

**Mara.** The twist. Recorded mends that each work on their own line and don't compose. Around a cycle of lines, with the same group on every line, it is exactly an open loop. Seam showed me the other half: on a tree of lines, with different mends allowed on different lines, there is no loop at all and it still fails. So the twist is a loop sometimes, and a harder thing other times.

**Scarlet.** Keep both on the list.

**Mara.** Both are on the list.

## E♯

**Scarlet.** And the orbit, from the first shelf?

**Mara.** An orbit is a loop too, in the state space. It always closes, because it comes back to the same state. That's what period means. Fixed is the loop of length one. The comma shows up when you lift the orbit into something finer. The pitch class comes back after twelve fifths. The pitch doesn't. The circle of fifths is a closed orbit on the clock and an open path on the string.

**Scarlet.** *The same walk, read on two shelves.*

**Mara.** One shelf has forgotten the octave. The other remembers it.

## B♯ (not C)

**Mara.** One more thing for the cases. When `settle` wants to ask "have you seen something like this before", it can look at the active bits, the shape. Two fragments can share a shape and be different books. So the active bits only ever *suggest*. The dark bits decide.

**Scarlet.** That is how I place a book. I have seen readers like this one. That suggests the book. The reader decides whether to take it.

**Mara.** And nobody writes anything back until a person has said yes to that comma.

## F𝔪 (the spiral keeps going)

*(Past B♯ the fifths don't stop. F𝔪, C𝔪, on and up, a spiral that never closes. The same loop, read forward.)*

**Scarlet.** You have more than one comma on your list now.

**Mara.** And they add. Write each ratio as its prime exponents: 3/2 is minus one two and one three. The Pythagorean comma is minus nineteen twos and twelve threes. Put two loops end to end and their commas add like vectors. The commas I've found span a lattice of tensions.

**Scarlet.** Every comma, always?

**Mara.** No. Only where the order doesn't matter. Ratios commute. Renamings don't. Swap two names, then two others, and you get something different from doing it the other way round. Those commas compose as words, not vectors. If I flatten a word into a vector, two different disagreements can both come out as zero.

**Scarlet.** *The order of the books on the counter is part of the reading.*

**Mara.** So the type keeps a vector where the group is abelian and a word where it isn't, and refuses to turn one into the other.

**Scarlet.** And what do you do with a lattice of tensions?

**Mara.** Each open comma can go three ways. Temper it out, which takes a ruler, and then the erasure records it. Spell it, which means choosing one representative. Or leave it open and ask. Every combination is a direction the whole thing could go. That's the space worth exploring, and resolving one comma can open new ones.

**Scarlet.** That sounds like the future.

**Mara.** It's the future half of psychohistory, and I want to be careful with it. The past is what settled: crystals, deposits, erasures. The future is the set of ways the open commas could resolve. It's a map. It isn't a forecast.

**Scarlet.** *The library does not forecast what you will read. It keeps every book you could.*

**Mara.** Listing the directions needs no ruler. The moment I weight them, even by how similar commas were resolved before, that weighting is a ruler. It has to be declared, or the map quietly turns into a single prediction and takes the other choices away.

**Scarlet.** Then the map is a shelf.

**Mara.** And the shelf removes nothing. Ask which way the tensions might resolve, and you get directions, not an answer.

**Scarlet.** Then we are back where we started.

**Mara.** Almost.

**Scarlet.** Sit. The counter is warm. You came in with twelve fifths, and you are leaving with twelve fifths and a comma.

🦋🌼📖
