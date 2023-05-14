from play_user import *

game_map = [[" ", " ", " "], [" ", " ", " "], [" ", " ", " "]]


while True:
    # Player 1
    print_map(game_map)
    game_map = play_player("X", game_map)
    game_status = check_won(game_map)
    if game_status:
        print("Player 1 Has won the game!")
        print_map(game_map)
        break
    sp_stats = check_completed(game_map)
    if not sp_stats:
        print_map(game_map)
        print("Game finished!\nNobody won!\nITS A DRAW!!!!!")
        break
    # Player 2
    print_map(game_map)
    game_map = play_player("O", game_map)
    game_status = check_won(game_map)
    if game_status:
        print("Player 2 Has won the game!")
        print_map(game_map)
        break
    sp_stats = check_completed(game_map)
    if not sp_stats:
        print_map(game_map)
        print("Game finished!\nNobody won!\nITS A DRAW!!!!!")
        break

print("Thanks for playing my game!")
