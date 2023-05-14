def play_player(no, game_map):
    pl = {"X": 1, "O": 2}
    pinp = input("Player {} Please enter your move (in the format A2, B1): ".format(pl[no]))
    row = ord(pinp[0].upper()) - 65
    column = int(pinp[1]) - 1
    try:
        if game_map[column][row] == " ":
            game_map[column][row] = no
            return game_map
        else:
            print("You cannot mark there!")
            return play_player(no, game_map)

    except IndexError:
        print("Please enter valid address!")
        return play_player(no, game_map)


def print_map(game_map):
    g_header = ["A", "B", "C"]
    print("  {}".format(g_header))
    for i, e in enumerate(game_map):
        print("{} {}".format(i + 1, e))


def check_won(game_map):
    win_seq = ["X", "O"]
    for i in win_seq:
        if game_map[0][0] == game_map[0][1] == game_map[0][2] == i:
            return True
        elif game_map[1][0] == game_map[1][1] == game_map[1][2] == i:
            return True
        elif game_map[2][0] == game_map[2][1] == game_map[2][2] == i:
            return True
        elif game_map[0][0] == game_map[1][0] == game_map[2][0] == i:
            return True
        elif game_map[0][1] == game_map[1][1] == game_map[2][1] == i:
            return True
        elif game_map[2][0] == game_map[2][0] == game_map[2][0] == i:
            return True
        elif game_map[0][0] == game_map[1][1] == game_map[2][2] == i:
            return True
        elif game_map[0][2] == game_map[1][1] == game_map[2][0] == i:
            return True
        else:
            return False


def check_completed(game_map):
    for i in range(0,3):
        for j in range(0,3):
            if game_map[i][j] == " ":
                return True
    return False
