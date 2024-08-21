import argparse

parser = argparse.ArgumentParser(
    prog='Actix-Eval GraphBuilder',
    description='A script to automatically generate graphs out k6 data')
parser.add_argument('filename')

args = parser.parse_args()
print(args.filename)