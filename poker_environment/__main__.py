import os
from pathlib import Path
from typing import Callable

import poker_environment
from poker_environment.run_game import Game
import fire
import importlib.util
from time import sleep as zzz


def add_to_dict(d, k, v):
    def add_to_dict_helper(d, k, v, i):
        k_full = k + ("" if i == 0 else f"_{i}")
        if d.get(k_full) is None:
            d[k_full] = v
        else:
            add_to_dict_helper(d, k, v, i + 1)

    return add_to_dict_helper(d, k, v, 0)


def main_loop_no_tui(agents: dict[str, Callable[[poker_environment.PyPokerEnvironment], str]], delay):
    e = Game(agents)
    while not e.is_finished():
        e.advance()
        print(e)
        print()



def main_loop_tui(agents: dict[str, Callable[[poker_environment.PyPokerEnvironment], str]], delay):
    import curses
    import curses.textpad

    def unicodify_card(card: str):
        return (card
                .replace("S", "\u2660")
                .replace("H", "\u2665")
                .replace("D", "\u2666")
                .replace("C", "\u2663"))

    def draw_card(win, uly, ulx, value: str | None):
        win.addstr(uly, ulx, " _____ ")
        if value is None:
            win.addstr(uly + 1, ulx, "|# # #|")
            win.addstr(uly + 2, ulx, "| # # |")
            win.addstr(uly + 3, ulx, "|# # #|")
            win.addstr(uly + 4, ulx, "| # # |")
            win.addstr(uly + 5, ulx, "|#_#_#|")
        else:
            value = unicodify_card(value)
            match list(value):
                case [s, "2"]:
                    win.addstr(uly + 1, ulx,  "|2    |")
                    win.addstr(uly + 2, ulx, f"|  {s}  |")
                    win.addstr(uly + 3, ulx, f"|     |")
                    win.addstr(uly + 4, ulx, f"|  {s}  |")
                    win.addstr(uly + 5, ulx, f"|____2|")
                case [s, "3"]:
                    win.addstr(uly + 1, ulx,  "|3    |")
                    win.addstr(uly + 2, ulx, f"|  {s}  |")
                    win.addstr(uly + 3, ulx, f"|  {s}  |")
                    win.addstr(uly + 4, ulx, f"|  {s}  |")
                    win.addstr(uly + 5, ulx, f"|____3|")
                case [s, "4"]:
                    win.addstr(uly + 1, ulx,  "|4    |")
                    win.addstr(uly + 2, ulx, f"| {s} {s} |")
                    win.addstr(uly + 3, ulx, f"|     |")
                    win.addstr(uly + 4, ulx, f"| {s} {s} |")
                    win.addstr(uly + 5, ulx, f"|____4|")
                case [s, "5"]:
                    win.addstr(uly + 1, ulx,  "|5    |")
                    win.addstr(uly + 2, ulx, f"| {s} {s} |")
                    win.addstr(uly + 3, ulx, f"|  {s}  |")
                    win.addstr(uly + 4, ulx, f"| {s} {s} |")
                    win.addstr(uly + 5, ulx, f"|____5|")
                case [s, "6"]:
                    win.addstr(uly + 1, ulx,  "|6    |")
                    win.addstr(uly + 2, ulx, f"| {s} {s} |")
                    win.addstr(uly + 3, ulx, f"| {s} {s} |")
                    win.addstr(uly + 4, ulx, f"| {s} {s} |")
                    win.addstr(uly + 5, ulx, f"|____6|")
                case [s, "7"]:
                    win.addstr(uly + 1, ulx,  "|7    |")
                    win.addstr(uly + 2, ulx, f"| {s} {s} |")
                    win.addstr(uly + 3, ulx, f"|{s} {s} {s}|")
                    win.addstr(uly + 4, ulx, f"| {s} {s} |")
                    win.addstr(uly + 5, ulx, f"|____7|")
                case [s, "8"]:
                    win.addstr(uly + 1, ulx,  "|8    |")
                    win.addstr(uly + 2, ulx, f"|{s} {s} {s}|")
                    win.addstr(uly + 3, ulx, f"| {s} {s} |")
                    win.addstr(uly + 4, ulx, f"|{s} {s} {s}|")
                    win.addstr(uly + 5, ulx, f"|____8|")
                case [s, "9"]:
                    win.addstr(uly + 1, ulx,  "|9    |")
                    win.addstr(uly + 2, ulx, f"|{s} {s} {s}|")
                    win.addstr(uly + 3, ulx, f"|{s} {s} {s}|")
                    win.addstr(uly + 4, ulx, f"|{s} {s} {s}|")
                    win.addstr(uly + 5, ulx, f"|____9|")
                case [s, "X"]:
                    win.addstr(uly + 1, ulx, f"|10 {s} |")
                    win.addstr(uly + 2, ulx, f"|{s} {s} {s}|")
                    win.addstr(uly + 3, ulx, f"| {s} {s} |")
                    win.addstr(uly + 4, ulx, f"|{s} {s} {s}|")
                    win.addstr(uly + 5, ulx, f"|_{s}_10|")
                case [s, "J"]:
                    win.addstr(uly + 1, ulx, f"|J{s}   |")
                    win.addstr(uly + 2, ulx,  "|  o~o|")
                    win.addstr(uly + 3, ulx,  "| / J |")
                    win.addstr(uly + 4, ulx,  "| |   |")
                    win.addstr(uly + 5, ulx, f"|___{s}J|")
                case [s, "Q"]:
                    win.addstr(uly + 1, ulx, fr"|Q{s}/ \|")
                    win.addstr(uly + 2, ulx,   "|  u_u|")
                    win.addstr(uly + 3, ulx,   "| / Q |")
                    win.addstr(uly + 4, ulx,   "| |   |")
                    win.addstr(uly + 5, ulx,  f"|___{s}Q|")
                case [s, "K"]:
                    win.addstr(uly + 1, ulx, f"|K{s} MM|")
                    win.addstr(uly + 2, ulx,  "|  OwO|")
                    win.addstr(uly + 3, ulx,  "| / K |")
                    win.addstr(uly + 4, ulx,  "| |   |")
                    win.addstr(uly + 5, ulx, f"|___{s}K|")
                case [s, "A"]:
                    win.addstr(uly + 1, ulx,  "|A    |")
                    win.addstr(uly + 2, ulx,  "|     |")
                    win.addstr(uly + 3, ulx, f"|  {s}  |")
                    win.addstr(uly + 4, ulx,  "|     |")
                    win.addstr(uly + 5, ulx, f"|____A|")
                case _:
                    raise ValueError(f"Invalid card {value}")

    def draw_table(win, uly, ulx, table):
        table_copy = table[:]
        while len(table_copy) < 5:
            table_copy.append(None)
        for i,t in zip(range(0, 8 * 5, 8), table_copy):
            draw_card(win, uly, ulx + i, t)

    def draw_player(win, uly, ulx, player: poker_environment.PyPokerDealtPlayer, action : str | None):
        # Player Details
        win.addstr(uly, ulx, f"P: {player.player_id[:15]}")
        win.addstr(uly + 1, ulx, f"BAL: {player.remaining_balance} / {player.committed_balance}")
        win.addstr(uly + 2, ulx, "HAND: " + " ".join(unicodify_card(c) for c in player.hand))

        # Player Actions
        if action is not None and action.startswith("RAISE"):
            win.addstr(uly, 33 + ulx, "> RAISE", curses.A_BOLD)
        else:
            win.addstr(uly, 35 + ulx, "RAISE")
        if action is not None and action.startswith("CALL"):
            win.addstr(uly + 1, 34 + ulx, "> CALL", curses.A_BOLD)
        else:
            win.addstr(uly + 1, 36 + ulx, "CALL")
        if action is not None and action.startswith("FOLD"):
            win.addstr(uly + 2, 34 + ulx, "> FOLD", curses.A_BOLD)
        else:
            win.addstr(uly + 2, 36 + ulx, "FOLD")

    def draw_table_details(win, uly, ulx, env: poker_environment.PyPokerEnvironment):
        win.addstr(uly, ulx, f"// POT: {env.pot} // CUR.BET: {env.expected_bet} // MIN BET: {env.minimum_bet} //")

    e = Game(agents)

    def curse_runner(stdscr):
        stdscr.clear()

        pad = curses.newpad(17, 42 + 100)

        while not e.is_finished():
            pad.clear()
            environment_action = e.advance()
            if environment_action is not None:
                environment, action = environment_action
                # Table Card window
                curses.textpad.rectangle(pad, 0, 0, 8, 41)
                draw_table(pad, 1, 1, environment.table_cards)
                # Table details
                draw_table_details(pad, 9, 1,environment)
                # PlayerWindow
                curses.textpad.rectangle(pad, 10, 0, 14, 41)
                draw_player(pad, 11, 1, environment.current_player, None)

                pad.refresh(0, 0, 0, 0, curses.LINES - 1, curses.COLS - 1)
                stdscr.refresh()
                zzz(delay)

                pad.clear()

                # Table Card window
                curses.textpad.rectangle(pad, 0, 0, 8, 41)
                draw_table(pad, 1, 1, environment.table_cards)
                # Table details
                draw_table_details(pad, 9, 1,environment)
                # PlayerWindow
                curses.textpad.rectangle(pad, 10, 0, 14, 41)
                draw_player(pad, 11, 1, environment.current_player, action)
                match action.split(" "):
                    case ["RAISE", amount]:
                        pad.addstr(15, 1, f"{environment.current_player.player_id} raised by {amount}", curses.A_BOLD)
                    case ["CALL"]:
                        pad.addstr(15, 1, f"{environment.current_player.player_id} checked.", curses.A_BOLD)
                    case ["FOLD"]:
                        pad.addstr(15, 1, f"{environment.current_player.player_id} folded!", curses.A_BOLD)
                    case _:
                        raise ValueError(f"Invalid action: {action}")

            pad.refresh(0, 0, 0, 0, curses.LINES - 1, curses.COLS - 1)
            stdscr.refresh()

            zzz(delay)

    curses.wrapper(curse_runner)
    print("RESULTS:")
    for (p, (_, i)) in e.players.items():
        print(f"{p}: {i}")


def main(*bot_path_list: Path, delay=0, starting_balance=1000, no_tui = False):
    agent_list = {}
    for bot_path in bot_path_list:
        full_path = Path(bot_path)
        agent_lib_spec = importlib.util.spec_from_file_location(full_path.stem, full_path)
        agent_lib = importlib.util.module_from_spec(agent_lib_spec)
        agent_lib_spec.loader.exec_module(agent_lib)
        agent = agent_lib.execute
        add_to_dict(agent_list, full_path.stem, (agent, starting_balance))

    if no_tui:
        main_loop_no_tui(agent_list, delay)
    else:
        try:
            main_loop_tui(agent_list, delay)
        except ImportError:
            print("Could not import curses on this system, falling back to no_tui")
            main_loop_no_tui(agent_list, delay)


if __name__ == "__main__":
    fire.Fire(main)
