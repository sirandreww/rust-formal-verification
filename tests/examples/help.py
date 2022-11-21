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

def is_path_aig_and_not_folded(file_path):
    return is_path_aig(file_path) and ("_folded" not in file_path)

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

def zero_then_fold_aigs():
    for file_path in get_all_file_paths_in_dir_recursively("./hwmcc20/"):
        if is_path_aig_and_not_folded(file_path=file_path):
            assert(file_path[-4:] == ".aig")
            out_file = f"{file_path[:-4]}_zeroed_then_folded.aig"
            print(out_file)
            run_cmd(f'./abc -c "read {file_path}; zero ; fold2 ; write_aiger {out_file}"')
    # make aag files for these new aig files
    convert_aigs_to_aag()

"""
***************************************************************************************************
call depending on need
***************************************************************************************************
"""

if __name__ == "__main__":
    zero_then_fold_aigs()
