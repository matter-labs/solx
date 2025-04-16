import json
from pathlib import Path
import pandas as pd

# === Config ===

DATA_DIR = Path("data")  # Update if needed

# File paths are hardcoded for now
FILE_MAP = {
    "solc": "solc.json",
    "solc via-ir": "solc--via-ir.json",
    "solx": "solx.json",
    "solx via-ir": "solx--via-ir.json"
}


# === Helpers ===

def pct_diff(base, new):
    if base is None or new is None or base == 0:
        return "N/A", 0.0, None
    diff = (new - base) / base * 100
    return f"{diff:.2f}%", abs(diff), diff

def decorate(diff_str, raw_val):
    if raw_val is None:
        return diff_str
    return f"{diff_str} :white_check_mark:" if raw_val < 0 else f"{diff_str} :red_circle:"

def summarize_contracts(diffs_dict):
    def format_val(val):
        return f"{val:.2f}% :white_check_mark:" if val > 0 else f"{val:.2f}% :red_circle:"

    df = pd.DataFrame([
        {"Test": contract, "gas diff, %": format_val(sum(vals)/len(vals))}
        for contract, vals in diffs_dict.items()
    ])
    df["__sort"] = df["gas diff, %"].str.extract(r"([-+]?\d*\.\d+|\d+)").astype(float)
    return df.sort_values("__sort", ascending=False).drop(columns="__sort")


# === Load and Parse Data ===

compiler_data = {}
for compiler, filename in FILE_MAP.items():
    with open(DATA_DIR / filename) as f:
        content = json.load(f)
        if isinstance(content, list):
            compiler_data[compiler] = {entry['contract']: entry['functions'] for entry in content}
        else:
            compiler_data[compiler] = {content['contract']: content['functions']}

mean_table = {}
for compiler, contracts in compiler_data.items():
    for contract, functions in contracts.items():
        contract_name = contract.split(":")[-1]
        for fn, metrics in functions.items():
            key = (contract_name, fn)
            mean_table.setdefault(key, {})[compiler] = metrics["mean"]


# === Build Summary Rows ===

summary_rows = []
diffs_solc_vs_solx = []
diffs_solc_ir_vs_solx_ir = []
better_solx = better_solx_ir = 0

for (contract, fn), results in mean_table.items():
    solc, solc_ir = results.get("solc"), results.get("solc via-ir")
    solx, solx_ir = results.get("solx"), results.get("solx via-ir")

    diff1_str, sort1, raw1 = pct_diff(solc, solx)
    diff2_str, sort2, raw2 = pct_diff(solc_ir, solx_ir)

    if raw1 is not None:
        diffs_solc_vs_solx.append(raw1)
        if raw1 < 0:
            better_solx += 1
    if raw2 is not None:
        diffs_solc_ir_vs_solx_ir.append(raw2)
        if raw2 < 0:
            better_solx_ir += 1

    summary_rows.append({
        "Test": contract,
        "Function": fn,
        "evmla gas diff, %": decorate(diff1_str, raw1),
        "Yul gas diff, %": decorate(diff2_str, raw2),
        "_solc_vs_solx_raw": raw1 or 0.0,
        "_solc_ir_vs_solx_ir_raw": raw2 or 0.0,
        "_sort_diff": max(sort1, sort2)
    })


# === 1. Summary Table ===

df = pd.DataFrame(summary_rows)
total_functions = len(df)

summary_data = {
    "Codegen": ['evmla', 'Yul'],
    "Total functions": [total_functions] * 2,
    "Functions improved": [better_solx, better_solx_ir],
    "Functions regressed": [
        sum(d > 0 for d in diffs_solc_vs_solx),
        sum(d > 0 for d in diffs_solc_ir_vs_solx_ir)
    ],
    "Average diff (%)": [
        f"{sum(diffs_solc_vs_solx)/len(diffs_solc_vs_solx):.2f}%" if diffs_solc_vs_solx else "N/A",
        f"{sum(diffs_solc_ir_vs_solx_ir)/len(diffs_solc_ir_vs_solx_ir):.2f}%" if diffs_solc_ir_vs_solx_ir else "N/A"
    ]
}

summary_df = pd.DataFrame(summary_data)

print("### 📊 Summary\n")
print(summary_df.to_markdown(index=False))


# === 2. Top Function Improvements ===

top_no_ir = df[df["_solc_vs_solx_raw"].notnull()].sort_values("_solc_vs_solx_raw").head(15)
print("\n### 🚀 Top Improvements Per Function (evmla)\n")
print(top_no_ir[["Test", "Function", "evmla gas diff, %"]]
      .rename(columns={"evmla gas diff, %": "gas diff, %"})
      .to_markdown(index=False))

top_with_ir = df[df["_solc_ir_vs_solx_ir_raw"].notnull()].sort_values("_solc_ir_vs_solx_ir_raw").head(15)
print("\n### 🚀 Top Improvements Per Function (Yul)\n")
print(top_with_ir[["Test", "Function", "Yul gas diff, %"]]
      .rename(columns={"Yul gas diff, %": "gas diff, %"})
      .to_markdown(index=False))


# === 3. Top Contract-Level Improvements ===

contract_diffs_no_ir = {}
contract_diffs_ir = {}

for (contract, _), results in mean_table.items():
    solc, solx = results.get("solc"), results.get("solx")
    solc_ir, solx_ir = results.get("solc via-ir"), results.get("solx via-ir")

    if solc and solx and solc != 0:
        gain = (solc - solx) / solc * 100
        contract_diffs_no_ir.setdefault(contract, []).append(gain)

    if solc_ir and solx_ir and solc_ir != 0:
        gain = (solc_ir - solx_ir) / solc_ir * 100
        contract_diffs_ir.setdefault(contract, []).append(gain)

print("\n### 🧠 Contract-Level Gas Diff (evmla)\n")
print(summarize_contracts(contract_diffs_no_ir).to_markdown(index=False))

print("\n### 🧠 Contract-Level Gas Diff (Yul)\n")
print(summarize_contracts(contract_diffs_ir).to_markdown(index=False))


# === 4. All Regressed Functions ===

regressed_no_ir = df[df["_solc_vs_solx_raw"] > 0].sort_values("_solc_vs_solx_raw", ascending=False)
regressed_ir = df[df["_solc_ir_vs_solx_ir_raw"] > 0].sort_values("_solc_ir_vs_solx_ir_raw", ascending=False)

print("\n### ⚠️ All Regressed Functions (evmla)\n")
print(regressed_no_ir[["Test", "Function", "evmla gas diff, %"]]
      .rename(columns={"evmla gas diff, %": "gas diff, %"})
      .to_markdown(index=False))

print("\n### ⚠️ All Regressed Functions (Yul)\n")
print(regressed_ir[["Test", "Function", "Yul gas diff, %"]]
      .rename(columns={"Yul gas diff, %": "gas diff, %"})
      .to_markdown(index=False))
