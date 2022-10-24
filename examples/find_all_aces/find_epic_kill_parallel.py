from demoparser import DemoParser
import glob
import multiprocessing as mp
import pandas as pd
import tqdm


def parse(file):

    number_of_kills = 5

    parser = DemoParser(file)
    df = parser.parse_events("")
    print(df)
    # end of "parser" after this its just pandas operations.

    # Remove warmup rounds. Seems like something odd going on with round "1" so drop

    # Here we can include any other filters like weapons etc.
    # df = df[df["weapon"] == "ak47"]

    all_rounds = []
    players = df["attacker_id"].unique()
    for player in players:
        # slice players
        player_df = df[df["attacker_id"] == player]
        # Create df like this: ["round", "attacker_name", "total_kills"]
        kills_per_round = player_df.groupby(["round", "attacker_name", "attacker_id"]).size().to_frame(name = 'total_kills').reset_index()
        # Get rounds with kills > n
        kills_per_round = kills_per_round[kills_per_round["total_kills"] >= number_of_kills]
        # Make sure all ids are fine
        kills_per_round = kills_per_round[kills_per_round["attacker_id"] > 7651111111] # make sure id is ok
        # Add file name to df
        kills_per_round["file"] = file
        # Put all of those rounds in an output list
        all_rounds.append(kills_per_round)
    return pd.concat(all_rounds)


if __name__ == "__main__":
    from collections import Counter
    files = glob.glob("/home/laiho/Documents/demos/faceits/cu/*")
    with mp.Pool(processes=8) as pool:
        results = list(tqdm.tqdm(pool.imap_unordered(parse, files), total=len(files)))

    df = pd.concat(results)
    print(Counter(df["round"]))