'''
usage:
    python merge_toml.py <from_file_name> <to_file_name> <feature_name> [<write_file_name>]

example:
    python merge_toml.py ./openapi/Cargo.toml ./bot-core/Cargo.toml openapi ./bot-core/Cargo.tmp.toml

description:
    # Toml をマージします
    from_file_name: マージ元のファイル名
    to_file_name: マージ先のファイル名
    feature_name: マージ先での feature 名
    write_file_name: マージ先のファイル名を変更する場合に指定します

    feature_name を有効にした場合、２つの dependencies をマージした結果になり、
    有効でない場合は、to_file_name の dependencies そのままになります

    注:
        マージ先とマージ元の dependencies に同じ名前のものがある場合、バージョンはマージ先のものが優先されます
        format は完全に崩れるので、features, dependencies のみをコピペするのがおすすめ
'''

import toml
import sys

def merge_toml(from_file_name: str, to_file_name: str, feature_name: str, write_file_name: str | None = None):
    with open(from_file_name, "r") as from_file:
        from_toml = toml.load(from_file)
    with open(to_file_name, "r") as to_file:
        to_toml = toml.load(to_file)

    features = to_toml.get("features", {})
    target_features: list[str] = features.get(feature_name, [])

    to_deps = to_toml.get("dependencies", {})

    for k, v in from_toml.get("dependencies", {}).items():
        if type(v) is str:
            if k in to_deps:
                continue
            to_deps[k] = {
                "version": v,
                "optional": True,
            }
            target_features.append(k)
        else:
            if k in to_deps:
                if type(to_deps[k]) is str:
                    for feat in v.get("features", []):
                        target_features.append(f"{k}/{feat}")
                else:
                    base_features: list[str] = to_deps[k].get("features", [])
                    for feat in v.get("features", []):
                        if feat not in base_features:
                            target_features.append(f"{k}/{feat}")
            else:
                to_deps[k] = {
                    "version": v.get("version", ""),
                    "optional": True,
                }
                for feat in v.get("features", []):
                    target_features.append(f"{k}/{feat}")

    features['default'] = features.get('default', [])
    features[feature_name] = target_features

    to_toml['dependencies'] = to_deps
    to_toml['features'] = features

    write_file_name = write_file_name or to_file_name
    with open(write_file_name, "w") as to_file:
        toml.dump(to_toml, to_file, encoder=toml.TomlPreserveInlineDictEncoder())

if __name__ == "__main__":
    args = sys.argv[1:]
    if len(args) < 3:
        print("Usage: merge_toml.py <from> <to> <feature> [to_file]")
        sys.exit(1)

    merge_toml(args[0], args[1], args[2], args[3] if len(args) > 3 else None)
    print(f"Merge {args[0]} to {args[1]} with feature {args[2]}")
