import json
import glob
import argparse
import statistics
from collections import defaultdict
from pathlib import Path

def load_and_aggregate(json_files):
    aggregate = {}

    for file in json_files:
        with open(file, 'r') as f:
            reports = json.load(f)
            for report in reports:
                contract = report["contract"]
                if contract not in aggregate:
                    aggregate[contract] = {
                        "deployment_gas": [],
                        "deployment_size": [],
                        "functions": defaultdict(lambda: defaultdict(list))
                    }

                aggregate[contract]["deployment_gas"].append(report["deployment"]["gas"])
                aggregate[contract]["deployment_size"].append(report["deployment"]["size"])

                for func_name, func_data in report["functions"].items():
                    for key, value in func_data.items():
                        aggregate[contract]["functions"][func_name][key].append(value)

    return aggregate

def compute_final_report(aggregate):
    final_report = []

    for contract, data in aggregate.items():
        contract_entry = {
            "contract": contract,
            "deployment": {
                "gas": round(statistics.mean(data["deployment_gas"])),
                "size": round(statistics.mean(data["deployment_size"]))
            },
            "functions": {}
        }

        for func_name, stats in data["functions"].items():
            contract_entry["functions"][func_name] = {
                "calls": sum(stats["calls"]),
                "min": round(statistics.mean(stats["min"])),
                "mean": round(statistics.mean(stats["mean"])),
                "median": round(statistics.mean(stats["median"])),
                "max": round(statistics.mean(stats["max"]))
            }

        final_report.append(contract_entry)

    return final_report

def main():
    parser = argparse.ArgumentParser(description="Merge multiple Forge gas report JSONs into a single averaged report.")
    parser.add_argument("input_dir", help="Directory containing gas report JSON files")
    parser.add_argument("output_file", help="Filename for the merged JSON report")
    args = parser.parse_args()

    json_files = glob.glob(f"{args.input_dir}/*.json")
    if not json_files:
        print(f"No JSON files found in {args.input_dir}")
        return

    aggregate = load_and_aggregate(json_files)
    final_report = compute_final_report(aggregate)

    with open(args.output_file, "w") as out:
        json.dump(final_report, out, indent=2)

    print(f"Averaged report saved to {args.output_file}")

if __name__ == "__main__":
    main()
