import poker_environment

def bink_bot(env):
    return "CALL"

class PokerEnvironmentInactive:
    agents = {}
    game = None

    def register_agent(self, name: str, agent):
        if self.game is not None:
            raise RuntimeError("Cannot modify agents once a game has started")
        self.agents[name] = (agent, 20)

    def start_game(self):
        players = [poker_environment.PyPlayerInfo(name, balance) for name, (_, balance) in self.agents.items()]

        self.game = poker_environment.PyPokerGame(players, 2, 57)

    def mainloop(self):
        while True:
            env = self.game.get_environment()
            print(env.current_player.player_id)
            action = self.agents[env.current_player.player_id][0](env)
            state = self.game.advance(action)
            if state is not None:
                return state

if __name__ == "__main__":
    e = PokerEnvironmentInactive()
    for i in range(6):
        e.register_agent(f"Binks {i + 1}", bink_bot)

    e.start_game()
    fp = e.mainloop()
    for ff in fp:
        print(f"{ff.player_id} : {ff.balance}")

    assert sum(ff.balance for ff in fp) == 6 * 20
