"""Generates assets/BalanceData/75689, the balance data of 4.10 (the Linux client of AI Arena),
from assets/BalanceData/94137 with the ability links of 4.10.

The Editor only exports the balance data of the current patch, there is no export of 4.10. The
units of 94137 are kept: their costs, strengths and weaknesses are the ones of 5.0.11, not of
4.10. Their ability links are moved to the ones of 4.10, which has two abilities less before the
sprays and one less before the stop, move and attack abilities:

    link in 4.10 = link in 94137 - 1 until 46 (attack), - 2 from 62 (SprayTerran)

This was measured on 200 AI Arena replays: the units commanding each link have the ability of
that link (i.e. 42 move by the workers, 721 ChronoBoostEnergyCost by the Nexus, 691
BattlecruiserAttack), the Zerglings commanding 728 (MorphToBaneling, 730 in 94137) become Baneling
cocoons as many times, and the abilities named in both 76114_abilities.csv (the closest build in
sc2reader, https://github.com/ggtracker/sc2reader, MIT) and 96883_abilities.csv (the one that
names the links of 94137) differ by these offsets, all 397 of them. The units use no links below
40 or between 47 and 61.

The commands of an ability that 94137 does not have, i.e. the ScanMove (3) of move that the bots
use, are added with the names of sc2reader's ability_lookup.csv.

    python -I scripts/balance_data_75689/generate.py
"""

import csv
import json
from pathlib import Path

HERE = Path(__file__).parent
ROOT = HERE.parent.parent
SOURCE = ROOT / "assets/BalanceData/94137"
TARGET = ROOT / "assets/BalanceData/75689"

# The last link of 94137 that is one more in 4.10, the following ones are two more.
LAST_LINK_ONE_MORE = 46


def relink(link_94137: int | None) -> int | None:
    if link_94137 is None:
        return None
    return link_94137 - (1 if link_94137 <= LAST_LINK_ONE_MORE else 2)


def read_links(build: int) -> dict[str, int]:
    with open(HERE / f"{build}_abilities.csv") as file:
        return {row[1]: int(row[0]) for row in csv.reader(file) if row and row[0].isdigit()}


def check_with_sc2reader() -> None:
    """The abilities that sc2reader names in both builds have the links of the offsets."""
    links_94137, links_76114 = read_links(96883), read_links(76114)
    common = set(links_94137) & set(links_76114)
    wrong = sorted(name for name in common if relink(links_94137[name]) != links_76114[name])
    # 76114 is a later build than 75689, from its BattlecruiserAttack (690) on it has the
    # abilities at other links, the replays of 4.10 use the ones of the offsets.
    wrong = [name for name in wrong if links_76114[name] < 688]
    assert not wrong, f"Links that do not follow the offsets: {wrong}"


def read_command_names() -> dict[str, list[str]]:
    """The names of the commands of each ability, by command index."""
    with open(HERE / "ability_lookup.csv") as file:
        return {row[0]: row[1:] for row in csv.reader(file) if row}


def command(name: str, index: int) -> dict:
    # The name and tooltip are ids of the strings of the game, the command has none.
    meta = {key: None for key in ("@hotkey", "@icon", "@index", "@race", "@source")}
    return {"@id": name, "@index": index, "meta": {**meta, "@name": "", "@tooltip": "", "@type": "command"}}


# The abilities of 94137 that sc2reader names otherwise, for their command names.
COMMAND_ALIASES = {"NydusWormTransport": "NydusCanalTransport"}


def main() -> None:
    check_with_sc2reader()
    command_names = read_command_names()
    ability_names = {link: name for name, link in read_links(96883).items()}
    TARGET.mkdir(exist_ok=True)
    added_commands = 0
    for path in sorted(SOURCE.glob("*.json")):
        unit = json.loads(path.read_text())
        if not unit.get("abilities"):
            unit["abilities"] = {"ability": []}
        abilities = unit["abilities"].setdefault("ability", [])
        for ability in abilities:
            ability["@index"] = relink(ability.get("@index"))
            commands = ability.setdefault("command", [])
            indexes = {existing["@index"] for existing in commands}
            names = command_names.get(COMMAND_ALIASES.get(ability["@id"], ability["@id"]), [])
            for index, name in enumerate(names):
                if name and index not in indexes:
                    commands.append(command(name, index))
                    added_commands += 1
        # The trains, builds and researches have the commands that make something, the other
        # commands of their abilities (i.e. the cancel of TerranBuild) are added as abilities.
        produced: dict[int, set[int]] = {}
        for tag, key in (("trains", "unit"), ("builds", "unit"), ("researches", "upgrade")):
            for entry in (unit.get(tag) or {}).get(key) or []:
                if entry.get("@ability") is not None:
                    produced.setdefault(entry["@ability"], set()).add(entry["@index"])
                    entry["@ability"] = relink(entry["@ability"])
        listed = {ability["@index"] for ability in abilities}
        for link, indexes in sorted(produced.items()):
            name = ability_names.get(link)
            other = [
                command(command_name, index)
                for index, command_name in enumerate(command_names.get(name, []))
                if command_name and index not in indexes
            ]
            if name and other and relink(link) not in listed:
                abilities.append({"@id": name, "@index": relink(link), "command": other})
                added_commands += len(other)
        (TARGET / path.name).write_text(json.dumps(unit, indent=2))
    print(f"Wrote {TARGET}, {added_commands} commands added from sc2reader")


if __name__ == "__main__":
    main()
