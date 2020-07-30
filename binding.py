import sys
import subprocess
import json


def solve(exec_path, input_content, max_size=10, output=True):
    """
    `exec_path` - 可执行文件路径；
    `input_content` - 输入 JSON；
    `max_size` - 求解 AFA 尺寸限制；
    `output` - 是否输出程序运行结果。

    若有解，则返回 `(字符串, 元组)`，否则返回 `None`
    """
    result = subprocess.run(
        [exec_path, '-', '-', '-n', str(max_size), '-b'], input=json.dumps(input_content), encoding='utf-8',
        stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    if output:
        print(result.stderr)

    if result.returncode == 0:
        [s, t] = result.stdout.strip().split('\n')
        return (s, eval(t))
    else:
        return None


if __name__ == "__main__":
    print('求解结果', solve('target/release/limeltl',
                        {"vocab": ["p", "q", "r"], "traces_pos": [[["p"], ["p"], ["q"]], [["p"], ["q"]], [["p", "r"], ["q"]], [["q", "r"]]], "traces_neg": [[["p"], ["r"], ["q"]], [["p"], ["r"]], [["r"], ["q"]]]}, 3, False))
