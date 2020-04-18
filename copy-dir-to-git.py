"""
これは　わたし用のプログラムだぜ☆つ（＾～＾）！
"""
import os
import shutil

source = 'C:/Users/むずでょ/source/repos/rust-kifuwarabe-wcsc30'
destination = 'C:/Users/むずでょ/Documents/GitHub/rust-kifuwarabe-wcsc30'


def go():
    print('Trace   | Remove.')
    shutil.rmtree(f'{destination}/src')
    os.remove(f'{destination}/.gitignore')
    os.remove(f'{destination}/Cargo.toml')
    os.remove(f'{destination}/copy-dir-to-git.py')
    os.remove(f'{destination}/LICENSE')
    os.remove(f'{destination}/README.md')

    print('Trace   | Copy.')
    copy_dir('/src', ignore=shutil.ignore_patterns('*.pdb'))
    copy_file('/.gitignore')
    copy_file('/Cargo.toml')
    copy_file('/copy-dir-to-git.py')
    copy_file('/LICENSE')
    copy_file('/README.md')
    print('Trace   | Finished.')


def copy_dir(child_path, ignore=None):
    shutil.copytree(f'{source}{child_path}',
                    f'{destination}{child_path}', ignore=ignore)


def copy_file(child_path):
    shutil.copy2(f'{source}{child_path}', f'{destination}{child_path}')


go()
