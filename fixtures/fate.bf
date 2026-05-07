FATE in Brainfuck
=================
The five model selector with context dependent dispatch

Models: 0=Abyss 1=Cartographer 2=Pathfinder 3=Explorer 4=Fate

Tape layout:
  Cell  0 to 15 : 16 input features
  Cell 16       : current model index (input)
  Cell 17 to 21 : 5 output scores (Abyss Cart Path Expl Fate)
  Cell 22       : dispatch work cell
  Cell 23       : flag for zero test
  Cell 24 to 26 : scratch
  Cell 28       : argmax result index
  Cell 30       : model index backup copy

Weight set (hardcoded):
  After Abyss(0)       : bias Cartographer(1) = 10
  After Cartographer(1): bias Pathfinder(2)  = 10
  After Pathfinder(2)  : bias Explorer(3)    = 10
  After Explorer(3)    : bias Fate(4)        = 10
  After Fate(4)        : bias Abyss(0)       = 10
  Feature 0 contributes weight 1 to Pathfinder score

PHASE 1 Read 17 input bytes
============================
Read features 0 to 15 into cells 0 to 15
Read model index into cell 16
End with pointer at cell 16

,>,>,>,>,>,>,>,>,>,>,>,>,>,>,>,>,

dp is now 16 (last comma reads cell 16)

PHASE 2 Copy model index to work cells
=======================================
dp = 16

Copy cell 16 to cell 22 using cell 23 as temp
From cell 16 to cell 22 is plus 6
From cell 16 to cell 23 is plus 7

[>>>>>>+>+<<<<<<<-]

dp = 16 (loop ends here when cell 16 = 0)
Move to cell 23 (7 right of 16)

>>>>>>>

dp = 23
Move cell 23 back to cell 16

[-<<<<<<<+>>>>>>>]

dp = 23 (loop ends here)
Go back to cell 16

<<<<<<<

dp = 16
Now cell 16 = original value cell 22 = copy

Copy cell 16 to cell 30 using cell 31 as temp
From 16 to 30 is plus 14
From 16 to 31 is plus 15

[>>>>>>>>>>>>>>+>+<<<<<<<<<<<<<<<-]

dp = 16
Go to cell 31

>>>>>>>>>>>>>>>

dp = 31
Move cell 31 to cell 16

[-<<<<<<<<<<<<<<<+>>>>>>>>>>>>>>>]

dp = 31
Go to cell 16

<<<<<<<<<<<<<<<

dp = 16
Now cell 16 cell 22 cell 30 all have the model index

Move to cell 22 (6 right)

>>>>>>

dp = 22

PHASE 3 Dispatch on model index
================================
Cell 22 has model index 0 to 4
For each case N check if cell 22 equals N
If yes load biases into cells 17 to 21

CASE 0 check if cell 22 == 0
Flag technique: cell 23 = 1 then if cell 22 nonzero clear flag

>+<

dp = 22 cell 23 = 1

[>-<[-]]

if cell 22 was nonzero: cell 23 = 0 cell 22 = 0
if cell 22 was zero: nothing happened cell 23 still = 1
dp = 22

>

dp = 23

[

if flag = 1 then model was 0 load biases
Cell 18 is 5 left of cell 23

<<<<<++++++++++>>>>>

cell 18 = 10 back to cell 23

-

clear flag

]

dp = 23

<

dp = 22

CASE 1 check if original model == 1
Restore cell 22 from cell 30 then subtract 1

Move to cell 30 (8 right of 22)

>>>>>>>>

dp = 30
Copy cell 30 to cell 22 and cell 31
From 30: 8 left = 22 and 1 right = 31

[<<<<<<<<+>>>>>>>>>+<-]

dp = 30 (ends here when 30 = 0)
Move to cell 31 (1 right)

>

dp = 31
Move cell 31 back to cell 30

[-<+>]

dp = 31
Go to cell 22: 31 minus 22 = 9 left

<<<<<<<<<

dp = 22
Cell 22 now has original model index
Subtract 1

-

cell 22 = model minus 1 (zero if model was 1)

>+<[>-<[-]]>

dp = 23

[

model was 1 Load Cartographer biases
Cell 19 is 4 left of cell 23

<<<<++++++++++>>>>

cell 19 = 10

-
]
<

dp = 22

CASE 2 check if original model == 2

>>>>>>>>
[<<<<<<<<+>>>>>>>>>+<-]
>
[-<+>]
<<<<<<<<<

dp = 22 cell 22 = original model
Subtract 2

--

>+<[>-<[-]]>

dp = 23

[
<<<++++++++++>>>
-
]
<

dp = 22

CASE 3 check if original model == 3

>>>>>>>>
[<<<<<<<<+>>>>>>>>>+<-]
>
[-<+>]
<<<<<<<<<

dp = 22
Subtract 3

---

>+<[>-<[-]]>

dp = 23

[
<<++++++++++>>
-
]
<

dp = 22

CASE 4 check if original model == 4

>>>>>>>>
[<<<<<<<<+>>>>>>>>>+<-]
>
[-<+>]
<<<<<<<<<

dp = 22
Subtract 4

----

>+<[>-<[-]]>

dp = 23

[

model was 4 Load Fate biases
Cell 17 is 6 left of cell 23

<<<<<<++++++++++>>>>>>

cell 17 = 10

-
]
<

dp = 22

Bias loading complete
Cells 17 to 21 now have scores

PHASE 4 Add feature contribution
=================================
Feature 0 (cell 0) contributes weight 1 to Pathfinder (cell 19)
Destructive move: add cell 0 to cell 19

Move from cell 22 to cell 0: 22 lefts

<<<<<<<<<<<<<<<<<<<<<<

dp = 0

Move cell 0 value to cell 19: 19 rights

[>>>>>>>>>>>>>>>>>>>+<<<<<<<<<<<<<<<<<<<-]

dp = 0 (cell 0 now = 0 cell 19 += old cell 0)

Move from cell 0 to cell 17: 17 rights

>>>>>>>>>>>>>>>>>

dp = 17

PHASE 5 Argmax last nonzero wins
==================================
Cells 17 to 21 have scores
Find the index of the last nonzero cell
Result in cell 28

dp = 17

First clear cell 28: 11 right of 17

>>>>>>>>>>>[-]<<<<<<<<<<<

dp = 17

Check cell 17 (Abyss index = 0)
If nonzero set cell 28 = 0 and clear cell 17

[>>>>>>>>>>>[-]<<<<<<<<<<<[-]]

dp = 17

Move to cell 18

>

dp = 18

Check cell 18 (Cartographer index = 1)

[>>>>>>>>>>[-]+<<<<<<<<<<[-]]

dp = 18

>

dp = 19

Check cell 19 (Pathfinder index = 2)

[>>>>>>>>>[-]++<<<<<<<<<[-]]

dp = 19

>

dp = 20

Check cell 20 (Explorer index = 3)

[>>>>>>>>[-]+++<<<<<<<<[-]]

dp = 20

>

dp = 21

Check cell 21 (Fate index = 4)

[>>>>>>>[-]++++<<<<<<<[-]]

dp = 21

PHASE 6 Output result
======================
Cell 28 has the winning model index
From cell 21 to cell 28: 7 right

>>>>>>>
.

Done
