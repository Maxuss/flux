import json

if __name__ == "__main__":
    with open("items.json", "r") as f:
        text = f.read()
        items = json.loads(text)
        with open("generated/material.g.rs", "w+") as output:
            output.write("""
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Material {\n""")
            for item_name in items:
                actual_name = item_name.replace("minecraft:", "").replace(
                    "_", " ").title().replace(" ", "")
                output.write(f"  {actual_name},\n")
            output.write("}")
