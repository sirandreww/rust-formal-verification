"""
***************************************************************************************************
import
***************************************************************************************************
"""

import os
from pathlib import Path

"""
***************************************************************************************************
helper functions
***************************************************************************************************
"""

def get_all_file_paths_in_dir_recursively(rootdir: str):
    result = []
    for subdir, dirs, files in os.walk(rootdir):
        for file in files:
            file_path = os.path.join(subdir, file)
            result += [file_path]
    return result

def is_path_aig(file_path):
    return file_path[-4:] == ".aig"

def run_cmd(cmd: str):
    print(cmd)
    assert os.system(cmd) == 0


"""
***************************************************************************************************
functions
***************************************************************************************************
"""

def convert_aigs_to_aag():
    for file_path in get_all_file_paths_in_dir_recursively("./"):
        if is_path_aig(file_path=file_path):
            run_cmd(f"./aigtoaig {file_path} {file_path[:-4]}.aag")
            
            
def find_unsafe_tests():
    pass

def unfold_aigs():
    for file_path in get_all_file_paths_in_dir_recursively("./hwmcc20/"):
        if is_path_aig(file_path=file_path):
            out_file = f"{file_path[:2]}folded_{file_path[2:-4]}_folded.aig"
            out_file_parsed = out_file.split("/")
            path_to_out_file = '/'.join(out_file_parsed[:-1])
            print(path_to_out_file)
            Path(path_to_out_file).mkdir(parents=True, exist_ok=True)
            run_cmd(f'./abc -c "read {file_path}; zero ; fold ; write_aiger {out_file}"')
            # return

"""
***************************************************************************************************
call depending on need
***************************************************************************************************
"""

if __name__ == "__main__":
    unfold_aigs()
