"""
これは　わたし用のプログラムだぜ☆つ（＾～＾）！
"""
import os
import shutil

source = 'C:/Users/むずでょ/source/repos/rust-kifuwarabe-wcsc30'
destination = 'C:/Users/むずでょ/Documents/GitHub/rust-kifuwarabe-wcsc30'


def go():
    print('Trace   | Remove.')
    remove_destination_dir('/src')
    remove_destination_dir('/kifu')
    remove_destination_file('/.gitignore')
    remove_destination_file('/Cargo.toml')
    remove_destination_file('/copy-to-git.py')
    remove_destination_file('/LICENSE')
    remove_destination_file('/README.md')

    print('Trace   | Copy.')
    copy_dir('/src', ignore=shutil.ignore_patterns('*.pdb'))
    copy_dir('/kifu')
    copy_file('/.gitignore')
    copy_file('/Cargo.toml')
    copy_file('/copy-to-git.py')
    copy_file('/LICENSE')
    copy_file('/README.md')
    print('Trace   | Finished.')


def remove_destination_dir(child_path: str):
    path = f'{destination}{child_path}'
    if os.path.isdir(path):
        shutil.rmtree(path)


def remove_destination_file(child_path: str):
    path = f'{destination}{child_path}'
    if os.path.isfile(path):
        os.remove(path)


def copy_dir(child_path: str, ignore=None):
    shutil.copytree(f'{source}{child_path}',
                    f'{destination}{child_path}', ignore=ignore)


def copy_file(child_path: str):
    shutil.copy2(f'{source}{child_path}', f'{destination}{child_path}')


go()
