import os
from pathlib import Path
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


def main(*bot_path_list: Path, delay=0, starting_balance=1000):
    agent_list = {}
    for bot_path in bot_path_list:
        full_path = Path(bot_path)
        agent_lib_spec = importlib.util.spec_from_file_location(full_path.stem, full_path)
        agent_lib = importlib.util.module_from_spec(agent_lib_spec)
        agent_lib_spec.loader.exec_module(agent_lib)
        agent = agent_lib.execute
        add_to_dict(agent_list, full_path.stem, (agent, starting_balance))

    e = Game(agent_list)
    while not e.is_finished():
        e.advance()
        print(e)
        zzz(delay)
        print()


if __name__ == "__main__":
    if os.name == "nt":
        print("You smell!")
    fire.Fire(main)
