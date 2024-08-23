import sys
from collections import deque

def process_line(line):
    # Remove everything after the first closing parenthesis
    if ')' in line:
        return line.split(')')[0] + ')'
    return line

def compare_files(file1_path, file2_path):
    try:
        # Open both files
        with open(file1_path, 'r') as file1, open(file2_path, 'r') as file2:
            # Initialize deques to store lines with a fixed length of 1 for previous line tracking
            prev_lines1 = deque(maxlen=1)
            prev_lines2 = deque(maxlen=1)
            
            # Iterate over lines from both files simultaneously with line numbers
            for line_number, (line1, line2) in enumerate(zip(file1, file2), start=1):
                # Strip newlines and process lines to remove content after `)`
                line1 = process_line(line1.rstrip())
                line2 = process_line(line2.rstrip())
                
                # Check if the lines are different
                if line1 != line2:
                    # Output the previous line if available
                    if prev_lines1 and prev_lines2:
                        print(f"Previous differing line at line number {line_number - 1}:")
                        print(f"File1: {prev_lines1[0]}")
                        print(f"File2: {prev_lines2[0]}")
                    
                    print(f"First differing line at line number {line_number}:")
                    print(f"File1: {line1}")
                    print(f"File2: {line2}")
                    return
                
                # Update previous lines
                prev_lines1.append(line1)
                prev_lines2.append(line2)

            # If no differing lines were found
            print("No differing lines found.")
    
    except FileNotFoundError as e:
        print(f"Error: {e}")
    except Exception as e:
        print(f"An error occurred: {e}")

def main():
    # Check for correct number of arguments
    if len(sys.argv) != 3:
        print("Usage: python compare_files.py <file1> <file2>")
        sys.exit(1)

    file1_path = sys.argv[1]
    file2_path = sys.argv[2]

    compare_files(file1_path, file2_path)

if __name__ == "__main__":
    main()
