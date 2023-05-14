import os
import pathlib
import pickle
import time


def select_mode():
    path = str(pathlib.Path(__file__).parent.resolve()) + "\\modes"

    modes = os.listdir(path)
    mode_names = []

    for i in range(0, len(modes)):
        mode_names += [os.path.splitext(modes[i])[0]]

    if not mode_names:
        print("No Files in modes directory...")
        print("defaultinng to built-in mode(s)....")
        print("loading built-in mode(s).....")
        time.sleep(3)
        mode_words = ["apple", "banana", "orange", "pineapple", "peach", "kiwi", "mango", "watermelon", "jackfruit",
                      "lemon", "strawberry", "apricot", "grapes", "muskmelon", "tomato", "potato"]
        print("Built-in mode loaded --> Fruits(?)")
        print("Starting game..........")
        time.sleep(5)

    else:
        print("Loading modes....")
        time.sleep(3)
        print("Choose the mode of the game:")
        for i in range(0, len(modes)):
            print("{0}. {1}".format(i+1, mode_names[i].replace("_", " ").title()))

        u_mode = input("Enter the index you want:")
        if u_mode == "default":
            mode_words = ["apple", "banana", "orange", "pineapple", "peach", "kiwi", "mango", "watermelon", "jackfruit",
                          "lemon", "strawberry", "apricot", "grapes", "muskmelon", "tomato", "potato"]
        else:
            mode_file = open(str(pathlib.Path(__file__).parent.resolve()) + "\\modes\\{}".format(modes[int(u_mode)-1]), 'rb')
            mode_words = pickle.load(mode_file)

        print("Mode Selected!")
        print("loading game...")

        time.sleep(5)

    return mode_words
