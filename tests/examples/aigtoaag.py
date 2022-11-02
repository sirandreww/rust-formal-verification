


def main():
    import os
    rootdir = './'
    for subdir, dirs, files in os.walk(rootdir):
        for file in files:
            file_path = os.path.join(subdir, file)
            if file_path[-4:] == ".aig":
                print(file_path)
                os.system(f"./aigtoaig {file_path} {file_path[:-4]}.aag")
            


if __name__ == "__main__":
    main()
