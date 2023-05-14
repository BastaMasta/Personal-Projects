import pickle
import pathlib

destination = str(pathlib.Path(__file__).parent.resolve()) + "\\modes\\"

mode_name = input("please input name of mode:")
m_f_destn = destination + "{}.pickle".format(mode_name.lower().replace(" ", "_"))

inp = 0
o = 1
wordies = []

print("Please keep inputting the words for the game one after the other(make sure to hit enter after each word)")
print("enter 'q' to stop")

while True:

    inp = input(f"{o}. ")
    if inp == 'q':
        break
    wordies += [str(inp.strip().lower().replace(" ", "_"))]
    o += 1

fwile = open(m_f_destn, 'wb')
pickle.dump(wordies, fwile)
