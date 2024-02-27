import random
import sys
from typing import Callable
import poker_environment
from poker_environment import fold
from poker_environment.time_limit import time_limit


class Game:
    START_MIN_BET = 2
    NUM_ROUNDS_BEFORE_INC = 5

    def __init__(self, agents: dict[str, [Callable[[],str], int]]):
        self.players = agents
        self.current_rust_game: poker_environment.PyPokerGame | None = None
        self.game_counter = 1

    def advance(self) -> None | tuple[poker_environment.PyPokerEnvironment, str]:
        if self.current_rust_game is None:
            self.current_rust_game = poker_environment.PyPokerGame(
                [poker_environment.PyPokerPlayerInfo(name, balance)
                 for name, (_, balance) in self.players.items()
                 if balance > 0],
                max(int(self.game_counter / self.NUM_ROUNDS_BEFORE_INC * 1.50 * self.START_MIN_BET), self.START_MIN_BET), random.randint(0, sys.maxsize)
            )
            self.game_counter += 1
            return None
        else:
            current_environment = self.current_rust_game.get_environment()
            current_player_name = current_environment.current_player.player_id
            # noinspection PyBroadException
            try:
                with time_limit(20):
                    action = self.players[current_player_name][0](current_environment)
            except:
                action = fold()

            self.current_rust_game.advance(action)

            if self.current_rust_game.is_finished():
                (actives, folders) = self.current_rust_game.get_players()
                for ap in actives:
                    self.players[ap.player_id] = (self.players[ap.player_id][0], ap.remaining_balance)
                for fp in folders:
                    self.players[fp.player_id] = (self.players[ap.player_id][0], fp.balance)
                self.current_rust_game = None
            return current_environment, action

    def is_finished(self) -> bool:
        return sum(1 for (_, balance) in self.players.values() if balance > 0) == 1

    def __str__(self):
        return '\n'.join(f"{name}: {balance}" for name, (_, balance) in self.players.items())


if __name__ == "__main__":
    e = Game([(f"Binks {i + 1}", lambda x : "CALL", 20) for i in range(6)])

    while not e.is_finished():
        e.advance()
        print(e)
        print()

    print("Winner:", next(iter(e.players.keys())))
