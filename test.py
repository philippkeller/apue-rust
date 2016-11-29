#!/usr/bin/env python3

import subprocess, os

# 'Darwin' or 'Linux'
uname = os.uname().sysname

def check_same(file_path, cmd, from_cmd, comments):
    from_comments = "\n".join(comments)
    if from_cmd != from_comments:
        print("error in", file_path, "executing", cmd)
        print("expected >>{}<<, len:{}".format(from_comments, len(from_comments)))
        print("got: >>{}<<, len:{}".format(from_cmd, len(from_cmd)))


def run(cmd):
    try:
        a = subprocess.check_output(cmd, shell=True,
                                    universal_newlines=True)
    except subprocess.CalledProcessError as e:
        a = e.output + "ERROR: return code {}".format(e.returncode)

    return a.strip()

class CommentStateMachine:
    def __init__(self, file_path):
        self.last_command = None
        self.last_command_output = None
        self.comments_below_command = []
        self.file_path = file_path
        self.osrestriction = None

    # seen a line without comment -> end of comment block
    def no_comment(self):
        if len(self.comments_below_command) == 0:
            return
        if self.osrestriction and self.osrestriction != uname:
            return
        if self.last_command_output != None:
            check_same(self.file_path, self.last_command,
                       self.last_command_output, self.comments_below_command)
            self.last_command_output = None
        self.comments_below_command = []

    def comment(self, line):
        self.comments_below_command.append(line)

    def line_with_command(self, line):
        self.no_comment()
        if self.osrestriction and self.osrestriction != uname:
            return

        self.last_command_output = run(line)
        self.last_command = line

if __name__ == "__main__":
    # add target/debug to path
    cur_path = os.path.dirname(os.path.realpath(__file__))
    if os.environ.get('CARGO_TARGET_DIR'):
        d = os.environ.get('CARGO_TARGET_DIR')
        build_dir = os.path.join(cur_path, d, 'debug')
    else:
        build_dir = os.path.join(cur_path, 'target', 'debug')
    os.environ["PATH"] += os.pathsep + build_dir

    src_dir = os.path.join(cur_path, 'src', 'bin')

    for root, dirs, files in os.walk(src_dir):
        for f in [i for i in files if i.endswith('.rs')]:
            m = CommentStateMachine(os.path.join(root, f))
            for line in open(os.path.join(root, f)):
                if line.startswith('///'):
                    line = line[3:].strip()
                    if line.lower() == 'linux only:':
                        m.osrestriction = 'Linux'
                    elif line.lower() == 'mac only:':
                        m.osrestriction = 'Darwin'
                    elif line.startswith('$'):
                        m.line_with_command(line[1:].strip())
                    elif len(line) == 0:
                        m.no_comment()
                    else:
                        m.comment(line)
                elif len(line.strip()) == 0:
                    m.no_comment()
                else:
                    # stop when we see the first non-comment line, e.g. `extern crate`
                    m.no_comment()
                    break