import tomlkit
import sys
from typing import cast, TypedDict

class CargoWorkspace(TypedDict):
    members: list[str]

class TOML(TypedDict):
    workspace: CargoWorkspace

if len(sys.argv) != 2:
  raise UserWarning("Missing/invalid binary crate name, exiting")
crate = sys.argv[1]

with open('Cargo.toml') as f:
  toml = cast(TOML, tomlkit.load(f))
  members = toml['workspace']['members']
  if crate not in members:
    members.append(crate)
    with open('Cargo.toml', 'w') as f:
      tomlkit.dump(toml, f)
    print(f'Added crate {crate} to Cargo.toml')
  else:
    print(f'Crate {crate} already present in Cargo.toml')
