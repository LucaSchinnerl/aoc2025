import subprocess
import re
import os

def run_benchmarks():
    # Run the cargo command to get benchmarks
    result = subprocess.run(
        ["cargo", "run", "--release"], 
        capture_output=True, 
        text=True,
        check=True
    )
    return result.stdout

def parse_benchmarks(output):
    # Regex to match Day and Time
    # Output format from main.rs:
    # Day 01:
    # ...
    # Time: 123μs
    
    days = []
    
    # Split by "Day " to handle multiple days
    parts = output.split("Day ")
    
    total_time_str = "0ms"
    
    for part in parts:
        if not part.strip():
            continue
            
        # parsing "01:\n..."
        lines = part.splitlines()
        if not lines:
            continue
            
        day_num = lines[0].strip(":")
        
        # Find time line
        time_str = "N/A"
        for line in lines:
            if line.strip().startswith("Time:"):
                time_str = line.strip().split("Time:")[1].strip()
                break
        
        if "Total time:" in part:
             # This might be at the end of the output, handled separately?
             pass
             
        if day_num.isdigit():
             days.append((day_num, time_str))

    # Extract total time from the very end
    # "Total time: 45ms"
    total_match = re.search(r"Total time: (.+)", output)
    if total_match:
        total_time_str = total_match.group(1)

    return days, total_time_str

def generate_table(days, total_time):
    lines = ["| Day | Time |", "| --- | --- |"]
    
    for day, time in days:
        lines.append(f"| {day} | {time} |")
        
    lines.append(f"| **Total** | **{total_time}** |")
    
    return "\n".join(lines)

def update_readme(table):
    readme_path = "README.md"
    with open(readme_path, "r") as f:
        content = f.read()
        
    start_marker = "<!-- benchmarks-start -->"
    end_marker = "<!-- benchmarks-end -->"
    
    pattern = re.compile(
        f"{re.escape(start_marker)}.*{re.escape(end_marker)}", 
        re.DOTALL
    )
    
    replacement = f"{start_marker}\n{table}\n{end_marker}"
    
    new_content = pattern.sub(replacement, content)
    
    with open(readme_path, "w") as f:
        f.write(new_content)

if __name__ == "__main__":
    print("Running benchmarks...")
    output = run_benchmarks()
    print("Parsing results...")
    days, total = parse_benchmarks(output)
    print(f"Found {len(days)} days. Total time: {total}")
    table = generate_table(days, total)
    print("Updating README...")
    update_readme(table)
    print("Done.")

