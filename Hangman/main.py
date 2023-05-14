import random
from draw_man import *
from mode_selection import *

title = ["H", "A", "N", "G", "M", "A", "N"]
life = 6
corr = 0
ans = []
inp = ""
guessed = []

print_hangyman(life=69)
print("-----------------Mode Selection-----------------\n")

words = select_mode()
s_word = random.choice(words)
for i in s_word:
    ans += ["_"]

clear()

# Main Game loop
while (life > 0) and (corr < len(s_word)):

    print("")
    print("")
    print_hangyman(life=69)
    print_hangyman(life=life)
    print("             {0} \n\n ".format(ans))

    inp = input("Enter a letter:\n").lower()

    while inp in guessed:
        print("You have already guessed that!")
        inp = input("Try another letter:\n").lower()

    clear()
    flaggy = 0

    if s_word == inp:
        corr = len(s_word)
        for i in range(0, len(s_word)):
            ans[i] = inp[i]

    else:
        for i in range(0, len(s_word)):
            if s_word[i] == inp:
                ans[i] = inp
                corr += 1
                flaggy = min(1, flaggy + 1)

        if flaggy <= 0:
            print("WRONG GUESS!")
            life -= 1

    if life <= 0:
        print_hangyman(life=69)
        print_hangyman(life=0)
        print("YOU LOST!!!!!!")
        print("The man was hanged")
        print("P.S. the word was {0}".format(s_word))

    if corr >= len(s_word):
        print_hangyman(life=69)
        print("YOU WON!!!")
        print("             {0} ".format(ans))
        print("You saved the man from being hanged!!!")

    guessed += [inp]
