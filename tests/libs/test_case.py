import io
from contextlib import redirect_stdout, redirect_stderr
from typing import Callable

from libs.gitlab import GitLab
from libs.janet import Janet

test_cases = []


class TestCase:
    def __init__(self, name: str, func: Callable[[GitLab, Janet], None]):
        self.name = name
        self.func = func

    def run(self, janet_exec: str, janet_cwd: str) -> bool:
        print(f"[+] {self.name}")

        test_stdout = io.StringIO()
        test_stderr = io.StringIO()
        janet_stdout = ""
        janet_stderr = ""

        try:
            try:
                with redirect_stdout(test_stdout), redirect_stderr(test_stderr):
                    gitlab = GitLab()
                    janet = Janet(janet_exec, janet_cwd)

                    self.func(gitlab, janet)

                    gitlab.expect.ensure_handled()
            finally:
                if 'gitlab' in locals():
                    gitlab.kill()

                if 'janet' in locals():
                    janet.kill()
                    janet_stdout = janet.process.stdout.read().decode()
                    janet_stderr = janet.process.stderr.read().decode()
        except Exception as err:
            print(" -  [ FAILED ]")
            print()

            print("========== Error ==========")
            print(err)
            print()

            print("========== Test's stdout ==========")
            print(test_stdout.getvalue())
            print()

            print("========== Test's stderr ==========")
            print(test_stderr.getvalue())
            print()

            print("========== Janet's stdout ==========")
            print(janet_stdout)
            print()

            print("========== Janet's stderr ==========")
            print(janet_stderr)
            print()

            return False

        print(" -  [ OK ]")
        print()

        return True

    @staticmethod
    def all():
        return test_cases


def test_case(name):
    def decorator(func):
        test_cases.append(TestCase(name, func))

        return func

    return decorator
